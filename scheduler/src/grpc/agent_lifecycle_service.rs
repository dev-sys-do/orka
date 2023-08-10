//! Lifecycle gRPC service for the Orka node agents.

use orka_proto::scheduler_agent::{
    lifecycle_service_server::LifecycleService, ConnectionRequest, ConnectionResponse, Empty,
};
use tonic::{Request, Response, Result};

/// Implementation of the `LifecycleService` gRPC service.
pub struct AgentLifecycleSvc {}

impl AgentLifecycleSvc {
    /// Create a new `LifecycleService` gRPC service manager.
    pub fn new() -> Self {
        Self {}
    }
}

#[tonic::async_trait]
impl LifecycleService for AgentLifecycleSvc {
    /// Called by node agents when they request to join the cluster.
    async fn join_cluster(
        &self,
        _: Request<ConnectionRequest>,
    ) -> Result<Response<ConnectionResponse>> {
        todo!();
        // Ok(Response::new(ConnectionResponse { status_code: 200 }))
    }

    /// Called by node agents when they request to gracefully leave the cluster.
    async fn leave_cluster(&self, _: Request<Empty>) -> Result<Response<Empty>> {
        todo!();
        // Ok(Response::new(Empty {}))
    }
}
