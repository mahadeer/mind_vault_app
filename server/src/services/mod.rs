pub mod task_service;

use axum::Router;
use core::models::DbClient;
use crate::routes::task_routes::TaskRouter;
use crate::services::task_service::TaskService;

pub struct ApiService {
    pub db_client: DbClient,
}

impl ApiService {
    pub(crate) fn new(db_client: DbClient) -> ApiService {
        ApiService {
            db_client
        }
    }

    pub(crate) fn get_service_handlers(&self) -> Router {
        let task_router = TaskRouter::new(self.db_client.clone());
        let router = Router::new()
            .nest("/tasks", task_router.get_routes());
        router
    }
}