
use rmcp::ServiceExt;
use rmcp::transport::stdio;
use crate::tools::task_tools::TaskTool;

mod tools;
mod utils;
mod models;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let service =  TaskTool::new().serve(stdio()).await.inspect_err(|e| {
        println!("Error starting tool: {}", e);
    }).unwrap();
    service.waiting().await?;
    Ok(())
}
