use rmcp::schemars;
use rmcp::schemars::JsonSchema;
use serde::{Deserialize};

#[derive(Deserialize, JsonSchema)]
pub struct GetTaskById {
    pub task_id: String,
}