mod client;
mod errors;
mod routes;
mod store;
mod types;

use store::kv_manager::DB_STORE;
use orka_proto::scheduler_controller::{self, WorkloadInstance};
use store::kv_manager::{KeyValueBatch, DB_BATCH};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

use axum::Router;
use orka_proto::scheduler_controller::scheduling_service_server::SchedulingService;
use orka_proto::scheduler_controller::workload_status::Resources;
use orka_proto::scheduler_controller::{
    workload_status::Status as DeploymentStatus, SchedulingRequest, WorkloadStatus,
};
use std::net::SocketAddr;
use std::thread;
use std::time::Duration;
use tokio::task;

use axum::routing::{delete, post};
use log::{error, info};
use routes::instances::{
    delete_instance, delete_instance_force, get_instances, get_specific_instance, post_instance,
};
use routes::workloads::{delete_workload, get_specific_workload, get_workloads, post_workload};

#[derive(Debug, Default)]
pub struct Scheduler {}

#[tonic::async_trait]
impl SchedulingService for Scheduler {
    type ScheduleStream = ReceiverStream<Result<WorkloadStatus, Status>>;

    async fn stop(
        &self,
        request: Request<WorkloadInstance>,
    ) -> Result<Response<scheduler_controller::Empty>, Status> {
        info!("{:?}", request);
        Ok(Response::new(scheduler_controller::Empty {}))
    }

    async fn destroy(
        &self,
        request: Request<WorkloadInstance>,
    ) -> Result<Response<scheduler_controller::Empty>, Status> {
        info!("{:?}", request);
        Ok(Response::new(scheduler_controller::Empty {}))
    }

    async fn schedule(
        &self,
        request: Request<SchedulingRequest>,
    ) -> Result<Response<Self::ScheduleStream>, Status> {
        info!("{:?}", request);

        let (sender, receiver) = mpsc::channel(4);

        let workload = request.into_inner().workload.unwrap();

        tokio::spawn(async move {
            let fake_statuses_response = vec![
                WorkloadStatus {
                    instance_id: workload.instance_id.clone(),
                    status: Some(DeploymentStatus {
                        code: 0,
                        message: Some("The workload is waiting".to_string()),
                    }),
                    resource_usage: Some(Resources {
                        cpu: 2,
                        memory: 3,
                        disk: 4,
                    }),
                },
                WorkloadStatus {
                    instance_id: workload.instance_id.clone(),
                    status: Some(DeploymentStatus {
                        code: 1,
                        message: Some("The workload is running".to_string()),
                    }),
                    resource_usage: Some(Resources {
                        cpu: 2,
                        memory: 3,
                        disk: 4,
                    }),
                },
                WorkloadStatus {
                    instance_id: workload.instance_id,
                    status: Some(DeploymentStatus {
                        code: 2,
                        message: Some("The workload is terminated".to_string()),
                    }),
                    resource_usage: Some(Resources {
                        cpu: 2,
                        memory: 3,
                        disk: 4,
                    }),
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
    // let grpc_addr = "[::1]:50051".parse()?;
    // let scheduler = Scheduler::default();

    // // Spawn the gRPC server as a tokio task
    // let grpc_thread = task::spawn(async move {
    //     info!("gRPC server running at: {}", grpc_addr);
    //     Server::builder()
    //         .add_service(SchedulingServiceServer::new(scheduler))
    //         .serve(grpc_addr)
    //         .await
    //         .unwrap();
    // });

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
        )
        .route(
            "/instances/:id/force",
            delete(delete_instance_force).get(get_specific_instance),
        );

    // Spawn the HTTP server as a tokio task
    let http_thread = task::spawn(async move {
        info!("HTTP server running at: {}", http_addr);
        axum::Server::bind(&http_addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
    });

    // Create a thread to sync the batch with the database
    let db_thread = task::spawn(async move {
        loop {
            thread::sleep(Duration::from_secs(5));
            let kv_batch = DB_BATCH.lock();
            let kv_store = DB_STORE.lock();
            match kv_batch {
                Ok(mut kvbatch) => {
                    match kv_store {
                        Ok(store) => {
                            store
                                .instances_bucket()
                                .unwrap()
                                .batch(kvbatch.batch.clone())
                                .unwrap();
                            // Clear batch after update
                            *kvbatch = KeyValueBatch::new();
                        }
                        Err(e) => error!("{}", e),
                    }
                }
                Err(e) => error!("{}", e),
            }
        }
    });

    // Wait for both servers and a db thread to finish
    tokio::try_join!(http_thread, db_thread)?;

    Ok(())
}
