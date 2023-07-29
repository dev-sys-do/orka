use scheduler::scheduling_service_client::SchedulingServiceClient;
use scheduler::{SchedulingRequest, Workload};

pub mod scheduler {
    tonic::include_proto!("orkascheduler");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = SchedulingServiceClient::connect("http://[::1]:50051").await?;

    let mut workload = Workload::default();
    workload.name = "Mon_Workload".to_string();
    workload.image = "mon_image".to_string();
    workload.environment.push("variable1=valeur1".to_string());
    workload.environment.push("variable3=valeur3".to_string());

    let request = SchedulingRequest {
        workload: Some(workload),
    };

    let response = client.schedule(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
