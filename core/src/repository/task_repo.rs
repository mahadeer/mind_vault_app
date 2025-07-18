use futures_util::TryStreamExt;
use mongodb::bson::{doc, Bson};
use mongodb::Collection;
use tracing::error;
use shared::models::task_model::{CreateTaskRequest, Task, UpdateTaskRequest};
use crate::models::{AppDatabase, DbError};

#[derive(Debug, Clone)]
pub struct TaskRepo {
    tasks_collection: Collection<Task>,
}

impl TaskRepo {
    pub fn new(app_database: AppDatabase) -> Self {
        let tasks_collection: Collection<Task> = app_database.collection("tasks");
        Self { tasks_collection }
    }

    pub async fn find_all(&self) -> Result<Vec<Task>, DbError> {
        let cursor = self.tasks_collection.find(doc! {}).await?;
        let tasks = cursor.try_collect().await?;
        Ok(tasks)
    }

    pub async fn add_task(&self, created_task: CreateTaskRequest) -> Task {
        let total_docs = self.tasks_collection.count_documents(doc! {}).await;
        let new_task = Task {
            id: (total_docs.unwrap() + 1) as i64,
            name: created_task.name,
            status: created_task.status.unwrap_or("in_progress".parse().unwrap()),
            schedule: created_task.schedule,
            due_date: created_task.due_date,
            description: created_task.description
        };
        let created_task = self.tasks_collection.insert_one(new_task).await;
        let inserted_id = created_task.unwrap().inserted_id;
        let inserted_task_id = doc! { "_id": inserted_id };
        let created_task = self.tasks_collection.find_one(inserted_task_id).await.unwrap();
        created_task.unwrap()
    }

    pub async fn find_by_id(&self, task_id: i64) -> Task {
        let filter = doc! { "_id": task_id };
        let created_task = self.tasks_collection.find_one(filter).await.unwrap();
        created_task.unwrap()
    }

    pub async fn update_by_id(&self, task_id: i64, updated_task: UpdateTaskRequest) -> Task {
        let filter = doc! { "_id": task_id };

        let mut update_doc = doc! {};
        let mut set_fields = doc! {};

        // Populate set_fields as you do in your current update_task
        if let Some(name) = updated_task.name { set_fields.insert("name", Bson::String(name)); }
        if let Some(description) = updated_task.description { set_fields.insert("description", Bson::String(description)); }
        if let Some(status) = updated_task.status { set_fields.insert("status", Bson::String(status)); }
        if let Some(due_date) = updated_task.due_date { set_fields.insert("due_date", Bson::String(due_date)); }
        if let Some(schedule) = updated_task.schedule { set_fields.insert("schedule", Bson::String(schedule)); }

        if !set_fields.is_empty() {
            update_doc.insert("$set", set_fields);
        } else {
            return self.find_by_id(task_id).await;
        }

        let updated_task = self.tasks_collection
            .find_one_and_update(filter, update_doc)
            .await.unwrap();
        updated_task.unwrap()

    }

    pub async fn delete_by_id(&self, task_id: i64) -> Result<String, DbError> {
        let filter = doc! { "_id": task_id };
        match self.tasks_collection.delete_one(filter).await {
            Ok(_) => Ok(format!("Task with id {} deleted", task_id)),
            Err(_) => {
                error!("Task does not exist: {}", task_id);
                Ok(format!("Error deleting with id {}", task_id))
            }
        }
    }

    pub async fn delete_all(&self) -> Result<String, DbError> {
        match self.tasks_collection.drop().await {
            Ok(_) => Ok("All Tasks deleted".to_string()),
            Err(e) => {
                error!("Error dropping collection: {}", e);
                Ok("Error dropping collection".to_string())
            }
        }
    }
}