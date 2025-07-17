use std::env;
use crate::config::{MONGODB_URI, MONGO_DB_NAME};

pub struct MongoConfig {
    pub uri: String,
    pub db_name: String,
}

impl MongoConfig {
    pub fn from_env() -> Result<Self, env::VarError> {
        let uri = MONGODB_URI.to_owned();
        let db_name = MONGO_DB_NAME.to_string();
        Ok(MongoConfig { uri, db_name })
    }
}