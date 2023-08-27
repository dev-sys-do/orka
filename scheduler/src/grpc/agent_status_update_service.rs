//! Status update gRPC service for the Orka node agents.

use crate::managers::node_agent::manager::NodeAgentManager;
use crate::managers::node_agent::metrics::{NodeCpu, NodeMemory};
use orka_proto::scheduler_agent::{
    status_update_service_server::StatusUpdateService, Empty, NodeStatus,
};
use std::sync::{Arc, Mutex};
use tokio_stream::StreamExt;
use tonic::{Request, Response, Result, Status, Streaming};
use tracing::{event, Level};

/// Implementation of the `StatusUpdateService` gRPC service.
pub struct AgentStatusUpdateSvc {
    /// The shared instance of the node agent manager.
    node_agent_manager: Arc<Mutex<NodeAgentManager>>,
}

impl AgentStatusUpdateSvc {
    /// Create a new `StatusUpdateService` gRPC service manager.
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
impl StatusUpdateService for AgentStatusUpdateSvc {
    /// Called by node agents to start streaming status information about the node.
    async fn update_node_status(
        &self,
        request: Request<Streaming<NodeStatus>>,
    ) -> Result<Response<Empty>> {
        let mut stream = request.into_inner();

        while let Some(result) = stream.next().await {
            match result {
                Ok(status) => match self.node_agent_manager.lock() {
                    Ok(mut manager) => {
                        // Prepare metrics
                        let cpu: Option<NodeCpu> =
                            status.cpu_load.map(|cpu| NodeCpu { load: cpu.load });

                        let memory: Option<NodeMemory> = status.memory.map(|m| NodeMemory {
                            total: m.total,
                            free: m.free,
                        });

                        // Update the node status data
                        let res = manager.update_node_status(&status.id, cpu, memory);

                        if let Err(err) = res {
                            event!(
                                Level::WARN,
                                agent_id = status.id,
                                error = %err,
                                "Unable to process node status update"
                            );

                            return Err(Status::from(err));
                        }
                    }
                    Err(err) => {
                        event!(
                            Level::WARN,
                            agent_id = status.id,
                            error = %err,
                            "Failed to acquire node manager, cannot process node status update for agent"
                        );

                        return Err(Status::internal("Failed to process node status"));
                    }
                },
                Err(err) => {
                    event!(
                        Level::WARN,
                        error = %err,
                        "An error was received while processing a node status update stream"
                    );

                    return Err(Status::internal("An error was received from the client"));
                }
            }
        }

        Ok(Response::new(Empty {}))
    }
}
