use crate::client::Client;
use crate::client::scheduler::{Workload, SchedulingRequest};
use crate::errors::ApiError;
use crate::store::kv_manager::{KeyValueStore, DB_BATCH};
use crate::types::instance_request::InstanceRequest;
use axum::Json;
use axum::extract::Path;
use serde_json::{self, json, Value};
use validator::Validate;
use log::{trace, error};
use crate::types::instance_status::InstanceStatus;

pub async fn get_instances(_body: String) -> anyhow::Result<Json<Value>, ApiError> {
    let kv_store = KeyValueStore::new()?;
    let instance_list = kv_store.select_instances()?;
    Ok(Json(json!({"instances": instance_list})))
    
}

pub async fn get_specific_instance(Path(id): Path<String>) -> anyhow::Result<Json<Value>, ApiError> {
    let kv_store = KeyValueStore::new()?;
    let instance = kv_store.instances_bucket()?.get(&id)?;
    match instance {
        None => Ok(Json(json!({"description": "Instance not found"}))),
        Some(instance_status) =>Ok(Json(json!({"description": instance_status.as_ref()})))
    }
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

    let kv_store = KeyValueStore::new()?;
    let workload_request = kv_store.workloads_bucket()?.get(&json_body.workload_id)?;

    match workload_request {
        None => Ok(Json(json!({"description": "Workload not found"}))),
        Some(json_request) => {
            let wr = json_request.as_ref().clone();

            // Create a grpc workload object
            let workload = Workload::from(wr.workload.clone());

            // We spawn a thread to handle the request
            let mut client = Client::new().await?;

            let request = SchedulingRequest {
                workload: Some(workload),
            };
        
            let mut stream = client.schedule_workload(request).await?;

            tokio::spawn(async move {
                while let Some(status) = stream.message().await.unwrap() {
                    let id_with_prefix = format!("instance-{}-{}", wr.workload.name, &status.instance_id);
                    trace!("STATUS={:?}", status);
                    let result = DB_BATCH.lock().unwrap().set(&id_with_prefix, &kv::Json(InstanceStatus::from(&status)));
                    match result {
                        Ok(()) => {},
                        Err(e) => error!("{}", e)
                    }
                }
            });

            Ok(Json(json!({"description": "Instance creation started"})))
        },
    }
    
}
