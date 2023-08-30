use crate::client::scheduler::{SchedulingRequest, Workload, WorkloadInstance};
use crate::client::Client;
use crate::errors::ApiError;
use crate::store::kv_manager::{KeyValueStore, DB_BATCH};
use crate::types::instance_request::InstanceRequest;
use crate::types::instance_status::InstanceStatus;
use axum::extract::Path;
use axum::Json;
use log::{error, trace};
use serde_json::{self, json, Value};
use validator::Validate;

pub async fn get_instances(_body: String) -> anyhow::Result<Json<Value>, ApiError> {
    let kv_store = KeyValueStore::new()?;
    let instance_list = kv_store.select_instances()?;
    Ok(Json(json!({ "instances": instance_list })))
}

pub async fn get_specific_instance(
    Path(id): Path<String>,
) -> anyhow::Result<Json<Value>, ApiError> {
    let kv_store = KeyValueStore::new()?;
    let instance = kv_store.instances_bucket()?.get(&id)?;
    match instance {
        None => Ok(Json(json!({"description": "Instance not found"}))),
        Some(instance_status) => Ok(Json(json!({"description": instance_status.as_ref()}))),
    }
}

pub async fn delete_instance(Path(id): Path<String>) -> anyhow::Result<Json<Value>, ApiError> {
    let mut client = Client::new().await?;

    let instance = WorkloadInstance {
        instance_id: (*id).to_string(),
    };

    client.stop_instance(instance).await?;

    let kv_store = KeyValueStore::new()?;

    let instance = kv_store.instances_bucket()?.remove(&id)?;

    match instance {
        Some(_inst) => Ok(Json(json!({"description": "Deleted"}))),
        None => Ok(Json(json!({"description": "Instance not found"}))),
    }
}

pub async fn delete_instance_force(
    Path(id): Path<String>,
) -> anyhow::Result<Json<Value>, ApiError> {
    let mut client = Client::new().await?;

    let instance = WorkloadInstance {
        instance_id: (*id).to_string(),
    };

    client.destroy_instance(instance).await?;

    let kv_store = KeyValueStore::new()?;

    let instance = kv_store.instances_bucket()?.remove(&id)?;

    match instance {
        Some(_inst) => Ok(Json(json!({"description": "Deleted"}))),
        None => Ok(Json(json!({"description": "Instance not found"}))),
    }
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
            // Create a grpc workload object
            let workload = Workload::from(json_request.0.workload);

            // We spawn a thread to handle the request
            let mut client = Client::new().await?;

            let instance_id = (*workload.instance_id).to_string();

            let request = SchedulingRequest {
                workload: Some(workload),
            };

            let mut stream = client.schedule_workload(request).await?;

            tokio::spawn(async move {
                while let Some(status) = stream.message().await.unwrap() {
                    trace!("STATUS={:?}", status);
                    let result = DB_BATCH.lock().unwrap().batch.set(
                        &status.instance_id,
                        &kv::Json(InstanceStatus::from(&status)),
                    );
                    match result {
                        Ok(()) => {}
                        Err(e) => error!("{}", e),
                    }
                }
            });

            Ok(Json(json!({
                "description": format!("Instance creation started, id: {}", instance_id)
            })))
        }
    }
}
