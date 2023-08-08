use containerd_client::{
    connect,
    services::v1::{
        containers_client::ContainersClient, tasks_client::TasksClient, CreateContainerResponse, ListContainersResponse, ListContainersRequest, Container, CreateContainerRequest, DeleteContainerRequest,
    },
};
use tonic::{transport::Channel, Response, Code};

use anyhow::Result;

use super::error::ContainerClientError;

pub struct ContainerClient {
    sock_path: String,
}

impl ContainerClient {
    async fn get_channel(&self) -> Result<Channel> {
        let channel = connect(self.sock_path.clone()).await.map_err(|_| {
            ContainerClientError::ContainerdSocketNotFound {
                sock_path: self.sock_path.clone(),
            }
        })?;
        Ok(channel)
    }

    async fn get_task_client(&self) -> Result<TasksClient<Channel>> {
        let channel = self.get_channel().await?;
        Ok(TasksClient::new(channel))
    }

    async fn get_client(&self) -> Result<ContainersClient<Channel>> {
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

    pub async fn list(&mut self, filters: Vec<String>) -> Result<Response<ListContainersResponse>> {
        let request = ListContainersRequest {
            filters,
        };

        let mut client = self.get_client().await?;

        let response = client
            .list(request)
            .await
            .map_err(|error| ContainerClientError::Unknown { error })?;

        Ok(response)
    }

    pub async fn create(&mut self, id: &str) -> Result<Response<CreateContainerResponse>> {
        let new_container = Container {
            ..Container::default()
        };

        let request = CreateContainerRequest {
            container: Some(new_container),
        };

        let mut client = self.get_client().await?;

        let response = client.create(request).await.map_err(|error| {
            if error.code() == Code::AlreadyExists {
                return ContainerClientError::AlreadyExists {
                    container_id: id.to_string(),
                };
            }

            ContainerClientError::Unknown { error }
        })?;

        Ok(response)
    }

    pub async fn delete(&mut self, id: &str) -> Result<Response<()>> {
        let request = DeleteContainerRequest {
            id: id.to_string(),
        };

        let mut client = self.get_client().await?;

        let response = client.delete(request).await.map_err(|error| {
            if error.code() == Code::NotFound {
                return ContainerClientError::NotFound {
                    container_id: id.to_string(),
                };
            }

            ContainerClientError::Unknown { error }
        })?;

        Ok(response)
    }
}
