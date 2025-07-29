use crate::models::task_models::{BulkCreateTaskParams, CreateTaskParams, DeleteTasksByStatusParams, GetTaskByIdParams, SearchAndUpdateParams, SearchTasksParams, UpdateTaskByIdParams};
use crate::utils::mcp_utils::{EnumFormatter, MCPUtils};
use crate::utils::tool_metadata::TASKS_TOOL_METADATA;
use mindvault_core::repository::task_repo::TaskRepository;
use mindvault_shared::dtos::task_dtos::{BulkCreateTaskRequest, CreateTaskRequest, SearchAndUpdateRequest, TaskSearchParams, UpdateTaskRequest};
use mindvault_shared::models::tasks_model::{ETaskStatus, TaskResponse};
use rmcp::handler::server::tool::{Parameters, ToolRouter};
use rmcp::model::{
    CallToolResult, Content, Implementation, ProtocolVersion, ServerCapabilities, ServerInfo,
};
use rmcp::ErrorData as RMCPError;
use rmcp::{tool, tool_handler, tool_router, ServerHandler};
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
        description = r#"Retrieve comprehensive date and time information in multiple standardized
         formats (UTC, local timezone, ISO 8601, human-readable).
         Essential for accurate task scheduling, deadline setting, and temporal context
         in task management workflows."#
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
        name = "list_all_user_tasks",
        description = r#"Retrieve a comprehensive inventory of all tasks belonging
        to the authenticated user. Returns task metadata including
        IDs, names, statuses, priorities, and due dates.
        Ideal for dashboard views, task overview reports, and
        bulk task management operations."#
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
        name = "fetch_task_details_by_id",
        description = r#"Retrieve detailed information for a specific task using its
        unique identifier. Returns complete task metadata including
        creation timestamps, modification history, and all associated properties.
        Perfect for task detail views and targeted task operations."#
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
        name = "create_new_task",
        description = r#"Create a new task with comprehensive configuration.
        Supports task naming, flexible due date scheduling (DD-MM-YYYY format),
        priority classification (Normal/High), and status initialization (NotStarted/Pending/InProgress/Completed).
        Automatically timestamps creation and assigns to current user."#
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
        description = r#"Update specific properties of an existing task identified by its unique ID.
        Supports selective modification of status progression, priority adjustments, and due date
        rescheduling. Maintains audit trail and preserves unchanged properties.
        Ideal for task lifecycle management and workflow transitions.
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
                Ok(CallToolResult::success(vec![Content::text(
                    success_message,
                )]))
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

    #[tool(
        name = "bulk_create_tasks",
        description = r#"Efficiently create multiple tasks in a single atomic operation with individual metadata configuration.
        Supports batch processing of task arrays with per-task customization of names, due dates, priorities, and statuses.
        Optimized for project initialization, template deployment, and large-scale task management scenarios.
        Requires an array of tasks, each with name, and optional due dates(DD-MM-YYYY), priority(high|normal|low), status(complete|pending|in_progress).
        Status options: 'NotStarted', 'Pending', 'InProgress', 'Completed'
        Priority options: 'Normal', 'High'
        If the optional fields are not provided, they will be set to default values."#
    )]
    pub async fn bulk_create_tasks(
        &self,
        Parameters(bulk_request): Parameters<BulkCreateTaskParams>,
    ) -> Result<CallToolResult, RMCPError> {
        let bulk_request = BulkCreateTaskRequest {
            tasks: bulk_request
                .names
                .into_iter()
                .zip(bulk_request.statuses.into_iter())
                .zip(bulk_request.priorities.into_iter())
                .zip(bulk_request.due_dates.into_iter())
                .map(|(((name, status), priority), due_date)| CreateTaskRequest {
                    name,
                    due_date: due_date
                        .and_then(|d| chrono::NaiveDate::parse_from_str(&d, "%d-%m-%Y").ok()),
                    priority: priority.as_type(),
                    status: status.as_type(),
                })
                .collect(),
        };
        let created_tasks = self.task_repository.bulk_create_tasks(bulk_request).await;
        match created_tasks {
            Ok(tasks) => {
                let task_responses: Vec<TaskResponse> =
                    tasks.into_iter().map(TaskResponse::from).collect();
                Ok(MCPUtils::into_call_tool_result(task_responses))
            }
            Err(e) => {
                let error_message = format!("Error bulk creating tasks: {:?}", e);
                Ok(CallToolResult::error(vec![Content::text(error_message)]))
            }
        }
    }

    #[tool(
        name = "search_tasks_by_params",
        description = r#"Search for tasks based on specific criteria.
        Allows complex queries combining multiple filters, can be used to find bulk tasks by optional parameters.
        Supports filtering by task name(query), status, priority, and due date.
        Returns matching tasks with complete metadata.
        Schema: {
            "query": "optional search term",
            "status": "optional status filter",
            "priority": "optional priority filter",
            "due_date": "optional due date filter"
        }"#
    )]
    pub async fn search_tasks(
        &self,
        Parameters(params): Parameters<SearchTasksParams>,
    ) -> Result<CallToolResult, RMCPError> {
        let params = TaskSearchParams {
            query: params.query,
            status: params.status.as_type(),
            priority: params.priority.as_type(),
            due_date: params.due_date.and_then(|d| chrono::NaiveDate::parse_from_str(&d, "%d-%m-%Y").ok()),
        };
        let tasks = self.task_repository.search_task(params).await;
        match tasks {
            Ok(tasks) => {
                let task_responses: Vec<TaskResponse> =
                    tasks.into_iter().map(TaskResponse::from).collect();
                Ok(MCPUtils::into_call_tool_result(task_responses))
            }
            Err(e) => {
                let error_message = format!("Error searching tasks: {:?}", e);
                Ok(CallToolResult::error(vec![Content::text(error_message)]))
            }
        }
    }

    #[tool(
        name = "search_and_update_tasks",
        description = r#"Search for tasks based on specific criteria and update them.
        Allows complex queries combining multiple filters, can be used to find bulk tasks by optional parameters.
        Supports filtering by task name(query), status, priority, and due date.
        Returns matching tasks with complete metadata.
        Schema: {
            "query": "optional search term",
            "status_filter": "optional status filter",
            "priority_filter": "optional priority filter",
            "due_date_filter": "optional due date filter",
            "status": "optional status update",
            "priority": "optional priority update",
            "due_date": "optional due date update"
        }"#
    )]
    pub async fn search_and_update_tasks(
        &self,
        Parameters(params): Parameters<SearchAndUpdateParams>,
    ) -> Result<CallToolResult, RMCPError> {
        let params = SearchAndUpdateRequest {
            query: params.query,
            status_filter: params.status_filter.as_type(),
            priority_filter: params.priority_filter.as_type(),
            due_date_filter: params.due_date_filter.and_then(|d| chrono::NaiveDate::parse_from_str(&d, "%d-%m-%Y").ok()),
            due_date: params.due_date.and_then(|d| chrono::NaiveDate::parse_from_str(&d, "%d-%m-%Y").ok()),
            priority: params.priority.as_type(),
            status: params.status.as_type(),
        };
        let tasks = self.task_repository.search_and_update_tasks(params).await;
        match tasks {
            Ok(tasks) => {
                let task_responses: Vec<TaskResponse> =
                    tasks.into_iter().map(TaskResponse::from).collect();
                Ok(MCPUtils::into_call_tool_result(task_responses))
            }
            Err(e) => {
                let error_message = format!("Error searching and updating tasks: {:?}", e);
                Ok(CallToolResult::error(vec![Content::text(error_message)]))
            }
        }
    }

    #[tool(
        name = "bulk_delete_tasks_by_status",
        description = r#"Bulk delete tasks by status.
        Deletes all tasks with the specified status.
        Supports filtering by task status.
        Returns the number of deleted tasks.
        Schema: {
            "status": "status to delete"
        }"#
    )]
    pub async fn bulk_delete_tasks_by_status(
        &self,
        Parameters(params): Parameters<DeleteTasksByStatusParams>,
    ) -> Result<CallToolResult, RMCPError> {
        let status: ETaskStatus = params.status.as_type().unwrap();
        let deleted = self.task_repository.bulk_soft_delete_by_status(status).await;
        match deleted {
            Ok(count) => {
                let success_message = format!("Deleted {} tasks", count);
                Ok(CallToolResult::success(vec![Content::text(
                    success_message,
                )]))
            }
            Err(e) => {
                let error_message = format!("Error deleting tasks: {:?}", e);
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
