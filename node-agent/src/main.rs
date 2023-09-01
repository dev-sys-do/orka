mod workload_manager;

use orka_proto::node_agent::Workload;
use workload_manager::container::client::ContainerClient;

use crate::workload_manager::container::metrics::metrics::any_to_resource;

const CID: &str = "nginx";

#[tokio::main]
async fn main() {
    let mut workload_manager = ContainerClient::new("/var/run/containerd/containerd.sock").await.unwrap();

    let workload = Workload {
        instance_id: CID.to_string(),
        image: "docker.io/library/nginx:latest".to_string(),
        environment: vec!["FOO=BAR".to_string()],
        ..Default::default()
    };

    workload_manager.create(&workload).await.unwrap();

    let response = workload_manager.info(CID).await.unwrap();

    println!("{:?}", response);

    let response = workload_manager.status(CID).await.unwrap();

    println!("{:?}", response);

    for _ in 0..10 {
        let response = workload_manager.metrics(CID).await.unwrap();

        for metric in response.into_inner().metrics {
            let data = metric.data.unwrap();

            println!("{:?}", any_to_resource(&data));
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }

    let signal = 9;
    workload_manager.kill(CID, signal).await.unwrap();

    workload_manager.wait(CID).await.unwrap();

    workload_manager.cleanup(CID).await.unwrap();

    match workload_manager.status(CID).await {
        Ok(_) => panic!("Workload should not exist"),
        Err(_) => println!("Workload does not exist"),
    };
}
