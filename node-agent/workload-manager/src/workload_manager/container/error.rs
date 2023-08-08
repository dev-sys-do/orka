use thiserror::Error;
use tonic::Status;

#[derive(Error, Debug)]
pub enum ContainerClientError {
    #[error("Socket {sock_path:?} not found")]
    ContainerdSocketNotFound { sock_path: String },
    #[error("Container {container_id:?} already exists")]
    AlreadyExists { container_id: String },
    #[error("Container {container_id:?} not found")]
    NotFound { container_id: String },
    #[error("Unknown error occured {error:?}")]
    Unknown { error: Status },
}
