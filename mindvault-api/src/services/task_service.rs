use axum::response::ErrorResponse;
use mindvault_core::models::AppDatabase;
use mindvault_core::repository::task_repo::TaskRepository;
use mindvault_shared::dtos::task_dtos::{
    BulkCreateTaskRequest, CreateTaskRequest, SearchAndUpdateRequest, TaskSearchParams, UpdateTaskRequest,
};
use mindvault_shared::models::tasks_model::TaskResponse;
use tracing::error;

pub(crate) struct TaskService {
    task_repository: TaskRepository,
}

impl TaskService {
    pub(crate) fn new(app_database: AppDatabase) -> Self {
        let task_repository = TaskRepository::new(app_database.clone());
        Self { task_repository }
    }

    pub(crate) async fn create_task(
        &self,
        new_task: CreateTaskRequest,
    ) -> Result<TaskResponse, ErrorResponse> {
        let created_task = self.task_repository.create_task(new_task).await;
        match created_task {
            Ok(task) => Ok(TaskResponse::from(task)),
            Err(e) => {
                let error_message = format!("Error creating task: {:?}", e);
                error!("{}", error_message);
                Err(ErrorResponse::from(error_message))
            }
        }
    }

    pub(crate) async fn bulk_create_tasks(
        &self,
        bulk_request: BulkCreateTaskRequest,
    ) -> Result<Vec<TaskResponse>, ErrorResponse> {
        let created_tasks = self.task_repository.bulk_create_tasks(bulk_request).await;
        match created_tasks {
            Ok(tasks) => Ok(TaskResponse::from_vec(tasks)),
            Err(e) => {
                let error_message = format!("Error bulk creating tasks: {:?}", e);
                error!("{}", error_message);
                Err(ErrorResponse::from(error_message))
            }
        }
    }

    pub(crate) async fn get_all_tasks(&self) -> Result<Vec<TaskResponse>, ErrorResponse> {
        let tasks = self.task_repository.find_all().await;
        match tasks {
            Ok(tasks) => Ok(TaskResponse::from_vec(tasks)),
            Err(e) => {
                let error_message = format!("Error finding tasks: {}", e);
                error!(error_message);
                Err(ErrorResponse::from(error_message))
            }
        }
    }

    pub(crate) async fn get_by_id(&self, task_id: i64) -> Result<TaskResponse, ErrorResponse> {
        let task = self.task_repository.find_by_id(task_id).await;
        match task {
            Ok(Some(task)) => Ok(TaskResponse::from(task)),
            Ok(None) => {
                let error_message = format!("Task with id {} not found", task_id);
                error!("{}", error_message);
                Err(ErrorResponse::from(error_message))
            }
            Err(e) => {
                let error_message = format!("Error finding task: {}", e);
                error!("{}", error_message);
                Err(ErrorResponse::from(error_message))
            }
        }
    }

    pub(crate) async fn search_tasks(
        &self,
        params: TaskSearchParams,
    ) -> Result<Vec<TaskResponse>, ErrorResponse> {
        let tasks = self.task_repository.search_task(params).await;
        match tasks {
            Ok(tasks) => Ok(TaskResponse::from_vec(tasks)),
            Err(e) => {
                let error_message = format!("Error searching tasks: {}", e);
                error!(error_message);
                Err(ErrorResponse::from(error_message))
            }
        }
    }

    pub(crate) async fn soft_delete_task(&self, id: i64) -> Result<bool, ErrorResponse> {
        let result = self.task_repository.soft_delete_by_id(id).await;
        match result {
            Ok(deleted) => Ok(deleted),
            Err(e) => {
                let error_message = format!("Error deleting task: {:?}", e);
                error!("{}", error_message);
                Err(ErrorResponse::from(error_message))
            }
        }
    }

    pub(crate) async fn update_task(
        &self,
        id: i64,
        update_request: UpdateTaskRequest,
    ) -> Result<Option<TaskResponse>, ErrorResponse> {
        let result = self.task_repository.update_task_by_id(id, update_request).await;
        match result {
            Ok(Some(task)) => Ok(Some(TaskResponse::from(task))),
            Ok(None) => Ok(None),
            Err(e) => {
                let error_message = format!("Error updating task: {:?}", e);
                error!("{}", error_message);
                Err(ErrorResponse::from(error_message))
            }
        }
    }

    pub(crate) async fn bulk_delete_by_status(
        &self,
        status: mindvault_shared::models::tasks_model::ETaskStatus,
    ) -> Result<u64, ErrorResponse> {
        let result = self.task_repository.bulk_soft_delete_by_status(status).await;
        match result {
            Ok(count) => Ok(count),
            Err(e) => {
                let error_message = format!("Error bulk deleting tasks: {:?}", e);
                error!("{}", error_message);
                Err(ErrorResponse::from(error_message))
            }
        }
    }

    pub(crate) async fn search_and_update_tasks(
        &self,
        request: SearchAndUpdateRequest,
    ) -> Result<Vec<TaskResponse>, ErrorResponse> {
        let result = self.task_repository.search_and_update_tasks(request).await;
        match result {
            Ok(tasks) => Ok(TaskResponse::from_vec(tasks)),
            Err(e) => {
                let error_message = format!("Error searching and updating tasks: {:?}", e);
                error!("{}", error_message);
                Err(ErrorResponse::from(error_message))
            }
        }
    }
}
