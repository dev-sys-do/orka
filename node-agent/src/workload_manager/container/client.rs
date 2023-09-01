use containerd_client::{
    connect,
    services::v1::{
        containers_client::ContainersClient, tasks_client::TasksClient,
        GetContainerRequest, GetContainerResponse,
    },
    with_namespace,
};
use tokio::process::Command;
use tonic::{transport::Channel, Code, Response};
use tonic::{Request, Status};

use super::error::ContainerClientError;

use orka_proto::node_agent::Workload;

const NAMESPACE: &str = "default";

pub struct CreateContainerResponse {
    pub container_id: String,
}

pub struct ContainerClient {
    sock_path: String,
}

impl ContainerClient {
    async fn get_channel(&self) -> Result<Channel, ContainerClientError> {
        let channel = connect(self.sock_path.clone()).await.map_err(|_| {
            ContainerClientError::ContainerdSocketNotFound {
                sock_path: self.sock_path.clone(),
            }
        })?;
        Ok(channel)
    }

    async fn get_task_client(&self) -> Result<TasksClient<Channel>, ContainerClientError> {
        let channel = self.get_channel().await?;
        Ok(TasksClient::new(channel))
    }

    async fn get_client(&self) -> Result<ContainersClient<Channel>, ContainerClientError> {
        let channel = self.get_channel().await?;
        Ok(ContainersClient::new(channel))
    }

    pub async fn new(sock_path: &str) -> Result<Self, ContainerClientError> {
        let _ = connect(sock_path.clone()).await.map_err(|_| {
            ContainerClientError::ContainerdSocketNotFound {
                sock_path: sock_path.to_string(),
            }
        })?;

        Ok(Self {
            sock_path: sock_path.to_string(),
        })
    }

    pub async fn info(
        &mut self,
        container_id: &str,
    ) -> Result<Response<GetContainerResponse>, ContainerClientError> {
        let request = GetContainerRequest {
            id: container_id.to_string(),
        };

        let request = with_namespace!(request, NAMESPACE);

        let mut client = self.get_client().await?;

        let response = client
            .get(request)
            .await
            .map_err(|status| ContainerClientError::GRPCError { status })?;

        Ok(response)
    }

    pub async fn pull_image_if_not_present(
        &mut self,
        image_name: &str,
    ) -> Result<(), ContainerClientError> {
        // TODO - pull image with standard rust instead of CLI
        let command = Command::new("ctr")
            .arg("image")
            .arg("pull")
            .arg(image_name)
            .output()
            .await
            .map_err(|error| ContainerClientError::GRPCError {
                status: Status::unknown(format!("ctr image pull {:?}", error)),
            })?;

        if !command.status.success() {
            return Err(ContainerClientError::GRPCError {
                status: Status::unknown(format!(
                    "failed pulling image {:?}",
                    String::from_utf8_lossy(&command.stderr)
                )),
            });
        }

        Ok(())
    }

    pub async fn create(
        &mut self,
        workload: &Workload,
    ) -> Result<Response<CreateContainerResponse>, ContainerClientError> {
        match self.info(&workload.instance_id).await {
            Ok(_) => {
                return Err(ContainerClientError::GRPCError {
                    status: Status::already_exists("container already exists".to_string()),
                })
            }
            Err(ContainerClientError::ContainerdSocketNotFound { sock_path }) => {
                return Err(ContainerClientError::ContainerdSocketNotFound { sock_path })
            }
            Err(ContainerClientError::GRPCError { status }) => {
                if status.code() != Code::NotFound {
                    return Err(ContainerClientError::GRPCError { status });
                }
            }
        }

        // TODO - pull image with standard rust instead of CLI
        self.pull_image_if_not_present(&workload.image).await?;

        let env = workload
            .environment
            .iter()
            .map(|value| format!("--env={}", value));

        // TODO - use containerd library to create container instead of ctr
        let command = Command::new("ctr")
            .arg("run")
            .arg("--detach")
            .args(env)
            .arg(&workload.image)
            .arg(&workload.instance_id)
            .output()
            .await
            .map_err(|error| ContainerClientError::GRPCError {
                status: Status::unknown(format!("ctr run failed {:?}", error)),
            })?;

        if !command.status.success() {
            return Err(ContainerClientError::GRPCError {
                status: Status::unknown(format!(
                    "ctr run failed: {}",
                    String::from_utf8_lossy(&command.stderr)
                )),
            });
        }

        Ok(Response::new(CreateContainerResponse {
            container_id: workload.instance_id.to_string(),
        }))
    }
}