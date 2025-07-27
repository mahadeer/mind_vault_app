use crate::handle_service_response;
use crate::models::{ApiResponse, ApiTextResponse};
use crate::services::task_service::TaskService;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::routing::{delete, get, post};
use axum::{Json, Router};
use mindvault_core::models::AppDatabase;
use mindvault_shared::dtos::task_dtos::{
    BulkCreateTaskRequest, CreateTaskRequest, SearchAndUpdateRequest, TaskSearchParams,
    UpdateTaskRequest,
};
use mindvault_shared::models::tasks_model::{ETaskStatus, TaskResponse};
use std::sync::Arc;
use tracing::info;

pub(crate) struct TaskRouter {
    task_service: Arc<TaskService>,
}

impl TaskRouter {
    pub(crate) fn new(app_database: AppDatabase) -> Self {
        let task_service = Arc::new(TaskService::new(app_database));
        Self { task_service }
    }

    pub(crate) fn get_routes(&self) -> Router {
        Router::new()
            .route(
                "/",
                get(TaskRouter::get_tasks_handler).post(TaskRouter::create_task_handler),
            )
            .route(
                "/{:id}",
                get(TaskRouter::get_task_by_id_handler)
                    .put(TaskRouter::update_task_handler)
                    .delete(TaskRouter::delete_task_handler),
            )
            .route(
                "/search",
                get(TaskRouter::search_tasks_by_text_handler)
                    .put(TaskRouter::search_and_update_handler),
            )
            .route("/bulk", post(TaskRouter::bulk_create_tasks_handler))
            .route(
                "/status/{:status}",
                delete(TaskRouter::bulk_delete_by_status_handler),
            )
            .with_state(self.task_service.clone())
    }

    async fn create_task_handler(
        State(task_service): State<Arc<TaskService>>,
        Json(payload): Json<CreateTaskRequest>,
    ) -> ApiResponse<TaskResponse> {
        if payload.name.trim().is_empty() {
            return Err((
                StatusCode::BAD_REQUEST,
                "Name field is required".to_string(),
            ));
        }
        handle_service_response!(
            task_service.create_task(payload).await,
            "Created a new task {}",
            |data: &TaskResponse| data.id,
            "Unable to insert a new task into database"
        )
    }

    async fn bulk_create_tasks_handler(
        State(task_service): State<Arc<TaskService>>,
        Json(payload): Json<BulkCreateTaskRequest>,
    ) -> ApiResponse<Vec<TaskResponse>> {
        if payload.tasks.is_empty() {
            return Err((
                StatusCode::BAD_REQUEST,
                "Tasks array cannot be empty".to_string(),
            ));
        }

        // Validate that all tasks have non-empty names
        for task in &payload.tasks {
            if task.name.trim().is_empty() {
                return Err((
                    StatusCode::BAD_REQUEST,
                    "All tasks must have non-empty names".to_string(),
                ));
            }
        }

        handle_service_response!(
            task_service.bulk_create_tasks(payload).await,
            "Bulk created {} tasks",
            |data: &Vec<TaskResponse>| data.len(),
            "Unable to bulk insert tasks into database"
        )
    }

    async fn get_tasks_handler(
        State(task_service): State<Arc<TaskService>>,
    ) -> ApiResponse<Vec<TaskResponse>> {
        info!("Fetching tasks from database");
        handle_service_response!(
            task_service.get_all_tasks().await,
            "Fetched {} tasks",
            |data: &Vec<TaskResponse>| data.len(),
            "Failed to get tasks"
        )
    }

    async fn get_task_by_id_handler(
        State(task_service): State<Arc<TaskService>>,
        Path(id): Path<i64>,
    ) -> ApiResponse<TaskResponse> {
        info!("Fetching task with id {} from database", id);
        handle_service_response!(
            task_service.get_by_id(id).await,
            "Found task with id {}",
            |data: &TaskResponse| data.id,
            "Failed to get current task"
        )
    }

    // ?search=Update Document
    async fn search_tasks_by_text_handler(
        State(task_service): State<Arc<TaskService>>,
        Query(params): Query<TaskSearchParams>,
    ) -> ApiResponse<Vec<TaskResponse>> {
        info!("Searching tasks with params {:?}", params);
        handle_service_response!(
            task_service.search_tasks(params).await,
            "Found tasks with params {:?}",
            |data: &Vec<TaskResponse>| data.len(),
            "Failed to search tasks"
        )
    }

    async fn delete_task_handler(
        State(task_service): State<Arc<TaskService>>,
        Path(id): Path<i64>,
    ) -> ApiTextResponse {
        info!("Soft deleting task with id {}", id);
        match task_service.soft_delete_task(id).await {
            Ok(true) => Ok("Task deleted successfully".to_string()),
            Ok(false) => Err((StatusCode::NOT_FOUND, "Task not found".to_string())),
            Err(e) => {
                let error_message = format!("Failed to delete task: {:?}", e);
                Err((StatusCode::INTERNAL_SERVER_ERROR, error_message))
            }
        }
    }

    async fn update_task_handler(
        State(task_service): State<Arc<TaskService>>,
        Path(id): Path<i64>,
        Json(payload): Json<UpdateTaskRequest>,
    ) -> ApiResponse<TaskResponse> {
        info!("Updating task with id {} with payload {:?}", id, payload);

        // Validate that at least one field is provided
        if payload.status.is_none() && payload.due_date.is_none() && payload.priority.is_none() {
            return Err((
                StatusCode::BAD_REQUEST,
                "At least one field must be provided for update".to_string(),
            ));
        }

        match task_service.update_task(id, payload).await {
            Ok(Some(task)) => Ok(Json(task)),
            Ok(None) => Err((StatusCode::NOT_FOUND, "Task not found".to_string())),
            Err(e) => {
                let error_message = format!("Failed to update task: {:?}", e);
                Err((StatusCode::INTERNAL_SERVER_ERROR, error_message))
            }
        }
    }

    async fn bulk_delete_by_status_handler(
        State(task_service): State<Arc<TaskService>>,
        Path(status_str): Path<String>,
    ) -> ApiTextResponse {
        info!("Bulk deleting tasks with status {}", status_str);

        // Parse the status string
        let status = match status_str.as_str() {
            "NotStarted" => ETaskStatus::NotStarted,
            "Pending" => ETaskStatus::Pending,
            "InProgress" => ETaskStatus::InProgress,
            "Completed" => ETaskStatus::Completed,
            _ => return Err((StatusCode::BAD_REQUEST, "Invalid status".to_string())),
        };

        match task_service.bulk_delete_by_status(status).await {
            Ok(count) => Ok(format!("Deleted {} tasks", count)),
            Err(e) => {
                let error_message = format!("Failed to bulk delete tasks: {:?}", e);
                Err((StatusCode::INTERNAL_SERVER_ERROR, error_message))
            }
        }
    }

    async fn search_and_update_handler(
        State(task_service): State<Arc<TaskService>>,
        Json(payload): Json<SearchAndUpdateRequest>,
    ) -> ApiResponse<Vec<TaskResponse>> {
        info!("Search and update with payload {:?}", payload);

        // Validate that at least one update field is provided
        if payload.status.is_none() && payload.due_date.is_none() && payload.priority.is_none() {
            return Err((
                StatusCode::BAD_REQUEST,
                "At least one update field must be provided".to_string(),
            ));
        }

        handle_service_response!(
            task_service.search_and_update_tasks(payload).await,
            "Updated {} tasks",
            |data: &Vec<TaskResponse>| data.len(),
            "Failed to search and update tasks"
        )
    }
}
