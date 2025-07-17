use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use crate::services::task_service::TaskService;
use core::models::DbClient;
use shared::models::task_model::{CreateTaskRequest};
use std::sync::Arc;
use tracing::{info, error, debug};

pub(crate) struct TaskRouter {
    task_service: Arc<TaskService>,
}

impl TaskRouter {
    pub(crate) fn new(db_client: DbClient) -> Self {
        let task_service = Arc::new(TaskService::new(db_client));
        Self { task_service }
    }

    pub(crate) fn get_routes(&self) -> Router {
        Router::new()
            .route("/", post(create_task_handler).get(get_tasks_handler))
            .route("/:id", get(get_task_by_id_handler))
            .with_state(self.task_service.clone())
    }
}

// Route Handlers
async fn create_task_handler(
    State(task_service): State<Arc<TaskService>>,
    Json(new_task_request): Json<CreateTaskRequest>,
) -> impl IntoResponse {
    match task_service.create_task(new_task_request) {
        Ok(created_task) => {
            info!("Task created successfully with ID: {}", created_task.id);
            (StatusCode::CREATED, Json(created_task)).into_response()
        }
        Err(e) => {
            error!("Failed to create task: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(format!("Failed to create task: {:?}", e)),
            )
                .into_response()
        }
    }
}

async fn get_tasks_handler(
    State(task_service): State<Arc<TaskService>>,
) -> impl IntoResponse {
    info!("Fetching all tasks");
    match task_service.get_all_tasks() {
        Ok(tasks) => {
            debug!("Fetched {} tasks", tasks.len());
            Json(tasks).into_response()
        }
        Err(e) => {
            error!("Failed to get tasks: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(format!("Failed to get tasks: {:?}", e)),
            )
                .into_response()
        }
    }
}

async fn get_task_by_id_handler(
    State(task_service): State<Arc<TaskService>>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    info!("Fetching task with ID: {}", id);
    match task_service.get_task_by_id(id) {
        Ok(task) => {
            debug!("Task found: {:?}", task);
            Json(task).into_response()
        }
        Err(e) => {
            error!("Task with ID {} not found: {:?}", id, e);
            (
                StatusCode::NOT_FOUND,
                Json(format!("Task not found: {:?}", e)),
            )
                .into_response()
        }
    }
}
