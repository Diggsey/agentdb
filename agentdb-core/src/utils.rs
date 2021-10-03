use foundationdb::{future::FdbValue, FdbError, RangeOption, Transaction};
use futures::TryStreamExt;

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
