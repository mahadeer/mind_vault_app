use mongodb::{bson};
use thiserror::Error;

// Re-export common MongoDB types for convenience
pub use mongodb::Client as MongoDbClient;
pub use mongodb::Database as MongoDatabase; // Renamed to avoid confusion with `db` module

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