//! The gRPC server for interacting with the Orka scheduler.

use std::net::SocketAddr;

use anyhow::{Context, Result};
use orka_proto::node_agent::workload_service_server::WorkloadServiceServer;
use tonic::transport::Server;
use tower_http::trace::TraceLayer;
use tracing::{event, Level};

use super::scheduler_workload_service::WorkloadSvc;

/// The gRPC server manager for the scheduler.
pub struct GrpcServer {
    /// The address to bind the gRPC server to.
    bind_socket_address: SocketAddr,
}

impl GrpcServer {
    /// Create a gRPC server manager.
    ///
    /// # Arguments
    ///
    /// * `bind_address` - The address to bind the gRPC server to.
    /// * `bind_port` - The port to bind the gRPC server to.
    pub fn new(bind_address: String, bind_port: u16) -> Result<Self> {
        let bind_socket_address = format!("{}:{}", bind_address, bind_port)
            .parse()
            .with_context(|| {
                format!(
                    "Unable to parse the gRPC bind address: bind_address={:?}, bind_port={:?}",
                    bind_address, bind_port
                )
            })?;

        Ok(Self {
            bind_socket_address,
        })
    }

    /// Start the gRPC server.
    pub async fn start_server(&self) -> Result<()> {
        // Configure the server
        event!(Level::INFO, bind_address = %self.bind_socket_address, "Starting gRPC server");

        let mut server_builder = Server::builder().layer(TraceLayer::new_for_grpc());

        // Configure the router
        let router = server_builder.add_service(WorkloadServiceServer::new(WorkloadSvc::new()));

        event!(Level::DEBUG, "The gRPC server was configured successfully");

        router
            .serve(self.bind_socket_address)
            .await
            .with_context(|| "An error occurred while serving gRPC requests")?;

        Ok(())
    }
}
