use rmcp::model::{CallToolResult, Content};
use mindvault_shared::models::tasks_model::{ETaskStatus, ETaskPriority};

pub trait MCPOutputFormatter {
    fn format_status(status: &ETaskStatus) -> String;
    fn format_priority(priority: &ETaskPriority) -> String;
    fn format_task_list(self) -> Vec<Content>;
}

pub trait MCPUtils: MCPOutputFormatter {
    fn into_call_tool_result(self) -> CallToolResult;
}

pub trait EnumFormatter<T> {
    fn as_type(&self) -> Option<T>;
}