use axum::response::ErrorResponse;
use core::models::AppDatabase;
use core::repository::task_repo::TaskRepo;
use shared::models::task_model::{CreateTaskRequest, Task, UpdateTaskRequest};
use tracing::error;

#[derive(Clone, Debug)]
pub(crate) struct TaskService {
    task_repo: TaskRepo,
}

impl TaskService {
    pub(crate) fn new(app_database: AppDatabase) -> TaskService {
        let task_repo = TaskRepo::new(app_database);
        TaskService { task_repo }
    }

    pub(crate) async fn create_task(
        &self,
        new_task: CreateTaskRequest,
    ) -> Result<Task, ErrorResponse> {
        let created_task = self.task_repo.add_task(new_task).await;
        Ok(created_task)
    }

    pub(crate) async fn get_all_tasks(&self) -> Result<Vec<Task>, ErrorResponse> {
        let tasks = self.task_repo.find_all().await;
        match tasks {
            Ok(tasks) => Ok(tasks),
            Err(e) => {
                error!("Error fetching tasks: {}", e);
                Err(ErrorResponse::from("Error fetching tasks"))
            }
        }
    }

    pub(crate) async fn get_task_by_id(&self, task_id: i64) -> Result<Task, ErrorResponse> {
        let task = self.task_repo.find_by_id(task_id).await;
        Ok(task)
    }

    pub(crate) async fn update_task_by_id(
        &self,
        task_id: i64,
        updated_task: UpdateTaskRequest,
    ) -> Result<Task, ErrorResponse> {
        let task = self.task_repo.update_by_id(task_id, updated_task).await;
        Ok(task)
    }

    pub(crate) async fn delete_task_by_id(&self, task_id: i64) -> Result<String, ErrorResponse> {
        match self.task_repo.delete_by_id(task_id).await {
            Ok(result) => Ok(result),
            Err(e) => Err(ErrorResponse::from(format!(
                "Error deleting tasks, {:?}",
                e
            ))),
        }
    }

    pub(crate) async fn delete_all_tasks(&self) -> Result<String, ErrorResponse> {
        match self.task_repo.delete_all().await {
            Ok(result) => Ok(result),
            Err(e) => Err(ErrorResponse::from(format!(
                "Error deleting all tasks: {}",
                e
            ))),
        }
    }

    pub(crate) async fn get_tasks_by_text(
        &self,
        search_text: &str,
    ) -> Result<Vec<Task>, ErrorResponse> {
        match self.task_repo.search_in_tasks(search_text).await {
            Ok(tasks) => Ok(tasks),
            Err(e) => Err(ErrorResponse::from(format!("Error searching tasks: {}", e))),
        }
    }

    pub(crate) async fn search_and_update(
        &self,
        search_term: &str,
        updated_task: UpdateTaskRequest,
    ) -> Result<Vec<Task>, ErrorResponse> {
        match self.task_repo.search_and_update_in_tasks(search_term, updated_task).await { 
            Ok(tasks) => Ok(tasks),
            Err(e) => {
                Err(ErrorResponse::from(format!("Error updating tasks: {}", e)))
            },
        }
    }
    
    pub(crate) async fn search_and_delete(
        &self,
        search_term: &str,
    ) -> Result<String, ErrorResponse> {
        match self.task_repo.delete_by_search_text(search_term).await { 
            Ok(result) => Ok(result),
            Err(e) => Err(ErrorResponse::from(format!("Error deleting tasks: {}", e))),
        }
    }
}
