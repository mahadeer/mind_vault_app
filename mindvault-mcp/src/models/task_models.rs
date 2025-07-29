use rmcp::schemars;
use rmcp::schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetTaskByIdParams {
    pub task_id: i64,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateTaskParams {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateTaskByIdParams {
    pub task_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct BulkCreateTaskParams {
    pub names: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statuses: Vec<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priorities: Vec<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_dates: Vec<Option<String>>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SearchTasksParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SearchAndUpdateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_filter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority_filter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date_filter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DeleteTasksByStatusParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
