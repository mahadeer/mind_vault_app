use crate::models::task_models::{CreateTaskParams, GetTaskByIdParams, UpdateTaskByIdParams};
use crate::utils::mcp_utils::{EnumFormatter, MCPUtils};
use crate::utils::tool_metadata::TASKS_TOOL_METADATA;
use mindvault_core::repository::task_repo::TaskRepository;
use mindvault_shared::dtos::task_dtos::{CreateTaskRequest, UpdateTaskRequest};
use mindvault_shared::models::tasks_model::TaskResponse;
use rmcp::ErrorData as RMCPError;
use rmcp::handler::server::tool::{Parameters, ToolRouter};
use rmcp::model::{
    CallToolResult, Content, Implementation, ProtocolVersion, ServerCapabilities, ServerInfo,
};
use rmcp::{ServerHandler, tool, tool_handler, tool_router};
use std::sync::Arc;
use tracing::info;

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
        name = "get_current_date",
        description = "Get the current date and time. Returns the current date in multiple formats for easy reference."
    )]
    pub async fn get_current_date(&self) -> Result<CallToolResult, RMCPError> {
        let now = chrono::Utc::now();
        let local_now = chrono::Local::now();

        let content = vec![
            Content::text("📅 Current Date & Time Information:".to_string()),
            Content::text(format!("🌍 UTC: {}", now.format("%Y-%m-%d %H:%M:%S UTC"))),
            Content::text(format!(
                "🏠 Local: {}",
                local_now.format("%Y-%m-%d %H:%M:%S %Z")
            )),
            Content::text(format!("📋 Date Only: {}", local_now.format("%Y-%m-%d"))),
            Content::text(format!("⏰ Time Only: {}", local_now.format("%H:%M:%S"))),
            Content::text(format!(
                "📆 Formatted: {}",
                local_now.format("%A, %B %d, %Y")
            )),
            Content::text(format!("🗓️ ISO 8601: {}", now.to_rfc3339())),
        ];

        Ok(CallToolResult::success(content))
    }

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

    #[tool(
        name = "get_user_task_by_id",
        description = r#"Fetch a specific task by its unique identifier.
        Provides detailed information about the task."#
    )]
    pub async fn get_user_task_by_id(
        &self,
        Parameters(params): Parameters<GetTaskByIdParams>,
    ) -> Result<CallToolResult, RMCPError> {
        let task_id = params.task_id;
        let task = self.task_repository.find_by_id(task_id).await;
        match task {
            Ok(Some(task)) => {
                let task_response = TaskResponse::from(task);
                Ok(MCPUtils::into_call_tool_result(vec![task_response]))
            }
            Ok(None) => {
                let error_message = format!("Task with id {} not found", task_id);
                Ok(CallToolResult::error(vec![Content::text(error_message)]))
            }
            Err(e) => {
                let error_message = format!("Error finding task: {:?}", e);
                Ok(CallToolResult::error(vec![Content::text(error_message)]))
            }
        }
    }

    #[tool(
        name = "create_user_task",
        description = r#"Create a new task for the current user.
        Requires task name, and optional due dates(DD-MM-YYYY), priority(high|normal|low), status(complete|pending|in_progress).
        Status options: 'NotStarted', 'Pending', 'InProgress', 'Completed'
        Priority options: 'Normal', 'High'"#
    )]
    pub async fn create_user_task(
        &self,
        Parameters(CreateTaskParams {
            name,
            due_date,
            priority,
            status,
        }): Parameters<CreateTaskParams>,
    ) -> Result<CallToolResult, RMCPError> {
        let params = CreateTaskRequest {
            name,
            due_date: due_date.and_then(|d| chrono::NaiveDate::parse_from_str(&d, "%d-%m-%Y").ok()),
            priority: priority.as_type(),
            status: status.as_type(),
        };
        let created_task = self.task_repository.create_task(params).await;
        match created_task {
            Ok(task) => {
                let task_response = TaskResponse::from(task);
                info!("Created task: {:?}", task_response);
                Ok(MCPUtils::into_call_tool_result(vec![task_response]))
            }
            Err(e) => {
                let error_message = format!("Error creating task: {:?}", e);
                Ok(CallToolResult::error(vec![Content::text(error_message)]))
            }
        }
    }

    #[tool(
        name = "update_user_task_by_id",
        description = r#"Update a specific task by its unique identifier.
        Allows modification of task properties including status (complete|pending|in_progress), priority(high|normal|low), due dates(DD-MM-YYYY).
        Status options: 'NotStarted', 'Pending', 'InProgress', 'Completed'
        Priority options: 'Normal', 'High'"#
    )]
    pub async fn update_user_task(
        &self,
        Parameters(UpdateTaskByIdParams {
            task_id,
            status,
            priority,
            due_date,
        }): Parameters<UpdateTaskByIdParams>,
    ) -> Result<CallToolResult, RMCPError> {
        let status = status.as_type();
        let priority = priority.as_type();
        let updated_task = UpdateTaskRequest {
            status,
            priority,
            due_date: due_date.and_then(|d| chrono::NaiveDate::parse_from_str(&d, "%d-%m-%Y").ok()),
        };
        let updated_task = self
            .task_repository
            .update_task_by_id(task_id, updated_task)
            .await;
        match updated_task {
            Ok(Some(task)) => {
                let task_response = TaskResponse::from(task);
                info!("Updated task: {:?}", task_response);
                Ok(MCPUtils::into_call_tool_result(vec![task_response]))
            }
            Ok(None) => {
                let error_message = format!("Task with id {} not found", task_id);
                Ok(CallToolResult::error(vec![Content::text(error_message)]))
            }
            Err(e) => {
                let error_message = format!("Error updating task: {:?}", e);
                Ok(CallToolResult::error(vec![Content::text(error_message)]))
            }
        }
    }

    #[tool(
        name = "delete_user_task_by_id",
        description = r#"Delete a specific task by its unique identifier.
        Soft deletes the task, maintaining data integrity and recovery options."#
    )]
    pub async fn delete_user_task(
        &self,
        Parameters(params): Parameters<GetTaskByIdParams>,
    ) -> Result<CallToolResult, RMCPError> {
        let task_id = params.task_id;
        let deleted = self.task_repository.soft_delete_by_id(task_id).await;
        match deleted {
            Ok(true) => {
                let success_message = format!("Task with id {} deleted", task_id);
                Ok(CallToolResult::success(vec![Content::text(success_message)]))
            }
            Ok(false) => {
                let error_message = format!("Task with id {} not found", task_id);
                Ok(CallToolResult::error(vec![Content::text(error_message)]))
            }
            Err(e) => {
                let error_message = format!("Error deleting task: {:?}", e);
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
