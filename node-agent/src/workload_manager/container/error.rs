use thiserror::Error;
use tonic::Status;

#[derive(Error, Debug)]
pub enum ContainerClientError {
    #[error("Socket {sock_path:?} not found")]
    ContainerdSocketNotFound { sock_path: String },
    #[error("gRPC with error code {status:?} occured")]
    GRPCError { status: Status },
}

pub fn into_tonic_status(error: ContainerClientError) -> tonic::Status {
    match error {
        ContainerClientError::GRPCError { status } => status,
        _ => tonic::Status::new(
            tonic::Code::Internal,
            format!("Failed to delete container: {:?}", error),
        ),
    }
}
