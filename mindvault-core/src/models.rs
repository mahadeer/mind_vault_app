use bson::Document;
use mongodb::{bson, Collection, Database};
use thiserror::Error;

pub type AppDatabase = Database;
pub type DbCollection<T> = Collection<T>;
pub type DbDocument = Document;

#[derive(Debug, Error)]
pub enum DbError {
    #[error("MongoDB error: {0}")]
    MongoError(#[from] mongodb::error::Error),
    #[error("BSON deserialization error: {0}")]
    BsonDeError(#[from] bson::de::Error),
    #[error("BSON serialization error: {0}")]
    BsonSerError(#[from] bson::ser::Error),
    #[error("Invalid ID format: {0}")]
    InvalidId(String),
    #[error("Not Found")]
    NotFound,
    #[error("Internal Database Error: {0}")]
    InternalError(String),
}
