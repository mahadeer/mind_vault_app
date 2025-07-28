mod formatters;
mod tools;
mod utils;

use crate::tools::task_tool_handler::TaskToolServer;
use mindvault_shared::logger::init_logger;
use rmcp::transport::stdio;
use rmcp::ServiceExt;
use tracing::{error, info};
use crate::utils::db_utils::get_task_repository;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let log_guard = init_logger();
    info!("🚀 Starting MindVault Tasks MCP Server...");
    let task_repository = get_task_repository().await;
    let tool_server = TaskToolServer::new(task_repository)
        .await
        .serve(stdio())
        .await
        .map_err(|e| {
            error!("Error starting tool: {}", e);
        })
        .unwrap();
    tool_server.waiting().await?;
    info!("🚀 Stopping MindVault Tasks MCP Server...");
    drop(log_guard);
    Ok(())
}
