use mindvault_shared::models::tasks_model::{ETaskPriority, ETaskStatus};
use crate::utils::mcp_utils::EnumFormatter;

impl EnumFormatter<ETaskStatus> for Option<String> {
    fn as_type(&self) -> Option<ETaskStatus> {
        if let Some(status_str) = self {
            match status_str.to_lowercase().as_str() {
                "notstarted" | "not started" => Some(ETaskStatus::NotStarted),
                "pending" => Some(ETaskStatus::Pending),
                "inprogress" => Some(ETaskStatus::InProgress),
                "completed" | "complete" | "done" => Some(ETaskStatus::Completed),
                _ => None,
            }
        } else {
            None
        }
    }
}

impl EnumFormatter<ETaskPriority> for Option<String> {
    fn as_type(&self) -> Option<ETaskPriority> {
        if let Some(priority_str) = self {
            match priority_str.as_str() {
                "normal" | "low" => Some(ETaskPriority::Normal),
                "high" | "urgent" => Some(ETaskPriority::High),
                _ => None,
            }
        } else {
            None
        }
    }
}