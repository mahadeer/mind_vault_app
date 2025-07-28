use mongodb::{Client, Database};
use tracing::info;
use crate::mongo_config::MongoConfig;
use crate::models::DbError;

pub async fn bootstrap_db() -> Result<Database, DbError> {
    info!("Attempting to connect to the database...");
    let mongo_config = MongoConfig::from_env().unwrap();
    let conn = Client::with_uri_str(mongo_config.uri).await?;
    info!("Connected to the database");
    info!("Database schema initialized successfully.");
    Ok(conn.database(mongo_config.db_name.as_str()))
}

pub async fn get_app_database() -> Result<Database, DbError> {
    bootstrap_db().await
}