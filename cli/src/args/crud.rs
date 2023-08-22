use clap::Parser;

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum ApiResourceType {
    Workload,
    Instance,
}

#[derive(Debug, Parser)]
pub struct CreateType {
    #[clap(subcommand)]
    command: CreateCommandType,
}

#[derive(Debug, clap::Subcommand)]
pub enum CreateCommandType {
    /// Create a workload
    Workload(CreateWorkloadType),
    /// Create a instance
    Instance(CreateInstanceType),
}

#[derive(Debug, Parser)]
pub struct CreateWorkloadType {}

#[derive(Debug, Parser)]
pub struct CreateInstanceType {}

#[derive(Debug, Parser)]
pub struct GetType {}

#[derive(Debug, Parser)]
pub struct DeleteType {}
