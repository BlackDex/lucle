use surrealdb::engine::local::SurrealKV;
use surrealdb::Surreal;

pub async fn create_database() -> surrealdb::Result<()> {
    let db = Surreal::new::<SurrealKV>("./").await?;
    db.use_ns("lucle").use_db("lucle").await?;
    tracing::info!("Creating database lucle");
    Ok(())
}
