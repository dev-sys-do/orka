//! Command-line arguments.

use std::{fs, io::ErrorKind};

use anyhow::{Context, Result};
use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};
use tracing::{event, Level};

/// Scheduler service for the Orka container orchestration system.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliArguments {
    /// Directory used to store the configuration.
    #[arg(long, default_value = "/var/lib/orka/scheduler/", env)]
    pub data_dir: String,

    /// Disable TLS for the gRPC server.
    #[arg(long, default_value_t = false, env)]
    pub no_tls: bool,

    /// Disable the automatic generation of the keypair and certificate for TLS.
    #[arg(long, default_value_t = false, env)]
    pub no_tls_secret_generation: bool,

    /// The address to bind the gRPC server to.
    #[arg(long, default_value = "[::]", env)]
    pub grpc_bind_address: String,

    /// The port to bind the gRPC server to.
    #[arg(long, default_value_t = 50051, env)]
    pub grpc_bind_port: u16,

    /// Verbosity level.
    #[command(flatten)]
    pub verbose: Verbosity<InfoLevel>,
}

impl CliArguments {
    /// Prepare the application directories by creating them.
    ///
    /// # Errors
    ///
    /// * The directories could not be created.
    pub fn prepare_directories(&self) -> Result<()> {
        // Create the required directories
        match fs::create_dir_all(&self.data_dir) {
            Err(e) if e.kind() != ErrorKind::AlreadyExists => Err(e),
            _ => Ok(()),
        }
        .with_context(|| {
            format!(
                "Failed to create the main data directory: {}",
                self.data_dir
            )
        })?;

        event!(
            Level::DEBUG,
            path = self.data_dir,
            "Created application data directory"
        );
        Ok(())
    }
}
