use crate::handle_service_response;
use crate::models::ApiResponse;
use crate::services::task_service::TaskService;
use axum::extract::{Path, Query, State};
use axum::routing::get;
use axum::Router;
use mindvault_core::models::AppDatabase;
use mindvault_shared::models::tasks_model::Task;
use std::sync::Arc;
use tracing::info;
use mindvault_shared::dtos::task_dtos::TaskSearchParams;

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
                get(TaskRouter::get_tasks_handler)
            )
            .route(
                "/{:id}",
                get(TaskRouter::get_task_by_id_handler)
            )
            .route(
                "/search",
                get(TaskRouter::search_tasks_by_text_handler)
            )
            .with_state(self.task_service.clone())
    }

    async fn get_tasks_handler(State(task_service): State<Arc<TaskService>>) -> ApiResponse<Vec<Task>> {
        info!("Fetching tasks from database");
        handle_service_response!(
            task_service.get_all_tasks().await,
            "Fetched {} tasks", |data: &Vec<Task>| data.len(),
            "Failed to get tasks"
        )
    }

    async fn get_task_by_id_handler(
        State(task_service): State<Arc<TaskService>>,
        Path(id): Path<i64>,
    ) -> ApiResponse<Task> {
        info!("Fetching task with id {} from database", id);
        handle_service_response!(
            task_service.get_by_id(id).await,
            "Found task with id {}", |data: &Task| data.id,
            "Failed to get current task"
        )
    }

    // ?search=Update Document
    async fn search_tasks_by_text_handler(
        State(task_service): State<Arc<TaskService>>,
        Query(params): Query<TaskSearchParams>,
    ) -> ApiResponse<Vec<Task>> {
        info!("Searching tasks with params {:?}", params);
        handle_service_response!(
            task_service.search_tasks(params).await,
            "Found tasks with params {:?}", |data: &Vec<Task>| data.len(),
            "Failed to search tasks"
        )
    }
}