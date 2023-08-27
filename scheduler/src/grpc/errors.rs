//! Conversions from internal errors to gRPC errors.

use tonic::Status;

use crate::managers::node_agent::errors::NodeAgentError;

impl From<NodeAgentError> for Status {
    fn from(value: NodeAgentError) -> Self {
        match value {
            NodeAgentError::NotFound(_) => Self::not_found(value.to_string()),
            NodeAgentError::AlreadyExists(_) => Self::already_exists(value.to_string()),
        }
    }
}
