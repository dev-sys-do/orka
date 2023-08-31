use crate::{node_agent, scheduler_controller};

impl From<scheduler_controller::Workload> for node_agent::Workload {
    fn from(value: scheduler_controller::Workload) -> Self {
        Self {
            instance_id: value.instance_id,
            image: value.image,
            environment: value.environment,
            r#type: value.r#type,
            resource_limits: value
                .resource_limits
                .map(|r| node_agent::workload::Resources {
                    memory: r.memory,
                    cpu: r.cpu,
                    disk: r.disk,
                }),
        }
    }
}

impl From<node_agent::WorkloadStatus> for scheduler_controller::WorkloadStatus {
    fn from(value: node_agent::WorkloadStatus) -> Self {
        Self {
            instance_id: value.instance_id,
            status: value
                .status
                .map(|s| scheduler_controller::workload_status::Status {
                    code: s.code,
                    message: s.message,
                }),
            resource_usage: value.resource_usage.map(|r| {
                scheduler_controller::workload_status::Resources {
                    cpu: r.cpu,
                    memory: r.memory,
                    disk: r.disk,
                }
            }),
        }
    }
}
