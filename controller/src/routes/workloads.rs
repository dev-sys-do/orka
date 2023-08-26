use crate::store::kv_manager::*;
use crate::types::workload_request::WorkloadRequest;
use crate::errors::ApiError;
use axum::Json;
use serde_json::{self, json, Value};

use uuid::Uuid;

use validator::Validate;

use crate::dbstore::STORE;

pub async fn get_workloads(_body: String) -> anyhow::Result<Json<Value>, ApiError> {
    tokio::spawn(async move {
        // TODO: Implement => retrieve list of workloads from hashmap
    });
    Ok(Json(json!({"workloads": "[]"})))
}

pub async fn get_specific_workload(_body: String) -> anyhow::Result<Json<Value>, ApiError> {
    tokio::spawn(async move {
        // TODO: Implement => retrieve the workload needed from hashmap
    });
    Ok(Json(json!({"description": "A workload description file"})))
}

pub async fn delete_workload(_body: String) -> anyhow::Result<Json<Value>, ApiError> {
    tokio::spawn(async move {
        // TODO: Implement => remove workload from hashmap
    });
    Ok(Json(json!({"description": "Deleted"})))
}

pub async fn post_workload(body: String) -> anyhow::Result<Json<Value>, ApiError> {
    // Generate a new uuid
    let id = Uuid::new_v4();
    let id_with_prefix = format!("workload-{}", id);

    // Init the database
    let db = KeyValueStore::new().unwrap();

    // Create a new Workload Request object out of the body
    let json_body: WorkloadRequest = serde_json::from_str(&body)?;

    // Validate if the workload request is valid
    json_body.validate()?;

    // Store the workload request in the database

    db.post_workload_value(id_with_prefix.as_str(), json_body.clone())
        .unwrap();

    STORE.lock().unwrap().workloads_bucket.set(&String::from("test"), &kv::Json(json_body.clone()));

    Ok(Json(
        json!({"Your workload is successfully created ": id_with_prefix}),
    ))
}
