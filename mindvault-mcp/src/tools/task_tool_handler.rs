use crate::utils::mcp_utils::MCPUtils;
use crate::utils::tool_metadata::TASKS_TOOL_METADATA;
use mindvault_core::repository::task_repo::TaskRepository;
use mindvault_shared::models::tasks_model::TaskResponse;
use rmcp::handler::server::tool::ToolRouter;
use rmcp::model::{
    CallToolResult, Content, Implementation, ProtocolVersion, ServerCapabilities, ServerInfo,
};
use rmcp::ErrorData as RMCPError;
use rmcp::{tool, tool_handler, tool_router, ServerHandler};
use std::sync::Arc;

#[derive(Debug)]
pub(crate) struct TaskToolServer {
    task_repository: Arc<TaskRepository>,
    pub(crate) tool_router: ToolRouter<Self>,
}

impl TaskToolServer {
    pub(crate) async fn new(task_repository: Arc<TaskRepository>) -> TaskToolServer {
        TaskToolServer {
            task_repository,
            tool_router: Self::tool_router(),
        }
    }
}

#[tool_router]
impl TaskToolServer {
    #[tool(
        name = "get_user_task_list",
        description = r#"Fetch all tasks associated with the current user account.
        Provides a comprehensive view of user's task inventory with basic task information."#
    )]
    pub async fn get_user_tasks(&self) -> Result<CallToolResult, RMCPError> {
        let tasks = self.task_repository.find_all().await;
        match tasks {
            Ok(tasks) => {
                let task_responses: Vec<TaskResponse> =
                    tasks.into_iter().map(TaskResponse::from).collect();
                Ok(MCPUtils::into_call_tool_result(task_responses))
            }
            Err(e) => {
                let error_message = format!("Error finding tasks: {:?}", e);
                Ok(CallToolResult::error(vec![Content::text(error_message)]))
            }
        }
    }
}

#[tool_handler]
//noinspection RsSortImplTraitMembers
impl ServerHandler for TaskToolServer {
    fn get_info(&self) -> ServerInfo {
        let tool_instruction = TASKS_TOOL_METADATA;
        ServerInfo {
            protocol_version: ProtocolVersion::V_2025_03_26,
            instructions: Some(tool_instruction.to_string()),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation::from_build_env(),
        }
    }
}
