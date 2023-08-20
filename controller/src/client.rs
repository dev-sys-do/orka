use scheduler::scheduling_service_client::SchedulingServiceClient;
use scheduler::SchedulingRequest;
use tonic::transport::Channel;

pub mod scheduler {
    tonic::include_proto!("orkascheduler");
}

pub struct Client {
    client: SchedulingServiceClient<Channel>,
}

impl Client {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let client = SchedulingServiceClient::connect("http://[::1]:50051").await?;
        Ok(Self { client })
    }

    pub async fn schedule_workload(
        &mut self,
        scheduling_request: SchedulingRequest,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let request = scheduling_request;

        let response = self.client.schedule(request).await?;

        let mut stream = response.into_inner();

        while let Some(status) = stream.message().await? {
            println!("STATUS={:?}", status);
        }
        Ok(())
    }
}

fn main() {}