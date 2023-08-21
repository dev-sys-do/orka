mod client;

use log::info;

use crate::client::scheduler;
use crate::client::Client;

use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server, Request, Response, Status};

use axum::{response::Html, routing::get, Router};
use scheduler::scheduling_service_server::{SchedulingService, SchedulingServiceServer};
use scheduler::{SchedulingRequest, Workload, WorkloadStatus};
use std::net::SocketAddr;
use tokio::task;

#[derive(Debug, Default)]
pub struct SchedulerService {}

#[tonic::async_trait]
impl SchedulingService for SchedulerService {
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
                    message: "Workload 1 is running".to_string(),
                    ..Default::default()
                },
                WorkloadStatus {
                    name: "Workload 1".to_string(),
                    status_code: 0,
                    message: "Workload 1 is terminated".to_string(),
                    ..Default::default()
                },
                WorkloadStatus {
                    name: "Workload 2".to_string(),
                    status_code: 0,
                    message: "Workload 2 is running".to_string(),
                    ..Default::default()
                },
                WorkloadStatus {
                    name: "Workload 2".to_string(),
                    status_code: 0,
                    message: "Workload 2 is terminated".to_string(),
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
    // // Initialize logger
    pretty_env_logger::init();

    // Initialize gRPC server
    let grpc_addr = "[::1]:50051".parse()?;
    let scheduler = SchedulerService::default();

    // Spawn the gRPC server as a tokio task
    let grpc_thread = task::spawn(async move {
        Server::builder()
            .add_service(SchedulingServiceServer::new(scheduler))
            .serve(grpc_addr)
            .await
            .unwrap();
    });

    // Initialize HTTP server
    let http_addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let app = Router::new().route("/workload", get(handler_workload));

    // Spawn the HTTP server as a tokio task
    let http_thread = task::spawn(async move {
        axum::Server::bind(&http_addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
    });

    info!("Running http here: {}", http_addr);
    info!("Running grpc here: {}", grpc_addr);

    // Wait for both servers to finish
    tokio::try_join!(grpc_thread, http_thread)?;

    Ok(())
}

async fn handler_workload(body: String) -> Html<String> {
    tokio::spawn(async move {
        let mut client = Client::new().await.unwrap();

        let mut workload = Workload::default();
        workload.name = "Mon_Workload".to_string();
        workload.image = "mon_image".to_string();
        workload.environment.push("variable1=valeur1".to_string());
        workload.environment.push("variable3=valeur3".to_string());

        let request = SchedulingRequest {
            workload: Some(workload),
        };

        client.schedule_workload(request).await.unwrap();
    });

    Html(format!("Hello, {}!", body))
}
