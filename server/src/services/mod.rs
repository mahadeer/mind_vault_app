pub mod task_service;

use crate::routes::task_routes::TaskRouter;
use axum::Router;
use axum::response::Html;
use axum::routing::get;
use core::models::AppDatabase;

pub struct ApiService {
    pub app_database: AppDatabase,
}

impl ApiService {
    pub(crate) fn new(app_database: AppDatabase) -> ApiService {
        ApiService { app_database }
    }

    pub(crate) fn get_service_handlers(&self) -> Router {
        let task_router = TaskRouter::new(self.app_database.clone());
        let router = Router::new()
            .route("/", get(index_handler))
            .nest("/tasks", task_router.get_routes());
        router
    }
}

async fn index_handler() -> Html<&'static str> {
    Html(include_str!("../templates/index.html"))
}
