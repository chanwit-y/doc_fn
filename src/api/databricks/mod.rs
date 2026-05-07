use std::env;

use dotenvy::dotenv;

use crate::core::http::ApiClient;

pub mod table;

pub struct DatabricksConfig {
    pub base_url: String,
    pub token: String,
    pub warehouse_id: String,
    pub wait_timeout: String,
}

impl DatabricksConfig {
    pub fn from_env() -> Self {
        dotenv().ok();
        Self {
            base_url: env::var("DATABRICKS_BASE_URL")
                .expect("DATABRICKS_BASE_URL must be set in environment"),
            token: env::var("DATABRICKS_TOKEN")
                .expect("DATABRICKS_TOKEN must be set in environment"),
            warehouse_id: env::var("DATABRICKS_WAREHOUSE_ID")
                .expect("DATABRICKS_WAREHOUSE_ID must be set in environment"),
            wait_timeout: env::var("DATABRICKS_WAIT_TIMEOUT")
                .expect("DATABRICKS_WAIT_TIMEOUT must be set in environment"),
        }
    }
}

pub fn client() -> (ApiClient, DatabricksConfig) {
    let cfg = DatabricksConfig::from_env();
    let client = ApiClient::with_api_key(&cfg.base_url, &cfg.token);
    (client, cfg)
}
