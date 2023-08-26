use std::sync::Arc;

use crate::client::Client;
use crate::client::scheduler::{Workload, SchedulingRequest};
use crate::errors::ApiError;
use crate::types::instance_request::InstanceRequest;
use axum::Json;
use serde_json::{self, json, Value};
use validator::Validate;
use log::trace;
use crate::dbstore::{DB_BATCH, STORE};
use crate::types::instance_status::InstanceStatus;

pub async fn get_instances(_body: String) -> anyhow::Result<Json<Value>, ApiError> {
    let db_store = Arc::clone(&STORE);
    let instances = db_store.lock().unwrap().instances_bucket.iter();
    let mut instance_list = Vec::new();
    // TODO: Implement => retrieve A JSON array of instances ids
    for item in instances {
        let item = item.unwrap();
        let key: String = item.key().unwrap();
        println!("key: {}", key);
        instance_list.push(key);
    }
    Ok(Json(json!({"instances": instance_list})))
}

pub async fn get_specific_instance(_body: String) -> anyhow::Result<Json<Value>, ApiError> {
    // Create a new Instance Request object out of the body
    let json_body: InstanceRequest = serde_json::from_str(&_body)?;

    // Validate the request
    json_body.validate()?;

    let db_store = Arc::clone(&STORE);
    let instance = db_store.lock().unwrap().instances_bucket.get(&json_body.workload_id).unwrap().unwrap();
    Ok(Json(json!({"description": instance.as_ref()})))
}

pub async fn delete_instance(_body: String) -> anyhow::Result<Json<Value>, ApiError> {
    tokio::spawn(async move {
        // TODO: Implement => remove instance from hashmap and stops it using it's id
    });
    Ok(Json(json!({"description": "Deleted"})))
}

pub async fn post_instance(body: String) -> anyhow::Result<Json<Value>, ApiError> {
    // Create a new Instance Request object out of the body
    let json_body: InstanceRequest = serde_json::from_str(&body)?;

    // Validate the request
    json_body.validate()?;

    let db_store = Arc::clone(&STORE);
    let workload_request = db_store.lock().unwrap().workloads_bucket.get(&json_body.workload_id).unwrap();
    match workload_request {
        None => Ok(Json(json!({"description": "Workload not found"}))),
        Some(json_request) => {
            let wr = json_request.as_ref();

            // Create a grpc workload object
            let workload = Workload::from(wr.workload.clone());

            // We spawn a thread to handle the request
            let mut client = Client::new().await?;

            let request = SchedulingRequest {
                workload: Some(workload),
            };
        
            let mut stream = client.schedule_workload(request).await.unwrap();

            while let Some(status) = stream.message().await.unwrap() {
                trace!("STATUS={:?}", status);
                DB_BATCH.lock().unwrap().set(&status.name, &kv::Json(InstanceStatus::from(&status))).unwrap();
            }

            Ok(Json(json!({"description": "Instance created"})))
        },
    }
    
}
