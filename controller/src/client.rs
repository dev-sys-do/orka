use scheduler::scheduling_service_client::SchedulingServiceClient;
use scheduler::SchedulingRequest;
use tonic::transport::Channel;
use tonic::Streaming;

use self::scheduler::{WorkloadInstance, WorkloadStatus};

pub mod scheduler {
    tonic::include_proto!("scheduler.controller");
}

pub struct Client {
    client: SchedulingServiceClient<Channel>,
}

impl Client {
    pub async fn new() -> anyhow::Result<Self, tonic::transport::Error> {
        let client = SchedulingServiceClient::connect("http://[::1]:50051").await?;
        Ok(Self { client })
    }

    pub async fn schedule_workload(
        &mut self,
        scheduling_request: SchedulingRequest,
    ) -> Result<Streaming<WorkloadStatus>, tonic::Status> {
        let response = self.client.schedule(scheduling_request).await?;

        let stream = response.into_inner();

        Ok(stream)
    }

    pub async fn stop_instance(
        &mut self,
        instance: WorkloadInstance,
    ) -> Result<scheduler::Empty, tonic::Status> {
        let response = self.client.stop(instance).await?;

        Ok(response.into_inner())
    }

    pub async fn destroy_instance(
        &mut self,
        instance: WorkloadInstance,
    ) -> Result<scheduler::Empty, tonic::Status> {
        let response = self.client.destroy(instance).await?;

        Ok(response.into_inner())
    }
}
