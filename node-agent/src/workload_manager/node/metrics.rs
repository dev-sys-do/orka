use orka_proto::scheduler_agent::status_update_service_client::StatusUpdateServiceClient;
use sysinfo::{CpuExt, System, SystemExt};

use orka_proto::scheduler_agent::node_status::{CpuLoad, Memory};
use orka_proto::scheduler_agent::NodeStatus;

use anyhow::Result;
use tonic::Request;
use uuid::Uuid;

fn get_node_status(node_id: &Uuid, system: &mut System) -> NodeStatus {
    system.refresh_all();

    let total_memory = system.total_memory();

    let free_memory = system.free_memory();

    let cpus = system.cpus();

    let average_load =
        cpus.iter().map(|cpu| cpu.cpu_usage() as f64).sum::<f64>() / cpus.len() as f64;

    let cpu_load = if average_load == f64::INFINITY {
        None
    } else {
        Some(CpuLoad { load: average_load })
    };

    NodeStatus {
        id: node_id.to_string(),
        memory: Some(Memory {
            total: total_memory,
            free: free_memory,
        }),
        cpu_load,
    }
}

pub async fn stream_node_status(
    node_id: Uuid,
    client: &mut StatusUpdateServiceClient<tonic::transport::Channel>,
    interval_sec: u64,
) -> Result<()> {
    let mut system = System::new();

    let outbound = async_stream::stream! {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(interval_sec));

        loop {
            let node_status = get_node_status(&node_id, &mut system);
            yield node_status;
            interval.tick().await;
        }
    };

    let _ = client.update_node_status(Request::new(outbound)).await?;

    Ok(())
}
