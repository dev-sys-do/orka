use crate::types::workload_request::WorkloadRequest;
use kv::*;

pub struct KeyValueStore {
    store: Store,
}

impl KeyValueStore {
    pub fn new() -> Result<Self, Error> {
        // Configure the database
        let cfg = Config::new("./db/controller");

        // Open the key/value store
        let store = Store::new(cfg)?;

        Ok(KeyValueStore { store })
    }

    pub fn post_workload_value(&self, key: &str, workload: WorkloadRequest) -> Result<(), Error> {
        let bucket = self.store.bucket::<&str, Json<WorkloadRequest>>(None)?;
        let key = key;
        let value = Json(workload);
        bucket.set(&key, &value)?;
        Ok(())
    }

    // pub fn get_value(&self, key: &str) -> Result<Json<WorkloadRequest>, Error> {
    //     let bucket = self.store.bucket::<&str, Json<WorkloadRequest>>(None)?;
    //     let value = bucket.get(&key)?.unwrap();
    //     Ok(value)
    // }
}
