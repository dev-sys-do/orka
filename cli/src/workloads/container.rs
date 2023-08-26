use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::workloads::file::{Kind};
use validator::{Validate, ValidationError};

#[derive(Serialize, Deserialize, Debug)]
enum Registry {
    #[serde(rename(deserialize = "ghcr", serialize = "Ghcr"))]
    Ghcr,
    #[serde(rename(deserialize = "docker", serialize = "Docker"))]
    Docker,
}

impl Registry {
    fn default() -> Self { Registry::Docker }
}


#[derive(Serialize, Deserialize, Validate)]
pub struct WorkloadContainerFile {
    #[validate(length(min = 1))]
    version: String,
    workload: Container
}


#[derive(Serialize, Deserialize, Validate)]
struct Container {
    kind: Kind,
    port: String,
    #[validate(length(min = 1))]
    name: String,
    #[serde(default)]
    environment: Vec<HashMap<String, String>>,
    #[serde(default)]
    network: Vec<String>,
    #[serde(default = "Registry::default")]
    registry: Registry,
    #[validate(length(min = 1))]
    image: String
}


