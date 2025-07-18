use rmcp::schemars;
use rmcp::schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, JsonSchema)]
pub struct GetTaskById {
    pub task_id: String,
}

#[derive(Deserialize, JsonSchema, Serialize)]
pub struct CreateUserTask {
    pub name: String,
    pub description: String,
    #[serde(default = "default_status")]
    pub status: Option<String>,
    pub due_date: Option<String>,
    pub schedule: Option<String>,
}

fn default_status() -> Option<String> {
    Some("pending".to_string())
}

#[derive(Deserialize, JsonSchema, Serialize)]
pub struct UpdateUserTask {
    pub task_id: String,
    pub user_task: CreateUserTask,
}