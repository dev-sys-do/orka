//! Node agent manager used to store agents.

use crate::managers::node_agent::metrics::{NodeAgent, NodeCpu, NodeMemory};
use anyhow::Result;
use std::collections::hash_map;
use std::collections::HashMap;
use std::net::SocketAddr;
use tracing::{event, Level};

use super::errors::NodeAgentError;

/// The node agent manager, handling all agents that contact the scheduler.
pub struct NodeAgentManager {
    /// The list of node agents that are currently active in the cluster.
    agents: HashMap<String, NodeAgent>,
}

impl NodeAgentManager {
    /// Create a new `NodeAgentManager` to manage the different node agents.
    pub fn new() -> Self {
        Self {
            agents: HashMap::new(),
        }
    }

    /// Get an iterator over all the registered agents.
    pub fn agents_iter(&self) -> impl Iterator<Item = &NodeAgent> {
        self.agents.values()
    }

    /// Get a registered agent.
    ///
    /// # Arguments
    ///
    /// * `agent_id` - The ID of the agent to return.
    pub fn get_agent(&self, agent_id: &str) -> Option<&NodeAgent> {
        self.agents.get(agent_id)
    }

    /// Add a new agent to the managed list to keep track of it.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the agent to add.
    /// * `addr` - The address the agent is reachable at.
    ///
    /// # Errors
    ///
    /// * An agent with the same ID is already in the cluster.
    pub fn add_agent(
        &mut self,
        id: String,
        addr: SocketAddr,
    ) -> Result<&NodeAgent, NodeAgentError> {
        if let hash_map::Entry::Vacant(e) = self.agents.entry(id.clone()) {
            // No other agent has this ID
            event!(
                Level::INFO,
                agent_id = e.key(),
                address = %addr,
                "Adding new agent to the cluster"
            );

            Ok(e.insert(NodeAgent::new(id, addr)))
        } else {
            // Reject agent as the ID is already registered
            Err(NodeAgentError::AlreadyExists(id))
        }
    }

    /// Remove an agent if it exists, returning it.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the agent to remove.
    pub fn remove_agent(&mut self, id: &str) -> Option<NodeAgent> {
        let removed_agent = self.agents.remove(id);

        if removed_agent.is_some() {
            event!(
                Level::INFO,
                agent_id = id,
                "Removing agent from the cluster"
            );
        }

        removed_agent
    }

    /// Update the node status for the given agent.
    /// Metrics are [`NodeCpu`] and [`NodeMemory`].
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the agent to update.
    /// * `cpu` - The new CPU metrics of the node, if any.
    /// * `memory` - The new memory metrics of the node, if any.
    pub fn update_node_status(
        &mut self,
        id: &str,
        cpu: Option<NodeCpu>,
        memory: Option<NodeMemory>,
    ) -> Result<&mut NodeAgent, NodeAgentError> {
        event!(Level::TRACE, agent_id = id, "Updating the status of a node");

        let agent = self
            .agents
            .get_mut(id)
            .ok_or(NodeAgentError::NotFound(id.to_string()))?;

        agent.update_node_metrics(cpu, memory);
        Ok(agent)
    }
}
