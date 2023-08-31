use orka_proto::scheduler_controller::SchedulingRequest;
use orka_proto::scheduler_controller::{self, scheduling_service_client::SchedulingServiceClient};
use tonic::transport::Channel;
use tonic::Streaming;

use orka_proto::scheduler_controller::{WorkloadInstance, WorkloadStatus};

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
    ) -> Result<scheduler_controller::Empty, tonic::Status> {
        let response = self.client.stop(instance).await?;

        Ok(response.into_inner())
    }

    pub async fn destroy_instance(
        &mut self,
        instance: WorkloadInstance,
    ) -> Result<scheduler_controller::Empty, tonic::Status> {
        let response = self.client.destroy(instance).await?;

        Ok(response.into_inner())
    }
}
