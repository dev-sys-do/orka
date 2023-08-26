use serde::{Deserialize,Serialize};
use validator::Validate;

use crate::client::scheduler::{WorkloadStatus, workload_status::Resources};

#[derive(Debug, Validate, Deserialize, Serialize, Clone)]
pub struct InstanceStatus {
    #[validate(length(min = 1))]
    pub name: String,
    pub status_code: i32,
    pub resource_usage: InstanceResources,
    pub message: String
}

impl From<&WorkloadStatus> for InstanceStatus {
    fn from(status: &WorkloadStatus) -> Self {
        InstanceStatus { 
            name: status.name.clone(),
            status_code: status.status_code,
            resource_usage: InstanceResources { cpu: 1, memory: 1, disk: 1 },
            message: status.message.clone()
        }
    }
}

#[derive(Debug, Validate, Deserialize, Serialize, Clone)]
pub struct InstanceResources {
    pub cpu: i32,

    pub memory: i32,

    pub disk: i32,
}

impl From<Resources> for InstanceResources {
    fn from(res: Resources) -> Self {
        InstanceResources { cpu: res.cpu, memory: res.memory, disk: res.disk }
    }
}
