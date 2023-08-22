use std::collections::HashMap;

use scheduler::scheduling_service_client::SchedulingServiceClient;
use scheduler::SchedulingRequest;
use tonic::transport::Channel;
use tonic::Streaming;

use self::scheduler::WorkloadStatus;

use self::scheduler::WorkloadStatus;

pub mod scheduler {
    tonic::include_proto!("orkascheduler");
}

pub struct Client {
    client: SchedulingServiceClient<Channel>,
    db_batch: HashMap<String, WorkloadStatus>
}

impl Client {
    pub async fn new() -> anyhow::Result<Self, tonic::transport::Error> {
        let client = SchedulingServiceClient::connect("http://[::1]:50051").await?;
        let db_batch = HashMap::new();
        Ok(Self { client, db_batch })
    }

    pub async fn schedule_workload(
        &mut self,
        scheduling_request: SchedulingRequest,
    ) -> Result<Streaming<WorkloadStatus>, tonic::Status> {
        let request = scheduling_request;

        let response = self.client.schedule(request).await?;

        let stream = response.into_inner();

        Ok(stream)
    }
}
