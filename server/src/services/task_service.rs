use axum::response::ErrorResponse;
use axum::Router;
use core::models::DbClient;
use shared::models::task_model::{CreateTaskRequest, Task};

#[derive(Clone, Debug)]
pub(crate) struct TaskService {
    db_client: DbClient,
}

impl TaskService {
    pub(crate) fn new(db_client: DbClient) -> TaskService {
        TaskService {
            db_client
        }
    }

    pub(crate) fn create_task(&self, new_task: CreateTaskRequest) -> Result<Task, ErrorResponse> {
     todo!()
    }

    pub(crate) fn get_all_tasks(&self) -> Result<Vec<Task>, ErrorResponse> {
        todo!()
    }

    pub(crate) fn get_task_by_id(&self, task_id: i64) -> Result<Task, ErrorResponse> {
        todo!()
    }
}
