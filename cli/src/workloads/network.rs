use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::workloads::file::{Kind};


#[derive(Serialize, Deserialize)]
pub struct WorkloadNetworkFile {
    version: u32,
    workload: Network
}


#[derive(Serialize, Deserialize)]
struct Network {
    kind: Kind,
    name: String,
    #[serde(rename = "allowService")]
    allowservice: Vec<String>,
    egress: Vec<HashMap<String, IpAdress>>,
    ingress: Vec<HashMap<String, IpAdress>>,
}


#[derive(Serialize, Deserialize)]
struct IpAdress {
    mask: String,
    ports: Vec<String>
}
