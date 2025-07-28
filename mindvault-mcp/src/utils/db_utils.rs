use std::sync::Arc;
use mindvault_core::repository::task_repo::TaskRepository;

pub(crate) async fn get_task_repository() -> Arc<TaskRepository> {
    let app_database_future = mindvault_core::db::get_app_database().await;
    let app_database = match app_database_future { 
        Ok(db) => db,
        Err(e) => {
            panic!("Failed to connect to database: {:?}", e);
        }
    };
    Arc::new(TaskRepository::new(app_database))
}