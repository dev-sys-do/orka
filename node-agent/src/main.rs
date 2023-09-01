mod args;
mod workload_manager;

use clap::Parser;
use orka_proto::{node_agent::Workload, scheduler_agent::{lifecycle_service_client::LifecycleServiceClient, ConnectionRequest, status_update_service_client::StatusUpdateServiceClient}};
use tracing::{info, error};
use uuid::Uuid;
use workload_manager::container::client::ContainerClient;
use anyhow::Result;
use tracing_log::AsTrace;

use crate::{workload_manager::container::metrics::metrics::any_to_resource, args::CliArguments};
use crate::workload_manager::node::metrics::stream_node_status;

async fn execute_node_lifecycle(
    node_id: Uuid,
    node_agent_port: u16,
    scheduler_connection_string: String,
) -> Result<()> {
    info!(
        "Connecting to scheduler on {}",
        scheduler_connection_string
    );

    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        let mut lifecycle_client =
            match LifecycleServiceClient::connect(scheduler_connection_string.clone()).await {
                Ok(client) => Ok(client),
                Err(e) => {
                    error!("Failed to connect to scheduler: {:?}", e);
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
                error!("Failed to join cluster: {:?}", e);
                Err(e)
            }
        }?;

        info!("Joined cluster");

        let mut client =
            match StatusUpdateServiceClient::connect(scheduler_connection_string.clone()).await {
                Ok(client) => Ok(client),
                Err(e) => {
                    error!("Failed to connect to scheduler: {:?}", e);
                    Err(e)
                }
            }?;

        match stream_node_status(node_id, &mut client, 15).await {
            Ok(_) => {
                info!("Node status stream ended, retrying");
            }
            Err(error) => {
                error!(
                    "Node status stream failed: {:?}, reconnecting to scheduler",
                    error
                );
            }
        };
    }
}

const CID: &str = "nginx";

#[tokio::main]
async fn main() {
    let mut workload_manager = ContainerClient::new("/var/run/containerd/containerd.sock").await.unwrap();

    let workload = Workload {
        instance_id: CID.to_string(),
        image: "docker.io/library/nginx:latest".to_string(),
        environment: vec!["FOO=BAR".to_string()],
        ..Default::default()
    };

    workload_manager.create(&workload).await.unwrap();

    let response = workload_manager.info(CID).await.unwrap();

    println!("{:?}", response);

    let response = workload_manager.status(CID).await.unwrap();

    println!("{:?}", response);

    for _ in 0..10 {
        let response = workload_manager.metrics(CID).await.unwrap();

        for metric in response.into_inner().metrics {
            let data = metric.data.unwrap();

            println!("{:?}", any_to_resource(&data));
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }

    let signal = 9;
    workload_manager.kill(CID, signal).await.unwrap();

    workload_manager.wait(CID).await.unwrap();

    workload_manager.cleanup(CID).await.unwrap();

    match workload_manager.status(CID).await {
        Ok(_) => panic!("Workload should not exist"),
        Err(_) => println!("Workload does not exist"),
    };

    let args = CliArguments::parse();

    tracing_subscriber::fmt()
        .with_max_level(args.verbose.log_level_filter().as_trace())
        .init();

    info!(
        app_name = env!("CARGO_PKG_NAME"),
        app_version = env!("CARGO_PKG_VERSION"),
        "Starting",
    );

    info!("Arguments: {:?}", args);

    let scheduler_connection_string =
        format!("http://{}:{}", args.scheduler_address, args.scheduler_port);

    let node_id = Uuid::new_v4();

    let retries = args.lifecycle_retries;

    for _ in 0..retries {
        match execute_node_lifecycle(
            node_id,
            args.node_agent_port,
            scheduler_connection_string.clone(),
        )
        .await
        {
            Ok(_) => {
                // will never be reached
            }
            Err(e) => {
                error!("Failed to execute node lifecycle: {:?}", e);
            }
        }
    }
}
