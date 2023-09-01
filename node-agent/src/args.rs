//! Command-line arguments.
use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};

/// Scheduler service for the Orka container orchestration system.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliArguments {
    #[arg(long, default_value_t = 3, env)]
    pub lifecycle_retries: i32,

    /// Enable the orka CNI in containerd.
    #[arg(long, default_value_t = false, env)]
    pub cni: bool,

    /// The address of the scheduler to connect the node agent to.
    #[arg(long, default_value = "[::]", env)]
    pub scheduler_address: String,

    /// The port of the scheduler to connect the node agent to.
    #[arg(long, default_value_t = 50051, env)]
    pub scheduler_port: u16,

    /// The address of the node agent for the scheduler to connect to.
    #[arg(long, default_value = "[::]", env)]
    pub node_agent_address: String,

    /// The port of the node agent for the scheduler to connect to.
    #[arg(long, default_value_t = 50052, env)]
    pub node_agent_port: u16,

    /// Verbosity level.
    #[command(flatten)]
    pub verbose: Verbosity<InfoLevel>,
}
