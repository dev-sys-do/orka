//! The gRPC server for interacting with the Orka scheduler.

use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use crate::managers::node_agent::manager::NodeAgentManager;
use anyhow::{Context, Result};
use orka_proto::{
    scheduler_agent::{
        lifecycle_service_server::LifecycleServiceServer,
        status_update_service_server::StatusUpdateServiceServer,
    },
    scheduler_controller::scheduling_service_server::SchedulingServiceServer,
};
use tonic::transport::{Identity, Server, ServerTlsConfig};
use tower_http::trace::TraceLayer;
use tracing::{event, Level};

use crate::tls::manager::TlsManager;

use super::{
    agent_lifecycle_service::AgentLifecycleSvc, agent_status_update_service::AgentStatusUpdateSvc,
    controller_scheduling_service::ControllerSchedulingSvc,
};

/// The gRPC server manager for the scheduler.
pub struct GrpcServer {
    /// The address to bind the gRPC server to.
    bind_socket_address: SocketAddr,

    /// The TLS manager, if it is enabled.
    tls_manager: Option<TlsManager>,
}

impl GrpcServer {
    /// Create a gRPC server manager.
    ///
    /// # Arguments
    ///
    /// * `bind_address` - The address to bind the gRPC server to.
    /// * `bind_port` - The port to bind the gRPC server to.
    /// * `tls_manager` - The TLS manager, if TLS is enabled.
    pub fn new(
        bind_address: String,
        bind_port: u16,
        tls_manager: Option<TlsManager>,
    ) -> Result<Self> {
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
            tls_manager,
        })
    }

    /// Start the gRPC server.
    pub async fn start_server(&self) -> Result<()> {
        // Configure the server
        event!(Level::INFO, bind_address = %self.bind_socket_address, "Starting gRPC server");

        let mut server_builder = Server::builder().layer(TraceLayer::new_for_grpc());

        // If the TLS manager is present, configure the gRPC server with TLS
        if let Some(tls_manager) = &self.tls_manager {
            event!(Level::DEBUG, "Configuring the gRPC server for TLS");

            let cert_data = tls_manager
                .cert_data()
                .with_context(|| "The certificate data is missing")?;

            let key_data = tls_manager
                .key_data()
                .with_context(|| "The private key data is missing")?;

            let tls_config =
                ServerTlsConfig::new().identity(Identity::from_pem(cert_data, key_data));

            server_builder = server_builder
                .tls_config(tls_config)
                .with_context(|| "Unable to configure TLS with the gRPC server")?;
        }

        // Create the shared node agent manager
        let node_agent_manager = Arc::new(Mutex::new(NodeAgentManager::new()));

        // Configure the router
        let router = server_builder
            .add_service(LifecycleServiceServer::new(AgentLifecycleSvc::new(
                Arc::clone(&node_agent_manager),
            )))
            .add_service(StatusUpdateServiceServer::new(AgentStatusUpdateSvc::new(
                Arc::clone(&node_agent_manager),
            )))
            .add_service(SchedulingServiceServer::new(ControllerSchedulingSvc::new()));

        event!(Level::DEBUG, "The gRPC server was configured successfully");

        router
            .serve(self.bind_socket_address)
            .await
            .with_context(|| "An error occurred while serving gRPC requests")?;

        Ok(())
    }
}
