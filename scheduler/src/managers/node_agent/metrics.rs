//! Node agent and its metrics.

use chrono::{DateTime, Local};
use std::net::SocketAddr;

/// The memory (RAM) information of the node the agent is installed on.
#[derive(Debug, Clone)]
pub struct NodeMemory {
    /// Total memory on the machine, in bytes.
    pub total: u64,
    /// Available memory on the machine, in bytes.
    pub free: u64,
}

/// The CPU information of the node the agent is installed on.
#[derive(Debug, Clone)]
pub struct NodeCpu {
    /// CPU load of the machine, represented by the overall CPU usage average.
    /// Lower bound is `0.0`, upper bound is `100.0`.
    pub load: f64,
}

/// The node agent and the information it broadcasts.
#[derive(Debug, Clone)]
pub struct NodeAgent {
    /// The agent's unique id.
    id: String,
    /// Address the agent is reachable at.
    address: SocketAddr,
    /// Heartbeat represents the last time the agent communicated with the scheduler.
    /// This is used to determine whether the agent has timed out.
    last_heartbeat: DateTime<Local>,
    /// The last transmitted memory metrics of the agent's machine.
    /// `None` only if the metrics were never communicated to the scheduler.
    memory: Option<NodeMemory>,
    /// The last transmitted CPU metrics of the agent's machine.
    /// `None` only if the metrics were never communicated to the scheduler.
    cpu: Option<NodeCpu>,
}

impl NodeAgent {
    /// Create a new `NodeAgent`.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the node agent.
    /// * `address` - The address where the node agent is reachable at.
    pub fn new(id: String, address: SocketAddr) -> Self {
        NodeAgent {
            id,
            address,
            last_heartbeat: Local::now(),
            memory: None,
            cpu: None,
        }
    }

    /// Get the agent's id.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Get the agent's formatted server address.
    /// This is the URL that should be used to contact its gRPC server.
    pub fn grpc_url(&self) -> String {
        format!("http://{}", self.address)
    }

    /// Update the agent's last heartbeat.
    pub fn heartbeat(&mut self) {
        self.last_heartbeat = Local::now();
    }

    /// Update the agent's node metrics.
    ///
    /// # Arguments
    ///
    /// * `cpu` - The metrics related to the CPU.
    /// * `memory` - The metrics related to memory.
    pub fn update_node_metrics(&mut self, cpu: Option<NodeCpu>, memory: Option<NodeMemory>) {
        self.heartbeat();
        self.cpu = cpu;
        self.memory = memory;
    }
}
