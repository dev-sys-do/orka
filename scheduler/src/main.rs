mod args;
mod grpc;
mod managers;
mod tls;

use anyhow::Context;
use clap::Parser;
use std::error;
use std::path::Path;
use tracing::{event, Level};
use tracing_log::AsTrace;

use crate::args::CliArguments;
use crate::grpc::server::GrpcServer;
use crate::tls::config::TlsConfig;
use crate::tls::manager::TlsManager;

/// The application entry point.
#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    // Parse the configuration and configure logger verbosity
    let args = CliArguments::parse();

    tracing_subscriber::fmt()
        .with_max_level(args.verbose.log_level_filter().as_trace())
        .init();

    event!(
        Level::INFO,
        app_name = env!("CARGO_PKG_NAME"),
        app_version = env!("CARGO_PKG_VERSION"),
        "Starting application"
    );

    event!(Level::TRACE, ?args, "Loaded configuration");

    // Prepare the application
    args.prepare_directories()?;

    let tls_manager = if !args.no_tls {
        let tls_base_dir = &Path::new(&args.data_dir).join("tls/");
        let tls_config = TlsConfig::new(tls_base_dir, !args.no_tls_secret_generation);

        event!(Level::TRACE, ?tls_config, "Loaded TLS configuration");

        tls_config.prepare_directory()?;

        let mut tls_manager = TlsManager::new(tls_config);
        tls_manager
            .populate_secrets()
            .with_context(|| "Unable to provide the certificate and private key for TLS")?;

        Some(tls_manager)
    } else {
        event!(Level::WARN, "The server will run in an unsecure mode because TLS was disabled. Are you certain whatever you're doing is worth it?");
        None
    };

    // Start the gRPC server
    let grpc_server = GrpcServer::new(args.grpc_bind_address, args.grpc_bind_port, tls_manager)
        .with_context(|| "Unable to create the gRPC server manager")?;

    grpc_server.start_server().await?;

    Ok(())
}
