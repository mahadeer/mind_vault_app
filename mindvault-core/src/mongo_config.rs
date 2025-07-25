use std::env;

const MONGODB_URI: &str = "mongodb://localhost:27017";
const MONGO_DB_NAME: &str = "mind_vault_v1";

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