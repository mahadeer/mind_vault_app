use crate::models::{AppDatabase, DbError};
use futures_util::TryStreamExt;
use mindvault_shared::models::tasks_model::Task;
use mongodb::Collection;
use mongodb::bson::doc;

pub struct TaskRepository {
    collection: Collection<Task>,
}

impl TaskRepository {
    pub fn new(app_database: AppDatabase) -> Self {
        let collection = app_database.collection::<Task>("tasks");
        Self { collection }
    }

    pub async fn find_all(&self) -> Result<Vec<Task>, DbError> {
        let query = doc! {
            "$or": [
                { "deleted": { "$ne": true } },
                { "deleted": { "$exists": false } }
            ]
        };

        self.collection
            .find(query)
            .await?
            .try_collect()
            .await
            .map_err(Into::into)
    }
}
