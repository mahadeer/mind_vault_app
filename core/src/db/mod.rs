mod config;

use mongodb::{Client};
use tracing::info;
use crate::db::config::MongoConfig;
use crate::models::DbError;

pub async fn bootstrap_db() -> Result<Client, DbError> {
    info!("Attempting to connect to the database...");
    let mongo_config = MongoConfig::from_env().unwrap();
    let conn = Client::with_uri_str(mongo_config.uri).await?;
    info!("Connected to the database");
    info!("Database schema initialized successfully.");
    Ok(conn)
}