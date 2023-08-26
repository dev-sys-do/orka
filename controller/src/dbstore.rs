use kv::*;

use crate::{types::{workload_request::WorkloadRequest, instance_status::InstanceStatus}};

use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;


pub struct DbStore<'a> {
    pub workloads_bucket: Bucket<'a, String, Json<WorkloadRequest>>,
    pub instances_bucket: Bucket<'a, String, Json<InstanceStatus>>
}

pub static DB_BATCH: Lazy<Arc<Mutex<Batch<String, Json<InstanceStatus>>>>> = Lazy::new(|| {Arc::new(Mutex::new(Batch::new()))});

// Create KV store
pub static STORE: Lazy<Arc<Mutex<DbStore>>> = Lazy::new(|| {Arc::new(Mutex::new(DbStore::new().unwrap()))});

impl DbStore<'static> {

    pub fn new() -> anyhow::Result<Self, Error> {
        println!("Creating store!");
        let cfg = Config::new("./.db/db_data");

        // Open the key/value store
        let store = Store::new(cfg.temporary(false))?;

        println!("{:?}", store.buckets());
    
        let workloads_bucket = store.bucket::<String, Json<WorkloadRequest>>(Some("workloads"))?;
        let instances_bucket = store.bucket::<String, Json<InstanceStatus>>(Some("instances"))?;

        println!("{:?}", store.buckets());
        Ok(Self { workloads_bucket, instances_bucket })
    }

}