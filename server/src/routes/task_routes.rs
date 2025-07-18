use crate::helpers::task_handlers::{create_task_handler, delete_all_tasks_handler, delete_task_by_id_handler, delete_tasks_by_text_handler, get_task_by_id_handler, get_tasks_by_text_handler, get_tasks_handler, update_task_by_id_handler, update_tasks_by_text_handler};
use crate::services::task_service::TaskService;
use axum::{
    Router,
    routing::{get, post},
};
use core::models::AppDatabase;
use std::sync::Arc;

pub(crate) struct TaskRouter {
    task_service: Arc<TaskService>,
}

impl TaskRouter {
    pub(crate) fn new(db_client: AppDatabase) -> Self {
        let task_service = Arc::new(TaskService::new(db_client));
        Self { task_service }
    }

    pub(crate) fn get_routes(&self) -> Router {
        Router::new()
            .route(
                "/",
                post(create_task_handler)
                    .get(get_tasks_handler)
                    .delete(delete_all_tasks_handler),
            )
            .route(
                "/{:id}",
                get(get_task_by_id_handler)
                    .post(update_task_by_id_handler)
                    .put(update_task_by_id_handler)
                    .delete(delete_task_by_id_handler),
            )
            .route(
                "/search",
                get(get_tasks_by_text_handler)
                    .post(update_tasks_by_text_handler)
                    .delete(delete_tasks_by_text_handler),
            )
            .with_state(self.task_service.clone())
    }
}
