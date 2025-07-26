use axum::response::ErrorResponse;
use tracing::error;
use mindvault_core::models::AppDatabase;
use mindvault_core::repository::task_repo::TaskRepository;
use mindvault_shared::dtos::task_dtos::TaskSearchParams;
use mindvault_shared::models::tasks_model::Task;

pub(crate) struct TaskService {
    task_repository: TaskRepository
}

impl TaskService {
    pub(crate) fn new(app_database: AppDatabase) -> Self {
        let task_repository = TaskRepository::new(app_database);
        Self { task_repository }
    }

    pub(crate) async fn get_all_tasks(&self) -> Result<Vec<Task>, ErrorResponse> {
        let tasks = self.task_repository.find_all().await;
        match tasks {
            Ok(tasks) => Ok(tasks),
            Err(e) => {
                let error_message = format!("Error finding tasks: {}", e);
                error!(error_message);
                Err(ErrorResponse::from(error_message))
            }
        }
    }

    pub(crate) async fn get_by_id(&self, task_id: i64) -> Result<Task, ErrorResponse> {
        let task = self.task_repository.find_by_id(task_id).await;
        match task {
            Ok(task) => Ok(task.unwrap()),
            Err(e) => {
                let error_message = format!("Error finding task: {}", e);
                error!(error_message);
                Err(ErrorResponse::from(error_message))
            }
        }
    }

    pub(crate) async fn search_tasks(&self, params: TaskSearchParams) -> Result<Vec<Task>, ErrorResponse> {
        let tasks = self.task_repository.search_task(params).await;
        match tasks {
            Ok(tasks) => Ok(tasks),
            Err(e) => {
                let error_message = format!("Error searching tasks: {}", e);
                error!(error_message);
                Err(ErrorResponse::from(error_message))
            }
        }
    }
}
