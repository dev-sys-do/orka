pub mod node_agent {
    tonic::include_proto!("node_agent");
}

pub mod scheduler_agent {
    tonic::include_proto!("scheduler.agent");
}

pub mod scheduler_controller {
    tonic::include_proto!("scheduler.controller");
}

pub mod scheduler;
