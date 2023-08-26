mod client;
mod errors;
mod routes;
mod store;
mod types;
mod dbstore;

use crate::client::scheduler;

use dbstore::{DB_BATCH, STORE};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server, Request, Response, Status};

use axum::Router;
use scheduler::scheduling_service_server::{SchedulingService, SchedulingServiceServer};
use scheduler::{SchedulingRequest, WorkloadStatus};
use std::net::SocketAddr;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tokio::task;

use axum::routing::{delete, post};
use log::info;
use routes::instances::{delete_instance, get_instances, get_specific_instance, post_instance};
use routes::workloads::{delete_workload, get_specific_workload, get_workloads, post_workload};

#[derive(Debug, Default)]
pub struct Scheduler {}

#[tonic::async_trait]
impl SchedulingService for Scheduler {
    type ScheduleStream = ReceiverStream<Result<WorkloadStatus, Status>>;

    async fn schedule(
        &self,
        request: Request<SchedulingRequest>,
    ) -> Result<Response<Self::ScheduleStream>, Status> {
        println!("Got a request: {:?}", request);

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
    // Initialize logger
    pretty_env_logger::init();

    // Initialize grpc
    let grpc_addr = "[::1]:50051".parse()?;
    let scheduler = Scheduler::default();

    // Spawn the gRPC server as a tokio task
    let grpc_thread = task::spawn(async move {
        info!("gRPC server running at: {}", grpc_addr);
        Server::builder()
            .add_service(SchedulingServiceServer::new(scheduler))
            .serve(grpc_addr)
            .await
            .unwrap();
    });

    // Initialize http
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
        info!("HTTP server running at: {}", http_addr);
        axum::Server::bind(&http_addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
    });

    let db_batch = Arc::clone(&DB_BATCH);
    let db_store = Arc::clone(&STORE);

    let db_thread = task::spawn(async move {
        loop {
            thread::sleep(Duration::from_secs(5));
            let batch = db_batch.lock().unwrap();

            db_store.lock().unwrap().instances_bucket.batch(batch.clone()).unwrap();
        }
    });

    // Wait for both servers and a db thread to finish
    tokio::try_join!(grpc_thread, http_thread, db_thread)?;

    Ok(())
}
