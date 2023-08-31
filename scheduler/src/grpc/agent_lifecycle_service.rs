//! Lifecycle gRPC service for the Orka node agents.

use crate::managers::node_agent::errors::NodeAgentError;
use crate::managers::node_agent::manager::NodeAgentManager;
use orka_proto::scheduler_agent::{
    lifecycle_service_server::LifecycleService, ConnectionRequest, DisconnectionNotice, Empty,
};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tonic::{Request, Response, Result, Status};
use tracing::{event, Level};

/// Implementation of the `LifecycleService` gRPC service.
pub struct AgentLifecycleSvc {
    /// The shared instance of the node agent manager.
    node_agent_manager: Arc<Mutex<NodeAgentManager>>,
}

impl AgentLifecycleSvc {
    /// Create a new `LifecycleService` gRPC service manager.
    ///
    /// # Arguments
    ///
    /// * `manager` - The shared instance of the node agent manager.
    pub fn new(manager: Arc<Mutex<NodeAgentManager>>) -> Self {
        Self {
            node_agent_manager: manager,
        }
    }
}

#[tonic::async_trait]
impl LifecycleService for AgentLifecycleSvc {
    /// Called by node agents when they request to join the cluster.
    async fn join_cluster(
        &self,
        request: Request<ConnectionRequest>,
    ) -> Result<Response<Empty>, Status> {
        let remote_addr = request.remote_addr();
        let inner = request.into_inner();
        let agent_id = inner.id;

        // Gather remote port and address for agent
        let agent_port = u16::try_from(inner.port).map_err(|_| {
            event!(
                Level::ERROR,
                agent_id,
                provided_port = inner.port,
                "Agent provided port outside valid range"
            );

            Status::from(NodeAgentError::NoRemoteAddress())
        })?;

        let remote_addr = remote_addr.ok_or_else(|| {
            event!(
                Level::ERROR,
                agent_id,
                "Could not retrieve remote address during agent registration. Agent would be unreachable, refusing registration"
            );

            Status::from(NodeAgentError::NoRemoteAddress())
        })?;

        // Prepare manager
        let mut manager = self.node_agent_manager.lock().map_err(|err| {
            event!(
                Level::ERROR,
                agent_id,
                error = %err,
                "Failed to acquire node manager, refusing registration for agent"
            );

            Status::internal("Failed to register agent")
        })?;

        // Add agent
        let agent_addr = SocketAddr::new(remote_addr.ip(), agent_port);

        match manager.add_agent(agent_id.clone(), agent_addr) {
            Ok(_) => Ok(Response::new(Empty {})),
            Err(err) => {
                event!(
                    Level::WARN,
                    agent_id,
                    error = %err,
                    "Unable to accept new node agent into the cluster"
                );

                Err(Status::from(err))
            }
        }
    }

    /// Called by node agents when they request to gracefully leave the cluster.
    async fn leave_cluster(
        &self,
        request: Request<DisconnectionNotice>,
    ) -> Result<Response<Empty>> {
        let agent_id = request.into_inner().id;

        match self.node_agent_manager.lock() {
            Ok(mut manager) => {
                manager.remove_agent(&agent_id);
            }
            Err(err) => {
                event!(
                    Level::ERROR,
                    agent_id,
                    error = %err,
                    "Failed to acquire node manager, could not remove agent"
                );
            }
        }

        // We are receiving a notice and are expected not to respond
        // so we always send an empty response
        Ok(Response::new(Empty {}))
    }
}
