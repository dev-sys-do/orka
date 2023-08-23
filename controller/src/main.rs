mod client;
mod errors;
mod routes;
mod types;

use crate::client::scheduler;

use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server, Request, Response, Status};

use axum::Router;
use scheduler::scheduling_service_server::{SchedulingService, SchedulingServiceServer};
use scheduler::{SchedulingRequest, WorkloadStatus};
use std::net::SocketAddr;
use tokio::task;

use axum::routing::{delete, post};
use log::info;
use routes::instances::{delete_instance, get_instances, get_specific_instance, post_instance};
use routes::workloads::{delete_workload, get_specific_workload, get_workloads, post_workload};

#[derive(Debug, Default)]
pub struct MySchedulingService {}

#[tonic::async_trait]
impl SchedulingService for MySchedulingService {
    type ScheduleStream = ReceiverStream<Result<WorkloadStatus, Status>>;

    async fn schedule(
        &self,
        request: Request<SchedulingRequest>,
    ) -> Result<Response<Self::ScheduleStream>, Status> {
        info!("Got a request: {:?}", request);

        let (sender, receiver) = mpsc::channel(4);

        tokio::spawn(async move {
            let fake_statuses_response = vec![
                WorkloadStatus {
                    name: "Workload 1".to_string(),
                    status_code: 0,
                    message: "Your workload is WAITING".to_string(),
                    ..Default::default()
                },
                WorkloadStatus {
                    name: "Workload 1".to_string(),
                    status_code: 1,
                    message: "Your workload is RUNNING".to_string(),
                    ..Default::default()
                },
                WorkloadStatus {
                    name: "Workload 2".to_string(),
                    status_code: 2,
                    message: "Your workload is TERMINATED".to_string(),
                    ..Default::default()
                },
            ];

            for status in fake_statuses_response {
                sender
                    .send(Ok(status))
                    .await
                    .expect("Failed to send status to stream");

                // Attendre 10 secondes avant le prochain envoi
                sleep(Duration::from_secs(10)).await;
            }

            sender
                .send(Err(Status::new(tonic::Code::Ok, "Workload terminated")))
                .await
                .expect("Failed to send status to stream");

            info!("Finished sending statuses");
        });

        let stream_of_workload_status = ReceiverStream::new(receiver);

        Ok(Response::new(stream_of_workload_status))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // SETUP LOGGING
    pretty_env_logger::init();

    // GRPC
    let grpc_addr = "[::1]:50051".parse()?;
    let scheduler = MySchedulingService::default();

    // Spawn the gRPC server as a tokio task
    let grpc_thread = task::spawn(async move {
        info!("Running grpc here: {}", grpc_addr);
        Server::builder()
            .add_service(SchedulingServiceServer::new(scheduler))
            .serve(grpc_addr)
            .await
            .unwrap();
    });

    // HTTP
    let http_addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let app = Router::new()
        .route("/workloads", post(post_workload).get(get_workloads))
        .route(
            "/workloads/:id",
            delete(delete_workload).get(get_specific_workload),
        )
        .route("/instances", post(post_instance).get(get_instances))
        .route(
            "/instances/:id",
            delete(delete_instance).get(get_specific_instance),
        );

    // Spawn the HTTP server as a tokio task
    let http_thread = task::spawn(async move {
        info!("Running http here: {}", http_addr);
        axum::Server::bind(&http_addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
    });

    // Wait for both servers to finish
    tokio::try_join!(grpc_thread, http_thread)?;

    Ok(())
}
