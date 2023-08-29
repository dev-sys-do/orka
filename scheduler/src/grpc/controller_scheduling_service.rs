//! Scheduling gRPC service for the Orka controller.

use std::pin::Pin;

use orka_proto::scheduler_controller::{
    scheduling_service_server::SchedulingService, Empty, SchedulingRequest, WorkloadInstance,
    WorkloadStatus,
};
use tokio_stream::Stream;
use tonic::{Request, Response, Result, Status};

/// Implementation of the `SchedulingService` gRPC service.
pub struct ControllerSchedulingSvc {}

impl ControllerSchedulingSvc {
    /// Create a new `SchedulingService` gRPC service manager.
    pub fn new() -> Self {
        Self {}
    }
}

#[tonic::async_trait]
impl SchedulingService for ControllerSchedulingSvc {
    type ScheduleStream = Pin<Box<dyn Stream<Item = Result<WorkloadStatus>> + Send>>;

    /// Called by the controller when it requests to schedule a workload on a node. The scheduler
    /// responds by streaming status information about the workload.
    async fn schedule(
        &self,
        _: Request<SchedulingRequest>,
    ) -> Result<Response<Self::ScheduleStream>> {
        todo!();
        // Example: https://github.com/hyperium/tonic/blob/master/examples/src/streaming
        //let (tx, rx) = mpsc::channel(128);
        //tokio::spawn(async move {
        //    match tx.send(Ok(WorkloadStatus {
        //        name: "my_workload".to_string(),
        //        status_code: StatusCode::Running.into(),
        //        resource_usage: Some(Resources {
        //            cpu: 1,
        //            memory: 1,
        //            disk: 1,
        //        }),
        //        message: "Everything is fine".to_string(),
        //    }))
        //    .await {
        //        Ok(_) => (),
        //        Err(_) => (),
        //    };
        //});

        //let output_stream = ReceiverStream::new(rx);
        //Ok(Response::new(
        //    Box::pin(output_stream) as Self::ScheduleStream
        //))
    }

    /// Called by the controller to request a workload instance to be gracefully stopped.
    async fn stop(
        &self,
        _: Request<WorkloadInstance>,
    ) -> std::result::Result<Response<Empty>, Status> {
        todo!()
    }

    /// Called by the controller to request a workload instance to be terminated.
    async fn destroy(
        &self,
        _: Request<WorkloadInstance>,
    ) -> std::result::Result<Response<Empty>, Status> {
        todo!()
    }
}
