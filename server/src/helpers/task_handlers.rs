use crate::services::task_service::TaskService;
use axum::extract::Query;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use shared::models::task_model::{CreateTaskRequest, UpdateTaskRequest};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{debug, error, info};

// Route Handlers
pub async fn create_task_handler(
    State(task_service): State<Arc<TaskService>>,
    Json(new_task_request): Json<CreateTaskRequest>,
) -> impl IntoResponse {
    match task_service.create_task(new_task_request).await {
        Ok(created_task) => {
            info!(
                "Task created successfully with ID: {}",
                created_task.id.clone()
            );
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

pub async fn get_tasks_handler(State(task_service): State<Arc<TaskService>>) -> impl IntoResponse {
    info!("Fetching all tasks");
    match task_service.get_all_tasks().await {
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

pub async fn get_task_by_id_handler(
    State(task_service): State<Arc<TaskService>>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    info!("Fetching task with ID: {}", id);
    match task_service.get_task_by_id(id).await {
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

pub async fn update_task_by_id_handler(
    State(task_service): State<Arc<TaskService>>,
    Path(id): Path<i64>,
    Json(updated_task): Json<UpdateTaskRequest>,
) -> impl IntoResponse {
    info!("Task updated with ID: {}", id);
    match task_service.update_task_by_id(id, updated_task).await {
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

pub async fn delete_task_by_id_handler(
    State(task_service): State<Arc<TaskService>>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    info!("Task deleted with ID: {}", id);
    match task_service.delete_task_by_id(id).await {
        Ok(result) => {
            info!("Task successfully deleted with ID: {}", id);
            Ok((StatusCode::ACCEPTED, result).into_response())
        }
        Err(e) => {
            error!("Task with ID {} not found: {:?}", id, e);
            Err((
                StatusCode::NOT_FOUND,
                Json(format!("Task not found: {:?}", e)),
            ))
        }
    }
}

pub async fn delete_all_tasks_handler(
    State(task_service): State<Arc<TaskService>>,
) -> impl IntoResponse {
    info!("Deleting all tasks");
    match task_service.delete_all_tasks().await {
        Ok(result) => {
            info!("Tasks successfully deleted");
            Ok((StatusCode::ACCEPTED, result).into_response())
        }
        Err(e) => {
            error!("Unable to delete all tasks: {:?}", e);
            Err((
                StatusCode::NOT_FOUND,
                Json(format!("Unable to delete all tasks: {:?}", e)),
            ))
        }
    }
}

pub async fn get_tasks_by_text_handler(
    State(task_service): State<Arc<TaskService>>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    info!("Getting tasks matching text");
    let search_term = params.get("search_term").unwrap().as_str();
    match task_service.get_tasks_by_text(search_term).await {
        Ok(tasks) => {
            Json(tasks).into_response()
        }
        Err(e) => {
            error!("Failed to get tasks: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json("Failed to get tasks".to_string())).into_response()
        }
    }
}

pub async fn update_tasks_by_text_handler(
    State(task_service): State<Arc<TaskService>>,
    Query(params): Query<HashMap<String, String>>,
    Json(updated_task): Json<UpdateTaskRequest>,
) -> impl IntoResponse {
    info!("Updating tasks matching text");
    let search_term = params.get("search_term").unwrap().as_str();
    match task_service.search_and_update(search_term, updated_task).await { 
        Ok(tasks) => {
            Json(tasks).into_response()
        }
        Err(e) => {
            error!("Failed to update tasks: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json("Failed to get tasks".to_string())).into_response()
        }
    }
}

pub async fn delete_tasks_by_text_handler(
    State(task_service): State<Arc<TaskService>>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    info!("Deleting tasks matching text");
    let search_term = params.get("search_term").unwrap().as_str();
    match task_service.search_and_delete(search_term).await {
        Ok(result) => {
            info!("Tasks successfully deleted");
            Ok((StatusCode::ACCEPTED, result).into_response())
        }
        Err(e) => {
            error!("Unable to delete all tasks: {:?}", e);
            Err((
                StatusCode::NOT_FOUND,
                Json(format!("Unable to delete all tasks: {:?}", e)),
            ))
        }
    }
}
