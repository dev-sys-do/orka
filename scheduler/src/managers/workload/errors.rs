//! Workload errors.

use thiserror::Error;

/// Agent error enum to have self-explanatory and compact errors.
#[derive(Error, Debug)]
pub enum WorkloadError {
    /// The controller doesn't provide a valid Workload .
    #[error("Invalid workload: `{0}`")]
    InvalidWorkload(String),
}
