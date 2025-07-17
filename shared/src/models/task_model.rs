use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub status: String,
    pub due_date: Option<String>,
    pub schedule: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTaskRequest {
    pub name: String,
    pub description: String,
    pub status: Option<String>,
    pub due_date: Option<String>,
    pub schedule: Option<String>
}