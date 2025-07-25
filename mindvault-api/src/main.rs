mod router;
mod services;
mod models;
mod response_macro;

use mindvault_core::db::bootstrap_db;
use mindvault_core::models::AppDatabase;
use mindvault_shared::logger::init_logger;
use std::error::Error;
use tokio::net::TcpListener;
use tracing::info;
use crate::router::MindVaultRouter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let log_guard = init_logger();
    info!("--- Starting MindVault Service ---");
    let db_client = bootstrap_db().await.expect("Failed to connect to db");
    bootstrap_server(db_client).await?;
    info!("--- MindVault Service Stopped ---");
    drop(log_guard);
    Ok(())
}

async fn bootstrap_server(db_client: AppDatabase) -> Result<(), Box<dyn Error>> {
    let app_router = MindVaultRouter::new(db_client);
    let mind_vault_router = app_router.get_router();
    let listener = TcpListener::bind("127.0.0.1:4500").await?;
    info!("Listening on {}", &listener.local_addr()?);
    axum::serve(listener, mind_vault_router).await?;
    Ok(())
}
