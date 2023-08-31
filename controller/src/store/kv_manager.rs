use std::{
    ops::{Deref, DerefMut},
    sync::{Arc, Mutex},
};

use crate::types::{instance_status::InstanceStatus, workload_request::WorkloadRequest};
use kv::*;
use once_cell::sync::Lazy;

pub static DB_BATCH: Lazy<Mutex<KeyValueBatch>> = Lazy::new(|| Mutex::new(KeyValueBatch::new()));

pub struct KeyValueBatch {
    pub batch: Batch<String, Json<InstanceStatus>>,
}

impl KeyValueBatch {
    pub fn new() -> Self {
        Self {
            batch: Batch::new(),
        }
    }
}

impl Default for KeyValueBatch {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for KeyValueBatch {
    type Target = Batch<String, Json<InstanceStatus>>;
    fn deref(&self) -> &Self::Target {
        &self.batch
    }
}

impl DerefMut for KeyValueBatch {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.batch
    }
}

pub static DB_STORE: Lazy<Arc<Mutex<KeyValueStore>>> =
    Lazy::new(|| Arc::new(Mutex::new(KeyValueStore::new())));

pub struct KeyValueStore {
    store: Store,
}

impl KeyValueStore {
    pub fn new() -> Self {
        // Configure the database
        let cfg = Config::new("./db/controller");

        // Open the key/value store
        let store = Store::new(cfg).unwrap();

        Self { store }
    }

    pub fn workloads_bucket(&self) -> Result<Bucket<'_, String, Json<WorkloadRequest>>, Error> {
        self.store
            .bucket::<String, Json<WorkloadRequest>>(Some("workloads"))
    }

    pub fn instances_bucket(&self) -> Result<Bucket<'_, String, Json<InstanceStatus>>, Error> {
        self.store
            .bucket::<String, Json<InstanceStatus>>(Some("instances"))
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

impl Default for KeyValueStore {
    fn default() -> Self {
        Self::new()
    }
}
