use std::sync::{Arc, Mutex};

use crate::types::{workload_request::WorkloadRequest, instance_status::InstanceStatus};
use kv::*;
use once_cell::sync::Lazy;

pub static DB_BATCH: Lazy<Arc<Mutex<Batch<String, Json<InstanceStatus>>>>> = Lazy::new(|| {Arc::new(Mutex::new(Batch::new()))});

pub struct KeyValueStore {
    store: Store,
}

impl KeyValueStore {
    pub fn new() -> Result<Self, Error> {
        // Configure the database
        let cfg = Config::new("./db/controller");

        // Open the key/value store
        let store = Store::new(cfg)?;
        
        Ok(Self { store })
    }

    pub fn workloads_bucket(&self) -> Result<Bucket<'_, String, Json<WorkloadRequest>>, Error> {
        Ok(self.store.bucket::<String, Json<WorkloadRequest>>(Some("workloads"))?)
    }

    pub fn instances_bucket(&self) -> Result<Bucket<'_, String, Json<InstanceStatus>>, Error> {
        Ok(self.store.bucket::<String, Json<InstanceStatus>>(Some("instances"))?)
    }

    // Get an array of workload ids
    pub fn select_workloads(&self) -> Result<Vec<String>, Error> {
        let mut workloads: Vec<String> = Vec::new();
        for workload in self.workloads_bucket()?.iter() {
            workloads.push(workload?.key()?);
        }
        Ok(workloads)
    }

    // Get an array of instance names
    pub fn select_instances(&self) -> Result<Vec<String>, Error> {
        let mut instances: Vec<String> = Vec::new();
        for instance in self.instances_bucket()?.iter() {
            instances.push(instance?.key()?);
        }
        Ok(instances)
    }

}
