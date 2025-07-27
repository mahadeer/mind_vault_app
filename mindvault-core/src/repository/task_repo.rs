use crate::models::{AppDatabase, DbCollection, DbError};
use crate::repository::auto_increment::{
    get_next_id_for_collection, get_next_id_range_for_collection, AUTO_INCREMENT_COLLECTION_NAME,
};
use bson::{DateTime as BsonDateTime, Document};
use chrono::{Duration, NaiveDate, TimeZone, Utc};
use futures_util::TryStreamExt;
use mindvault_shared::dtos::task_dtos::{
    BulkCreateTaskRequest, CreateTaskRequest, SearchAndUpdateRequest, TaskSearchParams,
    UpdateTaskRequest,
};
use mindvault_shared::models::tasks_model::{ETaskStatus, Task};
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

    /// Converts an optional NaiveDate to an optional BsonDateTime at midnight UTC
    fn convert_due_date(due_date: Option<NaiveDate>) -> Option<BsonDateTime> {
        due_date.map(|date: NaiveDate| {
            let naive_dt = date.and_hms_opt(0, 0, 0).unwrap();
            let utc_dt = Utc.from_utc_datetime(&naive_dt);
            BsonDateTime::from(utc_dt)
        })
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
        let query = doc! {
            "_id": id,
            "$or": [
                { "deleted": { "$ne": true } },
                { "deleted": { "$exists": false } }
            ]
        };
        self.collection.find_one(query).await.map_err(Into::into)
    }

    pub async fn create_task(&self, new_task: CreateTaskRequest) -> Result<Task, DbError> {
        // Get the next task id via your counters' collection
        let next_task_id =
            get_next_id_for_collection(&self.counters_collection, COLLECTION_NAME).await?;

        // Convert Option<NaiveDate> to Option<BsonDateTime> at midnight UTC
        let due_date = Self::convert_due_date(new_task.due_date);

        // Use current UTC time directly for created_at
        let created_at = BsonDateTime::now();

        let task = Task {
            id: next_task_id,
            name: new_task.name,
            priority: new_task.priority.unwrap_or_default(),
            status: new_task.status.unwrap_or_default(),
            due_date,
            created_at,
            deleted: Some(false),
        };

        info!("{:?}", task);

        match self.collection.insert_one(&task).await {
            Ok(_) => Ok(task),
            Err(e) => Err(e.into()),
        }
    }

    pub async fn bulk_create_tasks(
        &self,
        bulk_request: BulkCreateTaskRequest,
    ) -> Result<Vec<Task>, DbError> {
        if bulk_request.tasks.is_empty() {
            return Ok(Vec::new());
        }

        let task_count = bulk_request.tasks.len() as i64;

        // Get the starting ID for the batch using the centralized auto_increment function
        let start_id = get_next_id_range_for_collection(
            &self.counters_collection,
            COLLECTION_NAME,
            task_count,
        )
        .await?;

        let created_at = BsonDateTime::now();
        let mut tasks_to_insert = Vec::new();

        for (index, new_task) in bulk_request.tasks.into_iter().enumerate() {
            // Convert Option<NaiveDate> to Option<BsonDateTime> at midnight UTC
            let due_date = Self::convert_due_date(new_task.due_date);

            let task = Task {
                id: start_id + index as i64,
                name: new_task.name,
                priority: new_task.priority.unwrap_or_default(),
                status: new_task.status.unwrap_or_default(),
                due_date,
                created_at,
                deleted: Some(false),
            };

            tasks_to_insert.push(task);
        }

        info!("Bulk inserting {} tasks", tasks_to_insert.len());

        match self.collection.insert_many(&tasks_to_insert).await {
            Ok(_) => Ok(tasks_to_insert),
            Err(e) => Err(e.into()),
        }
    }

    pub async fn search_task(&self, params: TaskSearchParams) -> Result<Vec<Task>, DbError> {
        let mut query = doc! {};

        // Add deleted filter
        let deleted_filter = doc! {
            "$or": [
                { "deleted": { "$ne": true } },
                { "deleted": { "$exists": false } }
            ]
        };

        let mut conditions = vec![deleted_filter];

        if let Some(search_term) = params.query {
            let search_condition = doc! { "name": { "$regex": &search_term , "$options": "i" } };
            conditions.push(search_condition);
        }
        if let Some(status_query) = params.status {
            conditions.push(doc! { "status": bson::to_bson(&status_query)? });
        }
        if let Some(priority_query) = params.priority {
            conditions.push(doc! { "priority": bson::to_bson(&priority_query)? });
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

            conditions.push(doc! {
                "dueDate": {
                    "$gte": desired_min_date,
                    "$lt": desired_max_date
                }
            });
        }

        query.insert("$and", conditions);

        self.collection
            .find(query)
            .await?
            .try_collect()
            .await
            .map_err(Into::into)
    }

    /// Softly delete a task by setting deleted: true
    pub async fn soft_delete_by_id(&self, id: i64) -> Result<bool, DbError> {
        let filter = doc! {
            "_id": id,
            "$or": [
                { "deleted": { "$ne": true } },
                { "deleted": { "$exists": false } }
            ]
        };
        let update = doc! { "$set": { "deleted": true } };

        let result = self.collection.update_one(filter, update).await?;
        Ok(result.modified_count > 0)
    }

    /// Update a task by ID with partial updates (only status, due_date, priority)
    pub async fn update_task_by_id(
        &self,
        id: i64,
        update_request: UpdateTaskRequest,
    ) -> Result<Option<Task>, DbError> {
        let filter = doc! {
            "_id": id,
            "$or": [
                { "deleted": { "$ne": true } },
                { "deleted": { "$exists": false } }
            ]
        };

        let mut set_fields = doc! {};

        if let Some(status) = update_request.status {
            set_fields.insert("status", bson::to_bson(&status)?);
        }
        if let Some(priority) = update_request.priority {
            set_fields.insert("priority", bson::to_bson(&priority)?);
        }
        if let Some(due_date) = update_request.due_date {
            let converted_date = Self::convert_due_date(Some(due_date));
            set_fields.insert("due_date", bson::to_bson(&converted_date)?);
        }

        let update = doc! { "$set": set_fields };

        let options = mongodb::options::FindOneAndUpdateOptions::builder()
            .return_document(mongodb::options::ReturnDocument::After)
            .build();

        self.collection
            .find_one_and_update(filter, update)
            .with_options(options)
            .await
            .map_err(Into::into)
    }

    /// Bulk soft delete tasks by status
    pub async fn bulk_soft_delete_by_status(&self, status: ETaskStatus) -> Result<u64, DbError> {
        let filter = doc! {
            "status": bson::to_bson(&status)?,
            "$or": [
                { "deleted": { "$ne": true } },
                { "deleted": { "$exists": false } }
            ]
        };
        let update = doc! { "$set": { "deleted": true } };

        let result = self.collection.update_many(filter, update).await?;
        Ok(result.modified_count)
    }

    /// Search and update tasks based on search criteria
    pub async fn search_and_update_tasks(
        &self,
        request: SearchAndUpdateRequest,
    ) -> Result<Vec<Task>, DbError> {
        // Validate that at least one update field is provided
        if request.status.is_none() && request.due_date.is_none() && request.priority.is_none() {
            return Err(DbError::InvalidId(
                "At least one update field must be provided".to_string(),
            ));
        }

        // Build a search query
        let mut conditions = vec![doc! {
            "$or": [
                { "deleted": { "$ne": true } },
                { "deleted": { "$exists": false } }
            ]
        }];

        if let Some(search_term) = request.query {
            conditions.push(doc! { "name": { "$regex": &search_term, "$options": "i" } });
        }
        if let Some(status_filter) = request.status_filter {
            conditions.push(doc! { "status": bson::to_bson(&status_filter)? });
        }
        if let Some(priority_filter) = request.priority_filter {
            conditions.push(doc! { "priority": bson::to_bson(&priority_filter)? });
        }
        if let Some(due_date_filter) = request.due_date_filter {
            let day_start_naive = due_date_filter.and_hms_opt(0, 0, 0).unwrap();
            let next_day_start_naive = (due_date_filter + Duration::days(1))
                .and_hms_opt(0, 0, 0)
                .unwrap();
            let day_start_utc = Utc.from_utc_datetime(&day_start_naive);
            let next_day_start_utc = Utc.from_utc_datetime(&next_day_start_naive);
            let desired_min_date = BsonDateTime::from(day_start_utc);
            let desired_max_date = BsonDateTime::from(next_day_start_utc);

            conditions.push(doc! {
                "dueDate": {
                    "$gte": desired_min_date,
                    "$lt": desired_max_date
                }
            });
        }

        let filter = doc! { "$and": conditions };

        // Build update document
        let mut set_fields = doc! {};
        if let Some(status) = request.status {
            set_fields.insert("status", bson::to_bson(&status)?);
        }
        if let Some(priority) = request.priority {
            set_fields.insert("priority", bson::to_bson(&priority)?);
        }
        if let Some(due_date) = request.due_date {
            let converted_date = Self::convert_due_date(Some(due_date));
            set_fields.insert("due_date", bson::to_bson(&converted_date)?);
        }

        let update = doc! { "$set": set_fields };

        // Update all matching documents
        self.collection.update_many(filter.clone(), update).await?;

        // Return the updated documents
        self.collection
            .find(filter)
            .await?
            .try_collect()
            .await
            .map_err(Into::into)
    }
}
