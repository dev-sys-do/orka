use crate::errors::ApiError;
use crate::types::instance_request::InstanceRequest;
use axum::Json;
use serde_json::{self, json, Value};
use validator::Validate;

pub async fn get_instances(_body: String) -> anyhow::Result<Json<Value>, ApiError> {
    tokio::spawn(async move {
        // TODO: Implement => retrieve A JSON array of instances ids
    });
    Ok(Json(json!({"instances": "[]"})))
}

pub async fn get_specific_instance(_body: String) -> anyhow::Result<Json<Value>, ApiError> {
    tokio::spawn(async move {
        // TODO: Implement => retrieve the instance needed from hashmap
    });
    Ok(Json(json!({"description": "A instance description file"})))
}

pub async fn delete_instance(_body: String) -> anyhow::Result<Json<Value>, ApiError> {
    tokio::spawn(async move {
        // TODO: Implement => remove instance from hashmap and stops it using it's id
    });
    Ok(Json(json!({"description": "Deleted"})))
}

pub async fn post_instance(body: String) -> anyhow::Result<Json<Value>, ApiError> {
    // Create a new Workload Request object out of the body
    let json_body: InstanceRequest = serde_json::from_str(&body)?;

    // Validate the request
    json_body.validate()?;
    // TODO: Creates an instance based on an existing workload id
    Ok(Json(json!({"description": "Instance created"})))
}
