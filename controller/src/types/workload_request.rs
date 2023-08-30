use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::{Validate, ValidationError};

use crate::client::scheduler::{self, workload::{Type, Resources}};

#[derive(Debug, Validate, Deserialize, Serialize, Clone)]
pub struct WorkloadRequest {
    pub version: String,
    pub workload: Workload,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum WorkloadKind {
    Container,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum WorkloadRegistry {
    Docker,
    Podman,
    Ghcr,
}
#[derive(Debug, Validate, Deserialize, Serialize, Clone)]
pub struct Workload {
    #[validate(custom = "validate_workload_kind")]
    pub kind: WorkloadKind,

    #[validate(length(min = 1))]
    pub name: String,

    pub environment: Vec<String>,

    #[validate(custom = "validate_workload_registry")]
    pub registry: WorkloadRegistry,

    #[validate(length(min = 1))]
    pub image: String,

    pub port: String,

    pub network: Vec<String>,
}

fn validate_workload_kind(kind: &WorkloadKind) -> Result<(), ValidationError> {
    match kind {
        WorkloadKind::Container => Ok(()),
    }
}

fn validate_workload_registry(registry: &WorkloadRegistry) -> Result<(), ValidationError> {
    match registry {
        WorkloadRegistry::Docker => Ok(()),
        WorkloadRegistry::Podman => Ok(()),
        WorkloadRegistry::Ghcr => Ok(()),
    }
}

impl From<Workload> for scheduler::Workload {
    fn from(workload: Workload) -> scheduler::Workload {
        // Create a grpc workload object
        scheduler::Workload {
            instance_id: format!("instance-{}-{}", workload.name, Uuid::new_v4()),
            r#type: Type::Container.into(),
            image: workload.image,
            environment: workload.environment,
            resource_limits: Some(Resources::default()),
        }
    }
}
