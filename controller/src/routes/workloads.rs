use crate::store::kv_manager::*;
use crate::types::workload_request::WorkloadRequest;
use crate::errors::ApiError;
use axum::{Json, extract::Path};
use serde_json::{self, json, Value};

use uuid::Uuid;

use validator::Validate;

pub async fn get_workloads(_body: String) -> anyhow::Result<Json<Value>, ApiError> {
    // Init the database
    let kv_store = KeyValueStore::new()?;

    let workloads = kv_store.select_workloads()?;
    
    Ok(Json(json!({"workloads": workloads})))
}

pub async fn get_specific_workload(Path(id): Path<String>) -> anyhow::Result<Json<Value>, ApiError> {
    let kv_store = KeyValueStore::new()?;

    let workload = kv_store.workloads_bucket()?.get(&id)?;

    match workload {
        None => Ok(Json(json!({"description": "Workload not found"}))),
        Some(workload_description) =>Ok(Json(json!({"description": workload_description.as_ref()})))
    }
}

pub async fn delete_workload(Path(id): Path<String>) -> anyhow::Result<Json<Value>, ApiError> {
    let kv_store = KeyValueStore::new()?;
    kv_store.workloads_bucket()?.remove(&id)?;
    Ok(Json(json!({"description": "Workload deleted"})))
}

pub async fn post_workload(body: String) -> anyhow::Result<Json<Value>, ApiError> {
    // Init the database
    let kv_store = KeyValueStore::new()?;

    // Create a new Workload Request object out of the body
    let json_body: WorkloadRequest = serde_json::from_str(&body)?;

    // Validate if the workload request is valid
    json_body.validate()?;

    // Generate a new uuid
    let id_with_prefix = format!("workload-{}-{}", json_body.workload.name, Uuid::new_v4());

    // Store the workload request in the database
    kv_store.workloads_bucket()?.set(&id_with_prefix, &kv::Json(json_body))?;

    Ok(Json(
        json!({"Your workload is successfully created ": id_with_prefix}),
    ))
}
