use once_cell::sync::Lazy;
use surrealdb::engine::local::SurrealKV;
use surrealdb::Surreal;

//static DB: Lazy<Surreal<SurrealKV>> = Lazy::new(Surreal::init);

pub async fn create_database() -> surrealdb::Result<()> {
    //DB.connect::<SurrealKV>("./").await?;
    //DB.use_ns("lucle").use_db("lucle").await?;
    let db = Surreal::new::<SurrealKV>("./lucle").await?;
    tracing::info!("Creating database lucle");
    Ok(())
}
