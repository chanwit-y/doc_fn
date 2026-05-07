use reqwest::Error;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::core::http::ApiClient;

use super::client;

#[derive(Debug, Serialize)]
pub struct StatementRequest {
    pub warehouse_id: String,
    pub statement: String,
    pub wait_timeout: String,
}

#[derive(Debug, Deserialize)]
pub struct Warehouse {
    pub id: Option<String>,
    pub name: Option<String>,
    pub cluster_size: Option<String>,
    pub min_num_clusters: Option<u32>,
    pub max_num_clusters: Option<u32>,
    pub auto_stop_mins: Option<u32>,
    pub creator_name: Option<String>,
    pub state: Option<String>,
    pub warehouse_type: Option<String>,
    pub enable_serverless_compute: Option<bool>,
    pub enable_photon: Option<bool>,
    pub num_clusters: Option<u32>,
    pub num_active_sessions: Option<u32>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Deserialize)]
pub struct WarehousesResponse {
    pub warehouses: Option<Vec<Warehouse>>,
}

#[derive(Debug, Deserialize)]
pub struct StatementResponse {
    pub statement_id: Option<String>,
    pub status: Option<Value>,
    pub manifest: Option<Value>,
    pub result: Option<Value>,
}

impl StatementResponse {
    /// Combine `manifest.schema.columns` with `result.data_array` to produce a
    /// JSON array of row objects, e.g.
    /// `[{"carat":"0.23","cut":"Ideal","color":"J","clarity":"SI2"}, ...]`.
    pub fn rows_as_json(&self) -> Value {
        let columns: Vec<String> = self
            .manifest
            .as_ref()
            .and_then(|m| m.get("schema"))
            .and_then(|s| s.get("columns"))
            .and_then(|c| c.as_array())
            .map(|cols| {
                cols.iter()
                    .filter_map(|c| c.get("name").and_then(Value::as_str).map(String::from))
                    .collect()
            })
            .unwrap_or_default();

        let rows = self
            .result
            .as_ref()
            .and_then(|r| r.get("data_array"))
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .filter_map(|row| row.as_array().cloned())
            .map(|row| {
                let mut obj = Map::new();
                for (i, value) in row.into_iter().enumerate() {
                    let key = columns
                        .get(i)
                        .cloned()
                        .unwrap_or_else(|| format!("col_{i}"));
                    obj.insert(key, value);
                }
                Value::Object(obj)
            })
            .collect();

        Value::Array(rows)
    }
}

pub async fn execute_statement(
    client: &ApiClient,
    statement: &str,
    warehouse_id: &str,
    wait_timeout: &str,
) -> Result<StatementResponse, Error> {
    let body = StatementRequest {
        warehouse_id: warehouse_id.to_string(),
        statement: statement.to_string(),
        wait_timeout: wait_timeout.to_string(),
    };
    client.post("/api/2.0/sql/statements/", &body).await
}

/// Execute an arbitrary SQL statement against the configured Databricks
/// warehouse and return the rows as a JSON array of objects.
pub async fn exec(statement: &str) -> Result<Value, Error> {
    let (api, cfg) = client();
    let resp = execute_statement(
        &api,
        statement,
        &cfg.warehouse_id,
        &cfg.wait_timeout,
    )
    .await?;
    Ok(resp.rows_as_json())
}

/// Fetch the list of SQL warehouses from Databricks.
pub async fn list_warehouses(client: &ApiClient) -> Result<WarehousesResponse, Error> {
    client.get("/api/2.0/sql/warehouses").await
}

/// Convenience wrapper that builds the configured Databricks client and
/// returns all warehouses for the workspace.
pub async fn get_warehouses() -> Result<Vec<Warehouse>, Error> {
    let (api, _cfg) = client();
    let resp = list_warehouses(&api).await?;
    Ok(resp.warehouses.unwrap_or_default())
}
