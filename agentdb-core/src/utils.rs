use std::{
    fmt::{self, Display},
    ops::{Add, AddAssign, Sub, SubAssign},
    time::Duration,
};

use chrono::{DateTime, TimeZone, Utc};
use foundationdb::{
    future::FdbValue,
    tuple::{TuplePack, TupleUnpack},
    Database, FdbError, RangeOption, TransactOption, Transaction,
};
use futures::{future::BoxFuture, Future, FutureExt, TryStreamExt};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use uuid::Uuid;

use crate::{client::PartitionRange, Error, DEFAULT_PARTITION_RANGE};

pub fn partition_for_recipient(recipient_id: Uuid, partition_range: PartitionRange) -> u32 {
    let hash = recipient_id.as_u128();
    let hash = ((hash >> 64) ^ hash) as u64;
    let hash = ((hash >> 32) ^ hash) as u32;
    (hash % partition_range.count) + partition_range.offset
}

pub fn next_key(key: &[u8]) -> Vec<u8> {
    let mut v = Vec::with_capacity(key.len() + 1);
    v.extend_from_slice(key);
    v.push(0);
    v
}

pub fn move_entries<'trx, D: 'trx + Send + Sync>(
    db: &'trx Database,
    data: D,
    range: RangeOption<'trx>,
    conv: impl for<'a> FnMut(&'a FdbValue, &'a D) -> BoxFuture<'a, Result<Vec<u8>, Error>> + Send + 'trx,
) -> impl Future<Output = Result<bool, Error>> + Send + 'trx {
    db.transact_boxed(
        (range, conv, data),
        |tx, (range, conv, data)| {
            async move {
                let mut msg_stream = tx.get_ranges(range.clone(), false);
                let mut more = false;
                while let Some(batch) = msg_stream.try_next().await? {
                    more = batch.more();
                    for item in batch {
                        tx.clear(item.key());
                        let new_key = conv(&item, data).await?;
                        tx.set(&new_key, item.value());
                    }
                }

                Ok::<_, Error>(more)
            }
            .boxed()
        },
        TransactOption::idempotent(),
    )
}
pub async fn get_first_in_range(
    tx: &Transaction,
    mut range: RangeOption<'_>,
    snapshot: bool,
) -> Result<Option<FdbValue>, FdbError> {
    range.limit = Some(1);
    let mut stream = tx.get_ranges(range, snapshot);
    while let Some(values) = stream.try_next().await? {
        if let Some(value) = values.into_iter().next() {
            return Ok(Some(value));
        }
    }
    Ok(None)
}

pub async fn range_is_empty(
    tx: &Transaction,
    range: RangeOption<'_>,
    snapshot: bool,
) -> Result<bool, FdbError> {
    Ok(get_first_in_range(tx, range, snapshot).await?.is_none())
}

pub async fn load_value<T: DeserializeOwned>(
    tx: &Transaction,
    key: &[u8],
    snapshot: bool,
) -> Result<Option<T>, FdbError> {
    Ok(tx
        .get(key, snapshot)
        .await?
        .as_deref()
        .and_then(|slice| postcard::from_bytes(slice).ok()))
}

pub fn save_value<T: Serialize>(tx: &Transaction, key: &[u8], value: &T) {
    tx.set(
        key,
        &postcard::to_stdvec(value).expect("Infallible serialization"),
    );
}

pub async fn load_partition_range(
    tx: &Transaction,
    key: &[u8],
    snapshot: bool,
) -> Result<PartitionRange, FdbError> {
    Ok(load_value(tx, key, snapshot)
        .await?
        .unwrap_or(DEFAULT_PARTITION_RANGE))
}

/// Timestamp type used by AgentDB. Represents a real time.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Timestamp(DateTime<Utc>);

impl Timestamp {
    /// Zero timestamp is used to indicate that a message should be sent immediately.
    pub fn zero() -> Self {
        Self::from_millis(0)
    }
    /// Current timestamp.
    pub fn now() -> Self {
        Self(Utc::now())
    }
    /// Convert the timestamp to milliseconds since an arbitrary epoch.
    pub fn millis(self) -> i64 {
        self.0.timestamp_millis()
    }
    /// Convert from milliseconds since an arbitrary epoch to a timestamp.
    pub fn from_millis(ms: i64) -> Self {
        Self(Utc.timestamp_millis(ms))
    }
}

impl Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<'de> Deserialize<'de> for Timestamp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let (nanos, secs) = <(u32, i64)>::deserialize(deserializer)?;
        Ok(Self(DateTime::from_utc(
            chrono::NaiveDateTime::from_timestamp(secs, nanos),
            Utc,
        )))
    }
}

impl Serialize for Timestamp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (self.0.timestamp_subsec_nanos(), self.0.timestamp()).serialize(serializer)
    }
}

impl From<DateTime<Utc>> for Timestamp {
    fn from(other: DateTime<Utc>) -> Self {
        Self(other)
    }
}

impl From<Timestamp> for DateTime<Utc> {
    fn from(other: Timestamp) -> Self {
        other.0
    }
}

impl Add<Duration> for Timestamp {
    type Output = Self;

    fn add(self, rhs: Duration) -> Self::Output {
        Self(self.0 + chrono::Duration::from_std(rhs).expect("Duration in range"))
    }
}

impl Sub<Duration> for Timestamp {
    type Output = Self;

    fn sub(self, rhs: Duration) -> Self::Output {
        Self(self.0 - chrono::Duration::from_std(rhs).expect("Duration in range"))
    }
}

impl AddAssign<Duration> for Timestamp {
    fn add_assign(&mut self, rhs: Duration) {
        *self = *self + rhs;
    }
}

impl SubAssign<Duration> for Timestamp {
    fn sub_assign(&mut self, rhs: Duration) {
        *self = *self - rhs;
    }
}

impl Sub<Timestamp> for Timestamp {
    type Output = Duration;

    fn sub(self, rhs: Timestamp) -> Self::Output {
        (self.0 - rhs.0)
            .to_std()
            .unwrap_or_else(|_| Duration::from_secs(0))
    }
}

impl TuplePack for Timestamp {
    fn pack<W: std::io::Write>(
        &self,
        w: &mut W,
        tuple_depth: foundationdb::tuple::TupleDepth,
    ) -> std::io::Result<foundationdb::tuple::VersionstampOffset> {
        self.millis().pack(w, tuple_depth)
    }
}

impl<'de> TupleUnpack<'de> for Timestamp {
    fn unpack(
        input: &'de [u8],
        tuple_depth: foundationdb::tuple::TupleDepth,
    ) -> foundationdb::tuple::PackResult<(&'de [u8], Self)> {
        i64::unpack(input, tuple_depth).map(|(rest, v)| (rest, Self::from_millis(v)))
    }
}
