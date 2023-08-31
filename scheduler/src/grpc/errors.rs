//! Conversions from internal errors to gRPC errors.

use tonic::Status;

use crate::managers::node_agent::errors::NodeAgentError;
use crate::managers::workload::errors::WorkloadError;

impl From<NodeAgentError> for Status {
    fn from(value: NodeAgentError) -> Self {
        match value {
            NodeAgentError::NotFound(_) => Self::not_found(value.to_string()),
            NodeAgentError::AlreadyExists(_) => Self::already_exists(value.to_string()),
            NodeAgentError::NoRemoteAddress() => Self::internal(value.to_string()),
            NodeAgentError::InvalidPort() => Status::invalid_argument(value.to_string()),
        }
    }
}

impl From<WorkloadError> for Status {
    fn from(value: WorkloadError) -> Self {
        match value {
            WorkloadError::InvalidWorkload(_) => Status::invalid_argument(value.to_string()),
        }
    }
}
