use crate::workload_manager::container::metrics::metrics::metrics::Metrics;
use orka_proto::node_agent::workload_status::Resources;
use prost::{DecodeError, Message};
use prost_types::Any;

mod metrics {
    tonic::include_proto!("metrics");
}

fn any_to_metrics(any: &Any) -> Result<Metrics, DecodeError> {
    Metrics::decode(any.value.as_slice())
}

pub fn any_to_resource(any: &Any) -> Result<Resources, DecodeError> {
    match any_to_metrics(any) {
        Ok(m) => Ok(Resources {
            cpu: m.cpu.unwrap().usage_usec as i32,
            memory: m.memory.unwrap().usage as i32,
            disk: 0, // TODO!: implement for real
        }),
        Err(e) => Err(e),
    }
}
