use crate::utils::date_time_serde::{
    serialize_bson_datetime_as_chrono_date, serialize_option_bson_datetime_as_chrono_date,
};
use bson::DateTime as BsonDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ETaskPriority {
    Normal,
    High,
}

impl Default for ETaskPriority {
    fn default() -> Self {
        ETaskPriority::Normal
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ETaskStatus {
    NotStarted,
    Pending,
    InProgress,
    Completed,
}

impl Default for ETaskStatus {
    fn default() -> Self {
        ETaskStatus::NotStarted
    }
}

fn default_utc_now() -> BsonDateTime {
    BsonDateTime::now()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    #[serde(rename = "_id")]
    pub id: i64,
    pub name: String,
    #[serde(default)]
    pub priority: ETaskPriority,
    #[serde(default)]
    pub status: ETaskStatus,
    pub due_date: Option<BsonDateTime>,
    #[serde(default = "default_utc_now")]
    pub created_at: BsonDateTime,
    #[serde(default)]
    pub deleted: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskResponse {
    pub id: i64,
    pub name: String,
    pub priority: ETaskPriority,
    pub status: ETaskStatus,
    #[serde(
        serialize_with = "serialize_option_bson_datetime_as_chrono_date",
        skip_serializing_if = "Option::is_none"
    )]
    pub due_date: Option<BsonDateTime>,
    #[serde(
        serialize_with = "serialize_bson_datetime_as_chrono_date"
    )]
    pub created_at: BsonDateTime,
}

impl From<Task> for TaskResponse {
    fn from(task: Task) -> Self {
        Self {
            id: task.id,
            name: task.name,
            priority: task.priority,
            status: task.status,
            due_date: task.due_date,
            created_at: task.created_at,
        }
    }
}

impl TaskResponse {
    pub fn from_vec(tasks: Vec<Task>) -> Vec<Self> {
        tasks.into_iter().map(TaskResponse::from).collect()
    }
}
