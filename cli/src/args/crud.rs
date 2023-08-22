use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
pub struct CreateType {
    #[clap(subcommand)]
    pub command: CreateCommandType,
}

#[derive(Debug, clap::Subcommand)]
pub enum CreateCommandType {
    /// Create a workload
    Workload(CreateWorkload),
    /// Create a instance
    Instance(CreateInstance),
}

/// Create functions
#[derive(Debug, Parser)]
pub struct CreateWorkload {
    #[arg(short)]
    /// The file path
    pub file_path: PathBuf,
}

#[derive(Debug, Parser)]
pub struct CreateInstance {
    /// The workload id
    pub workload_id: String,
}

/// Get function
#[derive(Debug, Parser)]
pub struct GetType {
    #[clap(subcommand)]
    pub command: GetCommandType,
}

#[derive(Debug, clap::Subcommand)]
pub enum GetCommandType {
    /// Get all or a single workload
    Workload(GetWorkload),
    /// Get all or a single instance
    Instance(GetInstance),
}

#[derive(Debug, Parser)]
pub struct GetWorkload {
    /// The workload ID
    #[arg(id = "id", long)]
    pub workload_id: Option<String>,
}

#[derive(Debug, Parser)]
pub struct GetInstance {
    #[arg(id = "id", long)]
    /// The instance ID
    pub instance_id: Option<String>,
}

/// Delete command
#[derive(Debug, Parser)]
pub struct DeleteType {
    #[clap(subcommand)]
    pub command: DeleteCommandType,
}

#[derive(Debug, clap::Subcommand)]
pub enum DeleteCommandType {
    /// Delete a single workload
    Workload(DeleteWorkload),
    /// Delete a single instance
    Instance(DeleteInstance),
}

#[derive(Debug, Parser)]
pub struct DeleteWorkload {
    #[arg(long)]
    /// The workload id
    pub workload_id: String,
}

#[derive(Debug, Parser)]
pub struct DeleteInstance {
    #[arg(long)]
    /// The instance id
    pub instance_id: String,
}
