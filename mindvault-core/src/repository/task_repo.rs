use crate::models::{AppDatabase, DbCollection, DbError};
use crate::repository::auto_increment::{
    get_next_id_for_collection, AUTO_INCREMENT_COLLECTION_NAME,
};
use bson::{DateTime as BsonDateTime, Document};
use chrono::{Duration, NaiveDate, TimeZone, Utc};
use futures_util::TryStreamExt;
use mindvault_shared::dtos::task_dtos::{CreateTaskRequest, TaskSearchParams};
use mindvault_shared::models::tasks_model::{Task};
use mongodb::bson::doc;
use mongodb::{bson, Collection};
use tracing::info;

pub struct TaskRepository {
    collection: Collection<Task>,
    counters_collection: DbCollection<Document>,
}

const COLLECTION_NAME: &str = "tasks";

impl TaskRepository {
    pub fn new(app_database: AppDatabase) -> Self {
        let collection = app_database.collection::<Task>(COLLECTION_NAME);
        let counters_collection =
            app_database.collection::<Document>(AUTO_INCREMENT_COLLECTION_NAME);
        Self {
            collection,
            counters_collection,
        }
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

    pub async fn find_by_id(&self, id: i64) -> Result<Option<Task>, DbError> {
        let query = doc! {"_id": id};
        self.collection.find_one(query).await.map_err(Into::into)
    }

    pub async fn create_task(&self, new_task: CreateTaskRequest) -> Result<Task, DbError> {
        // Get the next task id via your counters' collection
        let next_task_id = get_next_id_for_collection(&self.counters_collection, COLLECTION_NAME).await?;

        // Convert Option<NaiveDate> to Option<BsonDateTime> at midnight UTC
        let due_date = new_task.due_date.map(|date: NaiveDate| {
            let naive_dt = date.and_hms_opt(0, 0, 0).unwrap();
            let utc_dt = Utc.from_utc_datetime(&naive_dt);
            BsonDateTime::from(utc_dt)
        });

        // Use current UTC time directly for created_at
        let created_at = BsonDateTime::now();

        let task = Task {
            id: next_task_id,
            name: new_task.name,
            priority: new_task.priority.unwrap_or_default(),
            status: new_task.status.unwrap_or_default(),
            due_date,
            created_at,
        };

        info!("{:?}", task);

        match self.collection.insert_one(&task).await {
            Ok(_) => Ok(task),
            Err(e) => Err(e.into()),
        }
    }

    pub async fn search_task(&self, params: TaskSearchParams) -> Result<Vec<Task>, DbError> {
        let mut query = doc! {};
        if let Some(search_term) = params.query {
            let or_condition = vec![doc! { "name": { "$regex": &search_term , "$options": "i" } }];
            query.insert("$or", or_condition);
        }
        if let Some(status_query) = params.status {
            query.insert("status", bson::to_bson(&status_query)?);
        }
        if let Some(priority_query) = params.priority {
            query.insert("priority", bson::to_bson(&priority_query)?);
        }
        if let Some(due_date_query) = params.due_date {
            // Construct NaiveDateTime bounds for the same day
            let day_start_naive = due_date_query.and_hms_opt(0, 0, 0).unwrap();
            let next_day_start_naive = (due_date_query + Duration::days(1))
                .and_hms_opt(0, 0, 0)
                .unwrap();
            let day_start_utc = Utc.from_utc_datetime(&day_start_naive);
            let next_day_start_utc = Utc.from_utc_datetime(&next_day_start_naive);
            let desired_min_date = BsonDateTime::from(day_start_utc);
            let desired_max_date = BsonDateTime::from(next_day_start_utc);

            query.insert(
                "dueDate",
                doc! {
                    "$gte": desired_min_date,
                    "$lt": desired_max_date
                },
            );
        }
        self.collection
            .find(query)
            .await?
            .try_collect()
            .await
            .map_err(Into::into)
    }
}
