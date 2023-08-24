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
    version: String,
    workload: Container
}


#[derive(Serialize, Deserialize)]
struct Container {
    kind: Kind,
    name: String,
    #[serde(default)]
    environment: Vec<HashMap<String, String>>,
    #[serde(default)]
    networks: Vec<String>,
    #[serde(default = "defaultRegistry")]
    registry: Registry,
    image: String
}

fn defaultRegistry() -> Registry {
    return Registry.Dockerhub
}