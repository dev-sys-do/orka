//! Node agent errors.

use thiserror::Error;

/// Agent error enum to have self-explanatory and compact errors.
#[derive(Error, Debug)]
pub enum NodeAgentError {
    /// The node agent could not be found.
    #[error("Agent not found: `{0}`")]
    NotFound(String),

    /// The node agent is already registered.
    #[error("Agent already exists: `{0}`")]
    AlreadyExists(String),
}
