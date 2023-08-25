use crate::store::kv_manager::*;

use crate::types::workload_request::WorkloadRequest;
use crate::{
    client::{
        scheduler::{
            workload::Type,
            SchedulingRequest, Workload,
        },
        Client,
    },
    errors::ApiError,
};
use axum::Json;
use log::info;
use serde_json::{self, json, Value};

use uuid::Uuid;

use validator::Validate;
use crate::client::scheduler::workload;

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

    // We spawn a thread to handle the request
    let mut client = Client::new().await?;

    // Init the database
    let db = KeyValueStore::new().unwrap();

    // Create a new Workload Request object out of the body
    let json_body: WorkloadRequest = serde_json::from_str(&body)?;

    // Validate if the workload request is valid
    json_body.validate()?;

    // Store the workload request in the database

    // TODO: Check if the workload exists in the database

    // let value = db.get_value(id_with_prefix.as_str());
    // let response = format!("{}", value.unwrap());
    // info!("Retrieved value: {:?}", response);

    db.post_workload_value(id_with_prefix.as_str(), json_body.clone())
        .unwrap();
    // Extract the env variable table
    let mut environment = Vec::new();
    if !json_body.workload.environment.is_empty() {
        for env in json_body.workload.environment.iter() {
            environment.push(env.clone());
        }
    }

    // Create a grpc workload object
    let workload = Workload {
        name: json_body.workload.name,
        r#type: Type::Container.into(),
        image: json_body.workload.image,
        environment,
        resource_limits: Some(workload::Resources::default()),
    };

    // TODO: Remove schedule request to the post_instance method
    let request = SchedulingRequest {
        workload: Some(workload),
    };

    let mut response_client = client.schedule_workload(request).await.unwrap();

    // Spawn a thread to handle the stream response
    tokio::spawn(async move {
        while let Some(status) = response_client.message().await.unwrap() {
            info!("Received status: {:?}", status);
        }
    });

    Ok(Json(
        json!({"Your workloak is successfully created ": id_with_prefix}),
    ))
}
