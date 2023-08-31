//! Workload manager used to store workload instances.

use std::collections::HashMap;

use tracing::{event, Level};

/// The workload manager, handling all workload instances that were created on the nodes of the
/// cluster.
pub struct WorkloadManager {
    /// The managed instances.
    /// Stored under the form `instance_id -> agent_id`.
    instances: HashMap<String, String>,
}

impl WorkloadManager {
    /// Create a new workload manager.
    pub fn new() -> Self {
        WorkloadManager {
            instances: HashMap::new(),
        }
    }

    /// Add a new instance to the list.
    ///
    /// # Arguments
    ///
    /// * `instance_id` - The ID of the workload instance to add.
    /// * `agent_id` - The ID of the node agent where this workload instance is running.
    pub fn add_instance(&mut self, instance_id: String, agent_id: String) {
        event!(
            Level::TRACE,
            workload_instance_id = instance_id,
            agent_id,
            "Registering new workload instance"
        );

        self.instances.insert(instance_id, agent_id);
    }

    /// Find the node agent associated with a workload instance.
    ///
    /// # Arguments
    ///
    /// * `instance_id` - The ID of the workload instance to find the associated node agent with.
    pub fn find_related_agent(&self, instance_id: &str) -> Option<&String> {
        self.instances.get(&instance_id.to_string())
    }
}
