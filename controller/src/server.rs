use tonic::{transport::Server, Request, Response, Status};

use scheduler::scheduling_service_server::{SchedulingService, SchedulingServiceServer};
use scheduler::{SchedulingRequest, SchedulingResponse};
use axum::{response::Html, routing::get, Router};
use std::net::SocketAddr;
use tokio::task;

pub mod scheduler {
    tonic::include_proto!("orkascheduler"); // The string specified here must match the proto package name
}

#[derive(Debug, Default)]
pub struct MySchedulingService {}

#[tonic::async_trait]
impl SchedulingService for MySchedulingService {
    async fn schedule(
        &self,
        request: Request<SchedulingRequest>,
    ) -> Result<Response<SchedulingResponse>, Status> {
        println!("Got a request: {:?}", request);

        let response = SchedulingResponse {
            status_code: 0,
            rejection_reason: Some(1),
        };

        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // GRPC
    let grpc_addr = "[::1]:50051".parse()?;
    let scheduler = MySchedulingService::default();

    // Spawn the gRPC server as a tokio task
    let grpc_thread = task::spawn(async move {
        println!("Running grpc here: {}", grpc_addr);
        Server::builder()
            .add_service(SchedulingServiceServer::new(scheduler))
            .serve(grpc_addr)
            .await
            .unwrap();
    });

    // HTTP
    let http_addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let app = Router::new().route("/", get(handler));

    // Spawn the HTTP server as a tokio task
    let http_thread = task::spawn(async move {
        println!("Running http here: {}", http_addr);
        axum::Server::bind(&http_addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
    });

    // Wait for both servers to finish
    tokio::try_join!(grpc_thread, http_thread)?;

    Ok(())
}


async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
