mod task_router;

use axum::extract::State;
use axum::response::Html;
use axum::Router;
use axum::routing::get;
use mindvault_core::models::AppDatabase;
use crate::router::task_router::TaskRouter;

pub struct MindVaultRouter {
    pub db_client: AppDatabase,
}

impl MindVaultRouter {
    pub(crate) fn new(db_client: AppDatabase) -> Self {
        MindVaultRouter { db_client }
    }

    pub(crate) fn get_router(&self) -> Router {
        let server_up_since = chrono::Local::now()
            .format("%d/%m/%y %H:%M %Z")
            .to_string()
            .to_string();
        let router = Router::new()
            .route("/", get(root_handler).with_state(server_up_since))
            .nest("/tasks", self.get_task_router());
        router
    }

    fn get_task_router(&self) -> Router {
        let task_router = TaskRouter::new(self.db_client.clone());
        task_router.get_routes()
    }
}

async fn root_handler(State(server_up_since): State<String>) -> Html<String> {
    let current_time = chrono::Local::now().format("%d/%m/%y %H:%M %Z").to_string();
    let resp = format!(
        r#"
        App is running!
        <p>Since <b>{}</b><p>
        <p>Now <b>{}</b></p>
    "#,
        server_up_since, current_time
    );
    Html(resp)
}