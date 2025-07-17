mod services;
mod routes;

use tokio::net::TcpListener;
use tracing::{info};
use shared::logger::init_logger;
use core::db::bootstrap_db;
use crate::services::ApiService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let log_guard = init_logger();

    info!("--- Starting MindVault Service ---");
    let db_client = bootstrap_db().await.expect("Failed to connect to db");
    
    let api_service = ApiService::new(db_client);
    let service_handlers = api_service.get_service_handlers();
    
    let listener = TcpListener::bind("0.0.0.0:4500").await?;
    info!("Listening on {}", listener.local_addr()?);
    axum::serve(listener, service_handlers).await?;
    
    info!("--- MindVault Service Stopped ---");
    drop(log_guard);
    Ok(())
}

