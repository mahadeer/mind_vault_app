use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum ETaskPriority {
    Normal,
    High,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum ETaskStatus {
    NotStarted,
    Pending,
    InProgress,
    Completed,
}

fn default_utc_now() -> DateTime<Utc> {
    Utc::now()
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    #[serde(rename = "_id")]
    pub id: i64,
    pub name: String,
    pub priority: ETaskPriority,
    pub status: ETaskStatus,
    #[serde(default)]
    pub due_date: Option<NaiveDateTime>,
    #[serde(default = "default_utc_now")]
    pub created_at: DateTime<Utc>,
}