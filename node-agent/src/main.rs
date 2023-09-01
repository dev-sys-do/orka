mod workload_manager;

use orka_proto::node_agent::Workload;
use workload_manager::container::client::ContainerClient;

#[tokio::main]
async fn main() {
    let mut workload_manager = ContainerClient::new("/var/run/containerd/containerd.sock").await.unwrap();

    let workload = Workload {
        instance_id: "nginx".to_string(),
        image: "docker.io/library/nginx:latest".to_string(),
        environment: vec!["FOO=BAR".to_string()],
        ..Default::default()
    };

    workload_manager.create(&workload).await.unwrap();

    let response = workload_manager.info("nginx").await.unwrap();

    println!("{:?}", response);
}
