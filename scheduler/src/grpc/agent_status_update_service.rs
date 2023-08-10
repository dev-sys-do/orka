//! Status update gRPC service for the Orka node agents.

use orka_proto::scheduler_agent::{
    status_update_service_server::StatusUpdateService, Empty, NodeStatus,
};
use tonic::{Request, Response, Result, Streaming};

/// Implementation of the `StatusUpdateService` gRPC service.
pub struct AgentStatusUpdateSvc {}

impl AgentStatusUpdateSvc {
    /// Create a new `StatusUpdateService` gRPC service manager.
    pub fn new() -> Self {
        Self {}
    }
}

#[tonic::async_trait]
impl StatusUpdateService for AgentStatusUpdateSvc {
    /// Called by node agents to start streaming status information about the node.
    async fn update_node_status(
        &self,
        _: Request<Streaming<NodeStatus>>,
    ) -> Result<Response<Empty>> {
        todo!();
        // Ok(Response::new(Empty {}))
    }
}
