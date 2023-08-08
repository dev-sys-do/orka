use containerd_client::{
    connect,
    services::v1::{
        namespaces_client::NamespacesClient, CreateNamespaceRequest, CreateNamespaceResponse,
        DeleteNamespaceRequest, ListNamespacesRequest, ListNamespacesResponse, Namespace,
    },
};
use tonic::{transport::Channel, Code, Response};

use anyhow::Result;

use super::error::NamespaceClientError;

pub struct NamespaceClient {
    sock_path: String,
}

impl NamespaceClient {
    async fn get_channel(&self) -> Result<Channel> {
        let channel = connect(self.sock_path.clone()).await.map_err(|_| {
            NamespaceClientError::ContainerdSocketNotFound {
                sock_path: self.sock_path.clone(),
            }
        })?;
        Ok(channel)
    }

    async fn get_client(&self) -> Result<NamespacesClient<Channel>> {
        let channel = self.get_channel().await?;
        Ok(NamespacesClient::new(channel))
    }

    pub async fn list(&mut self, filter: &str) -> Result<Response<ListNamespacesResponse>> {
        let request = ListNamespacesRequest {
            filter: filter.to_string(),
        };

        let mut client = self.get_client().await?;

        let response = client
            .list(request)
            .await
            .map_err(|error| NamespaceClientError::Unknown { error })?;

        Ok(response)
    }

    pub async fn create(&mut self, name: &str) -> Result<Response<CreateNamespaceResponse>> {
        let new_namespace = Namespace {
            name: name.to_string(),
            ..Namespace::default()
        };

        let request = CreateNamespaceRequest {
            namespace: Some(new_namespace),
        };

        let mut client = self.get_client().await?;

        let response = client.create(request).await.map_err(|error| {
            if error.code() == Code::AlreadyExists {
                return NamespaceClientError::AlreadyExists {
                    namespace: name.to_string(),
                };
            }

            NamespaceClientError::Unknown { error }
        })?;

        Ok(response)
    }

    pub async fn delete(&mut self, name: &str) -> Result<Response<()>> {
        let request = DeleteNamespaceRequest {
            name: name.to_string(),
        };

        let mut client = self.get_client().await?;

        let response = client.delete(request).await.map_err(|error| {
            if error.code() == Code::NotFound {
                return NamespaceClientError::NotFound {
                    namespace: name.to_string(),
                };
            }

            NamespaceClientError::Unknown { error }
        })?;

        Ok(response)
    }

    pub async fn new(sock_path: &str) -> Result<Self> {
        let _ = connect(sock_path.clone()).await.map_err(|_| {
            NamespaceClientError::ContainerdSocketNotFound {
                sock_path: sock_path.to_string(),
            }
        })?;

        Ok(Self {
            sock_path: sock_path.to_string(),
        })
    }
}
