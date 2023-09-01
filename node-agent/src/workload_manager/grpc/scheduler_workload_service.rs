use crate::workload_manager::container::client::ContainerClient;
use crate::workload_manager::container::error::into_tonic_status;
use crate::workload_manager::container::metrics::metrics::any_to_resource;
use orka_proto::node_agent::workload_service_server::WorkloadService;
use orka_proto::node_agent::workload_signal::Signal;
use orka_proto::node_agent::{workload_status, Empty, Workload, WorkloadSignal, WorkloadStatus};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Result, Status};
use tracing::debug;

pub struct WorkloadSvc {}

impl WorkloadSvc {
    /// Create a new `WorkloadService` gRPC service manager.
    pub fn new() -> Self {
        Self {}
    }
}

#[tonic::async_trait]
impl WorkloadService for WorkloadSvc {
    type CreateStream = ReceiverStream<Result<WorkloadStatus>>;

    async fn create(
        &self,
        request: Request<Workload>,
    ) -> Result<Response<Self::CreateStream>, Status> {
        let workload = request.into_inner();
        let mut client = match ContainerClient::new("/var/run/containerd/containerd.sock").await {
            Ok(x) => x,
            Err(e) => {
                return Err(Status::new(
                    tonic::Code::Internal,
                    format!("Failed to create container client{:?}", e),
                ))
            }
        };

        let container = match client.create(&workload).await {
            Ok(x) => x,
            Err(e) => {
                return Err(Status::new(
                    tonic::Code::Internal,
                    format!("Failed to create container: {:?}", e),
                ))
            }
        }
        .into_inner();

        let (tx, rx) = tokio::sync::mpsc::channel(4);
        tokio::spawn(async move {
            loop {
                let metrics = match client.metrics(&container.container_id).await {
                    Ok(x) => x,
                    Err(e) => {
                        return Status::new(
                            tonic::Code::Internal,
                            format!("Failed to get container info: {:?}", e),
                        )
                    }
                }
                .into_inner()
                .metrics;

                let status = match client.status(&container.container_id).await {
                    Ok(x) => x,
                    Err(e) => {
                        return Status::new(
                            tonic::Code::Internal,
                            format!("Failed to get container info: {:?}", e),
                        )
                    }
                }
                .into_inner()
                .process;

                for metric in metrics {
                    let data = metric.data.unwrap();
                    let resource = any_to_resource(&data);
                    let status_code = containerd_client::types::v1::Status::from_i32(
                        status.clone().unwrap().status,
                    )
                    .unwrap();
                    let workload_status = WorkloadStatus {
                        instance_id: workload.instance_id.clone(),
                        status: Some(workload_status::Status {
                            code: match status_code {
                                containerd_client::types::v1::Status::Running => {
                                    workload_status::status::StatusCode::Running as i32
                                }
                                containerd_client::types::v1::Status::Stopped => {
                                    workload_status::status::StatusCode::Terminated as i32
                                }
                                _ => workload_status::status::StatusCode::Waiting as i32,
                            },
                            ..Default::default()
                        }),
                        resource_usage: resource.ok(),
                    };
                    match tx.send(Ok(workload_status)).await {
                        Ok(x) => x,
                        Err(e) => {
                            return Status::new(
                                tonic::Code::Internal,
                                format!("Failed to send workload status: {:?}", e),
                            )
                        }
                    };
                }
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn signal(&self, request: Request<WorkloadSignal>) -> Result<Response<Empty>, Status> {
        let workload_signal = request.into_inner();

        let mut client = ContainerClient::new("/var/run/containerd/containerd.sock")
            .await
            .map_err(|error| {
                Status::new(
                    tonic::Code::Internal,
                    format!("Failed to create container client{:?}", error),
                )
            })?;

        let signal = if workload_signal.signal == Signal::Stop as i32 {
            sysinfo::Signal::Quit
        } else {
            sysinfo::Signal::Kill
        };

        // kill task
        client
            .kill(&workload_signal.instance_id, signal as u32)
            .await
            .map_err(|error| {
                debug!("Failed to kill container {:?}", error);
                into_tonic_status(error)
            })?;

        // wait for container to stop
        client
            .wait(&workload_signal.instance_id)
            .await
            .map_err(|error| {
                debug!("Failed to wait for container {:?}", error);
                into_tonic_status(error)
            })?;

        // cleanup container
        client
            .cleanup(&workload_signal.instance_id)
            .await
            .map_err(|error| {
                debug!("Failed to cleanup container {:?}", error);
                into_tonic_status(error)
            })?;

        Ok(Response::new(Empty {}))
    }
}
