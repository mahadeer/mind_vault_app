use crate::handle_service_response;
use crate::models::ApiResponse;
use crate::services::task_service::TaskService;
use axum::extract::State;
use axum::routing::get;
use axum::Router;
use mindvault_core::models::AppDatabase;
use mindvault_shared::models::tasks_model::Task;
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
                get(TaskRouter::get_tasks_handler)
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
}