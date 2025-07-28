use rmcp::model::{CallToolResult, Content};
use mindvault_shared::models::tasks_model::{ETaskPriority, ETaskStatus, TaskResponse};
use crate::utils::mcp_utils::{MCPOutputFormatter, MCPUtils};

impl MCPOutputFormatter for Vec<TaskResponse> {
    fn format_status(status: &ETaskStatus) -> String {
        match status {
            ETaskStatus::NotStarted => "🔴 Not Started".to_string(),
            ETaskStatus::Pending => "🟡 Pending".to_string(),
            ETaskStatus::InProgress => "🔵 In Progress".to_string(),
            ETaskStatus::Completed => "✅ Completed".to_string(),
        }
    }

    fn format_priority(priority: &ETaskPriority) -> String {
        match priority {
            ETaskPriority::Normal => "📝 Normal".to_string(),
            ETaskPriority::High => "🔥 High".to_string(),
        }
    }

    fn format_task_list(self) -> Vec<Content> {
        if self.is_empty() {
            vec![Content::text("📭 No tasks found".to_string())]
        } else {
            let mut content = vec![Content::text(format!("📋 Found {} task(s):\n", self.len()))];

            for task in &self {
                let due_date_str = match &task.due_date {
                    Some(date) => format!(" | 📅 {}", date.to_chrono().format("%d/%m/%y")),
                    None => "".to_string(),
                };

                content.push(Content::text(format!(
                    "{} | {} | {}{} (#{})",
                    task.name,
                    Self::format_status(&task.status),
                    Self::format_priority(&task.priority),
                    due_date_str,
                    task.id,
                )));
            }
            content
        }
    }
}

impl MCPUtils for Vec<TaskResponse> {
    fn into_call_tool_result(self) -> CallToolResult {
        CallToolResult::success(self.format_task_list())
    }
}
