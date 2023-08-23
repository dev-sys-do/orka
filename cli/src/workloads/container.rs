use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::workloads::file::{Kind};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Registry {
    Ghcr,
    Dockerhub,
}


#[derive(Serialize, Deserialize)]
pub struct WorkloadContainerFile {
    version: u32,
    workload: Container
}


#[derive(Serialize, Deserialize)]
struct Container {
    kind: Kind,
    name: String,
    environment: Vec<HashMap<String, String>>,
    networks: Vec<String>,
    registry: Registry,
    image: String
}