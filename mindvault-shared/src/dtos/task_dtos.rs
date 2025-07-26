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