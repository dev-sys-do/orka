use crate::types::workload_request::WorkloadRequest;
use crate::{
    client::{
        scheduler::{
            workload::{Resources, Type},
            SchedulingRequest, Workload,
        },
        Client,
    },
    errors::ApiError,
};
use axum::Json;
use serde_json::{self, json, Value};
use validator::Validate;

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
    // We spawn a thread to handle the request
    let mut client = Client::new().await?;
    // Create a new Workload Request object out of the body
    let json_body: WorkloadRequest = serde_json::from_str(&body)?;

    // Validate if the workload request is valid
    json_body.validate()?;

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
        resource_limits: Some(Resources {
            cpu: Some(1_i32),
            memory: Some(1_i32),
            disk: Some(1_i32),
        }),
    };

    let request = SchedulingRequest {
        workload: Some(workload),
    };

    let response = client.schedule_workload(request).await.unwrap();

    println!("RESPONSE={:?}", response);
    // TODO: Handle the grpc response and if OK save data and send response to cli
    Ok(Json(json!({"description": "Created"})))
}
