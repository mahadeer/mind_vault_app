use crate::utils::{as_content_list_string, as_content_string};
use reqwest::Client;
use rmcp::ErrorData as RmcpError;
use rmcp::handler::server::tool::{Parameters, ToolRouter};
use rmcp::model::{CallToolResult, Implementation, ProtocolVersion, ServerCapabilities, ServerInfo};
use rmcp::{ServerHandler, tool, tool_handler, tool_router};
use shared::models::task_model::Task;
use crate::models::GetTaskById;

#[derive(Debug)]
pub struct TaskTool {
    base_url: String,
    pub tool_router: ToolRouter<Self>,
}

#[tool_router]
impl TaskTool {
    pub(crate) fn new() -> TaskTool {
        TaskTool {
            base_url: "http://localhost:4500".to_string(),
            tool_router: Self::tool_router(),
        }
    }

    #[tool(
        name = "Get Tasks",
        description = "Get all available tasks for the user"
    )]
    pub async fn get_user_tasks(&self) -> Result<CallToolResult, RmcpError> {
        let client = Client::new(); // Create an HTTP client instance
        let get_all_tasks_url = format!("{}/tasks", self.base_url);
        let response = client.get(get_all_tasks_url).send().await;
        let tasks: Vec<Task> = response.unwrap().json().await.unwrap();
        if tasks.is_empty() {
            let no_tasks = as_content_list_string(vec!["Task not found".to_string()]);
            Ok(CallToolResult::success(no_tasks))
        } else {
            Ok(CallToolResult::success(as_content_string(tasks)))
        }
    }

    #[tool(
        name = "Get Task by id",
        description = "Get a task by its id"
    )]
    pub async fn get_task_by_id(&self, Parameters(GetTaskById{task_id}): Parameters<GetTaskById>) -> Result<CallToolResult, RmcpError> {
        let client = Client::new(); // Create an HTTP client instance
        let get_task_by_id_url = format!("{}/tasks/{}", self.base_url, task_id);
        let response = client.get(get_task_by_id_url).send().await;
        let task: Task = response.unwrap().json().await.unwrap();
        let selected_task = as_content_string(vec![task]);
        Ok(CallToolResult::success(selected_task))
    }
}

#[tool_handler]
impl ServerHandler for TaskTool {
    fn get_info(&self) -> ServerInfo {
        let tool_instruction = "This tool can provide a todo list for the user, this tool can update, delete, list and search task by id or name.";
        ServerInfo {
            protocol_version: ProtocolVersion::V_2025_03_26,
            instructions: Some(tool_instruction.to_string()),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation::from_build_env(),
        }
    }
}
