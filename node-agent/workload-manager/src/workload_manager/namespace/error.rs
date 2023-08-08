use thiserror::Error;
use tonic::Status;

#[derive(Error, Debug)]
pub enum NamespaceClientError {
    #[error("Socket {sock_path:?} not found")]
    ContainerdSocketNotFound { sock_path: String },
    #[error("Namespace {namespace:?} already exists")]
    AlreadyExists { namespace: String },
    #[error("Namespace {namespace:?} not found")]
    NotFound { namespace: String },
    #[error("Unkown error occured {error:?}")]
    Unknown { error: Status },
}
