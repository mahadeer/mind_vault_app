use tracing::{info};
use shared::logger::init_logger;
use core::db::bootstrap_db;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let log_guard = init_logger();

    info!("--- Starting MindVault Service ---");
    let db_client = bootstrap_db().await.expect("Failed to connect to db");
    info!("--- MindVault Service Stopped ---");
    
    drop(log_guard);
    Ok(())
}

