mod args;
mod workload_manager;

use anyhow::Result;
use args::CliArguments;
use clap::Parser;
use orka_proto::scheduler_agent::{
    lifecycle_service_client::LifecycleServiceClient,
    status_update_service_client::StatusUpdateServiceClient,
};
use orka_proto::scheduler_agent::{ConnectionRequest, DisconnectionNotice};
use std::process::exit;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tracing::{event, info, warn, Level};
use tracing_log::AsTrace;
use uuid::Uuid;
use workload_manager::grpc;

use crate::workload_manager::node::metrics::stream_node_status;

async fn execute_node_lifecycle(
    node_id: Uuid,
    node_agent_port: u16,
    scheduler_connection_string: String,
) -> Result<()> {
    event!(
        Level::INFO,
        "Connecting to scheduler on {}",
        scheduler_connection_string
    );

    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        let mut lifecycle_client =
            match LifecycleServiceClient::connect(scheduler_connection_string.clone()).await {
                Ok(client) => Ok(client),
                Err(e) => {
                    event!(Level::ERROR, "Failed to connect to scheduler: {:?}", e);
                    Err(e)
                }
            }?;

        match lifecycle_client
            .join_cluster(ConnectionRequest {
                id: node_id.to_string(),
                port: node_agent_port as u32,
            })
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => {
                event!(Level::ERROR, "Failed to join cluster: {:?}", e);
                Err(e)
            }
        }?;

        event!(Level::INFO, "Joined cluster");

        let mut client =
            match StatusUpdateServiceClient::connect(scheduler_connection_string.clone()).await {
                Ok(client) => Ok(client),
                Err(e) => {
                    event!(Level::ERROR, "Failed to connect to scheduler: {:?}", e);
                    Err(e)
                }
            }?;

        match stream_node_status(node_id, &mut client, 15).await {
            Ok(_) => {
                event!(Level::INFO, "Node status stream ended, retrying");
            }
            Err(error) => {
                event!(
                    Level::ERROR,
                    "Node status stream failed: {:?}, reconnecting to scheduler",
                    error
                );
            }
        };
    }
}

#[tokio::main]
async fn main() {
    let args = CliArguments::parse();

    tracing_subscriber::fmt()
        .with_max_level(args.verbose.log_level_filter().as_trace())
        .init();

    event!(
        Level::INFO,
        app_name = env!("CARGO_PKG_NAME"),
        app_version = env!("CARGO_PKG_VERSION"),
        "Starting",
    );

    event!(Level::INFO, "Arguments: {:?}", args);

    let scheduler_connection_string =
        format!("http://{}:{}", args.scheduler_address, args.scheduler_port);

    let node_id = Uuid::new_v4();

    // TODO for now the senders will always send 1, but we can use this to send more detailed error codes
    let (tx, mut rx): (Sender<i32>, Receiver<i32>) = mpsc::channel(128);

    let lifecycle_tx = tx.clone();
    let lifecycle_connection_string = scheduler_connection_string.clone();

    // join scheduler and stream node status to scheduler, retrying on failure
    tokio::spawn(async move {
        let retries = args.lifecycle_retries;

        for _ in 0..retries {
            match execute_node_lifecycle(
                node_id,
                args.node_agent_port,
                lifecycle_connection_string.clone(),
            )
            .await
            {
                Ok(_) => {
                    // will never be reached
                }
                Err(e) => {
                    event!(Level::ERROR, "Failed to execute node lifecycle: {:?}", e);
                }
            }
        }

        event!(
            Level::ERROR,
            "Node lifecycle failed, initiating graceful shutdown"
        );

        let _ = lifecycle_tx.send(1).await;
    });

    // start grpc server
    tokio::spawn(async move {
        event!(
            Level::INFO,
            "Starting gRPC server on {}:{}",
            args.node_agent_address,
            args.node_agent_port
        );

        let grpc = grpc::server::GrpcServer::new(args.node_agent_address, args.node_agent_port);

        let server = match grpc.map_err(|e| {
            event!(Level::ERROR, "Failed to create gRPC server: {:?}", e);
        }) {
            Ok(server) => server,
            Err(e) => {
                event!(Level::ERROR, "Failed to create gRPC server: {:?}", e);
                let _ = tx.send(1).await;
                exit(1);
            }
        };

        match server.start_server().await {
            Ok(_) => {}
            Err(e) => {
                event!(Level::ERROR, "Failed to start gRPC server: {:?}", e);
                let _ = tx.send(1).await;
                exit(1);
            }
        };
    });

    let _ = rx.recv().await;

    info!("Exiting");

    info!("Trying to quit cluster");

    match LifecycleServiceClient::connect(scheduler_connection_string).await {
        Err(error) => {
            warn!("Failed connecting to scheduler when exiting {:?}", error)
        }
        Ok(mut client) => {
            match client
                .leave_cluster(DisconnectionNotice {
                    id: node_id.to_string(),
                })
                .await
            {
                Ok(_) => {
                    info!("Successfully left cluster")
                }
                Err(error) => {
                    warn!("Failed existing cluster {:?}", error)
                }
            };
        }
    }

    exit(1);
}
