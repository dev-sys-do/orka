//! Node agent and its metrics.

use chrono::{DateTime, Local};

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
    pub fn new() -> Self {
        NodeAgent {
            last_heartbeat: Local::now(),
            memory: None,
            cpu: None,
        }
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
