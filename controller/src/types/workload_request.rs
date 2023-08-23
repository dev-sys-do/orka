use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

#[derive(Debug, Validate, Deserialize)]
pub struct WorkloadRequest {
    pub version: String,
    pub workload: Workload,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum WorkloadKind {
    Container,
    Baremetal,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum WorkloadRegistry {
    Docker,
    Podman,
    Ghcr,
}
#[derive(Debug, Validate, Deserialize)]
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
        WorkloadKind::Baremetal => Ok(()),
    }
}

fn validate_workload_registry(registry: &WorkloadRegistry) -> Result<(), ValidationError> {
    match registry {
        WorkloadRegistry::Docker => Ok(()),
        WorkloadRegistry::Podman => Ok(()),
        WorkloadRegistry::Ghcr => Ok(()),
    }
}
