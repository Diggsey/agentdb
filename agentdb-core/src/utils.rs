use std::{
    fmt::{self, Display},
    ops::{Add, AddAssign, Sub, SubAssign},
    time::Duration,
};

use chrono::{DateTime, TimeZone, Utc};
use foundationdb::{future::FdbValue, FdbError, RangeOption, Transaction};
use futures::TryStreamExt;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use uuid::Uuid;

use crate::{client::PartitionRange, subspace::Subspace, DEFAULT_PARTITION_RANGE};

pub fn partition_for_recipient(recipient_id: Uuid, partition_range: PartitionRange) -> u32 {
    let hash = recipient_id.as_u128();
    let hash = ((hash >> 64) ^ hash) as u64;
    let hash = ((hash >> 32) ^ hash) as u32;
    (hash % partition_range.count) + partition_range.offset
}

pub async fn get_first_in_range(
    tx: &Transaction,
    mut range: RangeOption<'_>,
    snapshot: bool,
) -> Result<Option<FdbValue>, FdbError> {
    range.limit = Some(1);
    let mut stream = tx.get_ranges(range, snapshot);
    while let Some(values) = stream.try_next().await? {
        for value in values {
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
    root: &[u8],
    space: &Subspace<()>,
    snapshot: bool,
) -> Result<PartitionRange, FdbError> {
    let key = space.key(root, ());
    Ok(load_value(tx, &key, snapshot)
        .await?
        .unwrap_or(DEFAULT_PARTITION_RANGE))
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Timestamp(DateTime<Utc>);

impl Timestamp {
    pub fn zero() -> Self {
        Self::from_millis(0)
    }
    pub fn now() -> Self {
        Self(Utc::now())
    }
    pub fn millis(self) -> i64 {
        self.0.timestamp_millis()
    }
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
        (self.0 - rhs.0).to_std().unwrap_or(Duration::from_secs(0))
    }
}
