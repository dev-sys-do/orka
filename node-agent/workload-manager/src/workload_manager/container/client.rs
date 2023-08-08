use containerd_client::{
    connect,
    services::v1::{
        containers_client::ContainersClient, tasks_client::TasksClient, CreateContainerResponse,
    },
};
use tonic::transport::Channel;

use crate::workload_manager::namespace::error::NamespaceClientError;

use anyhow::Result;

use super::error::ContainerClientError;

pub struct ContainerClient {
    sock_path: String,
}

impl ContainerClient {
    async fn get_channel(&self) -> Result<Channel> {
        let channel = connect(self.sock_path.clone()).await.map_err(|_| {
            NamespaceClientError::ContainerdSocketNotFound {
                sock_path: self.sock_path.clone(),
            }
        })?;
        Ok(channel)
    }

    async fn get_task_client(&self) -> Result<TasksClient<Channel>> {
        let channel = self.get_channel().await?;
        Ok(TasksClient::new(channel))
    }

    async fn get_container_client(&self) -> Result<ContainersClient<Channel>> {
        let channel = self.get_channel().await?;
        Ok(ContainersClient::new(channel))
    }

    pub async fn new(sock_path: &str) -> Result<Self> {
        let _ = connect(sock_path.clone()).await.map_err(|_| {
            ContainerClientError::ContainerdSocketNotFound {
                sock_path: sock_path.to_string(),
            }
        })?;

        Ok(Self {
            sock_path: sock_path.to_string(),
        })
    }
}
