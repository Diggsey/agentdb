use foundationdb::{Database, FdbError, TransactOption};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _network = unsafe { foundationdb::boot() };

    let db = Database::default()?;

    Ok(())
}
