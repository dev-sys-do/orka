//! Scheduling gRPC service for the Orka controller.

use crate::managers::node_agent::manager::NodeAgentManager;
use crate::managers::workload::errors::WorkloadError;
use crate::managers::workload::manager::WorkloadManager;
use anyhow::Result;
use orka_proto::node_agent::workload_service_client::WorkloadServiceClient;
use orka_proto::node_agent::workload_signal::Signal;
use orka_proto::node_agent::{Workload, WorkloadSignal, WorkloadStatus as AgentWorkloadStatus};
use orka_proto::scheduler_controller::scheduling_service_server::SchedulingService;
use orka_proto::scheduler_controller::WorkloadStatus as ControllerWorkloadStatus;
use orka_proto::scheduler_controller::{Empty, SchedulingRequest, WorkloadInstance};
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::{Stream, StreamExt};
use tonic::Response;
use tonic::{Request, Status, Streaming};
use tracing::{event, Level};

/// Implementation of the `SchedulingService` gRPC service.
pub struct ControllerSchedulingSvc {
    /// The shared instance of the node agent manager.
    node_agent_manager: Arc<Mutex<NodeAgentManager>>,
    /// The shared instance of the workload manager.
    workload_manager: Arc<Mutex<WorkloadManager>>,
}

impl ControllerSchedulingSvc {
    /// Create a new `SchedulingService` gRPC service manager.
    pub fn new(
        node_agent_manager: Arc<Mutex<NodeAgentManager>>,
        workload_manager: Arc<Mutex<WorkloadManager>>,
    ) -> Self {
        Self {
            node_agent_manager,
            workload_manager,
        }
    }
}

#[tonic::async_trait]
impl SchedulingService for ControllerSchedulingSvc {
    type ScheduleStream =
        Pin<Box<dyn Stream<Item = Result<ControllerWorkloadStatus, Status>> + Send>>;

    /// Called by the controller when it requests to schedule a workload on a node. The scheduler
    /// responds by streaming status information about the workload.
    async fn schedule(
        &self,
        request: Request<SchedulingRequest>,
    ) -> Result<Response<Self::ScheduleStream>, Status> {
        // Get workload ready to send
        let scheduling_request = request.into_inner();
        let scheduling_workload = scheduling_request.workload.ok_or_else(|| {
            event!(
                Level::ERROR,
                "Schedule request received, but the workload was missing"
            );

            WorkloadError::InvalidWorkload("".to_string())
        })?;

        let workload = Workload::from(scheduling_workload);

        let (tx, rx) = mpsc::channel(8);
        let manager = self.node_agent_manager.lock().map_err(|err| {
            event!(
                Level::ERROR,
                workload_instance_id = workload.instance_id,
                error = %err,
                "Failed to acquire node manager, unable to proceed with workload scheduling"
            );

            Status::internal("Failed to retrieve node agent manager")
        })?;

        // Choose a fitting node agent
        // TODO: Do a clean architecture for code choosing a target
        let agent = manager.agents_iter().next().ok_or_else(|| {
            event!(
                Level::ERROR,
                workload_instance_id = workload.instance_id,
                "No node is registered to schedule a workload on"
            );

            Status::failed_precondition("No nodes in the cluster")
        })?;

        let workload_manager = Arc::clone(&self.workload_manager);
        let agent_id = agent.id().to_string();
        let agent_address = agent.grpc_url();

        tokio::spawn(async move {
            // Request the agent to create the workload
            let workload_instance_id = workload.instance_id.clone();

            let mut workload_status_stream = match Self::create_workload_on_agent(
                workload,
                agent_address,
            )
            .await
            {
                Ok(v) => v,
                Err(err) => {
                    event!(
                        Level::ERROR,
                        workload_instance_id,
                        error = %err,
                        "Failed to acquire node manager, unable to proceed with workload scheduling"
                    );

                    let _ = tx.send(Err(err)).await;
                    return;
                }
            };

            // Register workload and link to its supporting agent
            let res = match workload_manager.lock() {
                Ok(mut v) => {
                    v.add_instance(workload_instance_id.clone(), agent_id);
                    Ok(())
                }
                Err(err) => {
                    event!(
                        Level::ERROR,
                        workload_instance_id,
                        error = %err,
                        "Failed to acquire node manager, unable to register the workload instance locally"
                    );

                    Err(Status::internal(
                        "Could not register the instance in the scheduler",
                    ))
                }
            };

            if let Err(err) = res {
                let _ = tx.send(Err(err)).await;
                return;
            }

            while let Some(result) = workload_status_stream.next().await {
                // Ensure no errors in the stream
                let agent_workload_status = match result {
                    Ok(v) => v,
                    Err(err) => {
                        event!(
                            Level::ERROR,
                            workload_instance_id,
                            error = %err,
                            "An error occurred while processing a workload status stream"
                        );

                        let _ = tx
                            .send(Err(Status::internal(
                                "Could not retrieve workload status from agent",
                            )))
                            .await;

                        break;
                    }
                };

                // Translate the received workload status before sending it back
                let controller_workload_status =
                    ControllerWorkloadStatus::from(agent_workload_status);

                let res = tx.send(Ok(controller_workload_status)).await;

                if let Err(err) = res {
                    event!(
                        Level::ERROR,
                        workload_instance_id,
                        error = %err,
                        "An error occurred while sending a workload status"
                    );

                    break;
                }
            }
        });

        let output_stream = ReceiverStream::new(rx);
        Ok(Response::new(
            Box::pin(output_stream) as Self::ScheduleStream
        ))
    }

    /// Called by the controller to request a workload instance to be gracefully stopped.
    async fn stop(&self, request: Request<WorkloadInstance>) -> Result<Response<Empty>, Status> {
        self.handle_signal_request(request, Signal::Stop).await
    }

    /// Called by the controller to request a workload instance to be terminated.
    async fn destroy(&self, request: Request<WorkloadInstance>) -> Result<Response<Empty>, Status> {
        self.handle_signal_request(request, Signal::Kill).await
    }
}

impl ControllerSchedulingSvc {
    /// Handle the request of sending a signal to a workload instance.
    ///
    /// # Arguments
    ///
    /// * `request` - The request that was received.
    /// * `signal` - The signal to send to the workload instance.
    ///
    /// # Errors
    ///
    /// * The node agent address associated with the workload instance could not be found.
    /// * The workload signal request failed.
    async fn handle_signal_request(
        &self,
        request: Request<WorkloadInstance>,
        signal: Signal,
    ) -> Result<Response<Empty>, Status> {
        // Find the agent address associated with this workload instance
        let inner_request = request.into_inner();

        let agent_address = match self
            .find_agent_address_from_instance(inner_request.instance_id.clone())
        {
            Ok(v) => v,
            Err(err) => {
                event!(
                    Level::ERROR,
                    workload_instance_id = inner_request.instance_id,
                    ?signal,
                    error = %err,
                    "Unable to get the agent address of the workload instance, a signal could not be delivered"
                );

                return Err(err);
            }
        };

        // Send the signal request to the node agent
        let res =
            Self::send_workload_signal(inner_request.instance_id.clone(), signal, agent_address)
                .await;

        if let Err(err) = res {
            event!(
                Level::ERROR,
                workload_instance_id = inner_request.instance_id,
                ?signal,
                error = %err,
                "An error occurred while sending a signal request to a node agent"
            );

            return Err(err);
        }

        Ok(Response::new(Empty {}))
    }

    /// Find the address of the node agent that's running a workload instance.
    ///
    /// # Arguments
    ///
    /// * `instance_id` - The ID of the workload instance to find the associated node agent with.
    ///
    /// # Errors
    ///
    /// * The workload manager mutex cannot be locked.
    /// * The workload is unknown.
    /// * The node agent manager mutex cannot be locked.
    /// * The agent associated with the workload is unknown.
    fn find_agent_address_from_instance(&self, instance_id: String) -> Result<String, Status> {
        let workload_manager = self
            .workload_manager
            .lock()
            .map_err(|_| Status::internal("Failed to retrieve workload manager"))?;

        let agent_id = workload_manager
            .find_related_agent(&instance_id)
            .ok_or(Status::invalid_argument("Unknown workload"))?;

        let node_agent_manager = self
            .node_agent_manager
            .lock()
            .map_err(|_| Status::internal("Failed to retrieve node agent manager"))?;

        let agent = node_agent_manager
            .get_agent(agent_id)
            .ok_or(Status::internal("No agent corresponding to workload"))?;

        Ok(agent.grpc_url())
    }

    /// Send a workload creation request to a node agent, and return a
    /// stream containing status updates about the instance of this workload.
    ///
    /// # Arguments
    ///
    /// * `workload` - The workload to create on the agent.
    /// * `address` - The address to connect to the agent.
    ///
    /// # Errors
    ///
    /// * Unable to connect to the agent.
    /// * The agent couldn't fulfill the request.
    async fn create_workload_on_agent(
        workload: Workload,
        address: String,
    ) -> Result<Streaming<AgentWorkloadStatus>, Status> {
        let mut client = WorkloadServiceClient::connect(address)
            .await
            .map_err(|_| Status::internal("Failed to contact designated node agent"))?;

        let request = Request::new(workload);
        match client.create(request).await {
            Ok(stream) => Ok(stream.into_inner()),
            Err(err) => Err(Status::internal(format!(
                "Failed to retrieve node agent workload status. Agent sent: {} (code {})",
                err.message(),
                err.code()
            ))),
        }
    }

    /// Send a request to a node agent to send a signal to a workload instance.
    ///
    /// # Arguments
    ///
    /// * `instance_id` - The ID of the workload instance to send a signal to.
    /// * `signal` - The signal to send to the workload instance.
    /// * `address` - The address to connect to the agent.
    ///
    /// # Errors
    ///
    /// * Unable to connect to the agent.
    /// * The agent couldn't fulfill the request.
    async fn send_workload_signal(
        instance_id: String,
        signal: Signal,
        address: String,
    ) -> Result<(), Status> {
        let mut client = WorkloadServiceClient::connect(address)
            .await
            .map_err(|_| Status::internal("Failed to contact designated node agent"))?;

        let request = Request::new(WorkloadSignal {
            instance_id,
            signal: signal.into(),
        });

        match client.signal(request).await {
            Ok(_) => Ok(()),
            Err(_) => Err(Status::internal("Failed to send signal to node agent")),
        }
    }
}
