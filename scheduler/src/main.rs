mod args;
mod grpc;
mod tls;

use anyhow::Context;
use clap::Parser;
use log::{info, trace, warn};
use std::error;
use std::path::Path;

use crate::args::CliArguments;
use crate::grpc::server::GrpcServer;
use crate::tls::config::TlsConfig;
use crate::tls::manager::TlsManager;

/// The application entry point.
#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    // Parse the configuration and configure logger verbosity
    let args = CliArguments::parse();

    env_logger::builder()
        .filter_level(args.verbose.log_level_filter())
        .init();

    info!(
        "Starting {} version {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );

    trace!("Loaded configuration: {:#?}", args);

    // Prepare the application
    args.prepare_directories()?;

    let tls_manager = if !args.no_tls {
        let tls_base_dir = &Path::new(&args.data_dir).join("tls/");
        let tls_config = TlsConfig::new(tls_base_dir, !args.no_tls_secret_generation);

        trace!("Loaded TLS configuration: {:#?}", tls_config);

        tls_config.prepare_directory()?;

        let mut tls_manager = TlsManager::new(tls_config);
        tls_manager
            .populate_secrets()
            .with_context(|| "Unable to provide the certificate and private key for TLS")?;

        Some(tls_manager)
    } else {
        warn!("The server will run in an unsecure mode because TLS was disabled. Are you certain whatever you're doing is worth it?");
        None
    };

    // Start the gRPC server
    let grpc_server = GrpcServer::new(args.grpc_bind_address, args.grpc_bind_port, tls_manager)
        .with_context(|| "Unable to create the gRPC server manager")?;

    grpc_server.start_server().await?;

    Ok(())
}
