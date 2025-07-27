use crate::models::tasks_model::{ETaskPriority, ETaskStatus};
use chrono::NaiveDate;
use serde::Deserialize;
use crate::utils::date_time_serde::deserialize_multiple_formats;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TaskSearchParams {
    pub query: Option<String>,
    pub status: Option<ETaskStatus>,
    pub priority: Option<ETaskPriority>,
    #[serde(default, deserialize_with = "deserialize_multiple_formats")]
    pub due_date: Option<NaiveDate>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTaskRequest {
    pub name: String,
    #[serde(default, deserialize_with = "deserialize_multiple_formats")]
    pub due_date: Option<NaiveDate>,
    pub priority: Option<ETaskPriority>,
    pub status: Option<ETaskStatus>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BulkCreateTaskRequest {
    pub tasks: Vec<CreateTaskRequest>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTaskRequest {
    #[serde(default, deserialize_with = "deserialize_multiple_formats")]
    pub due_date: Option<NaiveDate>,
    pub priority: Option<ETaskPriority>,
    pub status: Option<ETaskStatus>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchAndUpdateRequest {
    // Search parameters
    pub query: Option<String>,
    pub status_filter: Option<ETaskStatus>,
    pub priority_filter: Option<ETaskPriority>,
    #[serde(default, deserialize_with = "deserialize_multiple_formats")]
    pub due_date_filter: Option<NaiveDate>,

    // Update fields
    #[serde(default, deserialize_with = "deserialize_multiple_formats")]
    pub due_date: Option<NaiveDate>,
    pub priority: Option<ETaskPriority>,
    pub status: Option<ETaskStatus>,
}