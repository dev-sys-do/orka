use std::sync::Arc;

use crate::client::Client;
use crate::errors::ApiError;
use crate::store::kv_manager::{DB_BATCH, DB_STORE};
use crate::types::instance_request::InstanceRequest;
use crate::types::instance_status::InstanceStatus;
use axum::extract::Path;
use axum::Json;
use log::{trace, warn, error};
use orka_proto::scheduler_controller::{SchedulingRequest, Workload, WorkloadInstance};
use serde_json::{self, json, Value};
use validator::Validate;

pub async fn get_instances(_body: String) -> anyhow::Result<Json<Value>, ApiError> {
    let kv_store = DB_STORE.lock().unwrap();
    let instance_list = kv_store.select_instances()?;
    Ok(Json(json!({ "instances": instance_list })))
}

pub async fn get_specific_instance(
    Path(id): Path<String>,
) -> anyhow::Result<Json<Value>, ApiError> {
    let kv_store = DB_STORE.lock().unwrap();
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

    let kv_store = DB_STORE.lock().unwrap();

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

    let kv_store = DB_STORE.lock().unwrap();

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

    let kv_store = Arc::clone(&DB_STORE);
    let workload_request = kv_store
        .lock()
        .unwrap()
        .workloads_bucket()?
        .get(&json_body.workload_id)?;
    drop(kv_store);

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

            stream.message().await.map_err(|e| {
                warn!("Error while creating the instance: {:?}", e);
                ApiError::InstanceNotCreated {
                    message: format!("Instance not created {:?}", e),
                }
            })?;

            tokio::spawn(async move {
                while let Ok(Some(status)) = stream.message().await {
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
