use crate::{args::OrkaCtlArgs, config::Config, display::Display};
use args::config::ConfigOverride;
use clap::Parser;
use handler::Handler;
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

mod args;
mod config;
mod display;
mod handler;
mod workloads;

lazy_static! {
    pub static ref APP_CONFIG: Arc<Mutex<Config>> = Config::new_wrapped();
    pub static ref DISPLAY: Display = Display {};
}

#[tokio::main]
async fn main() {
    let args = OrkaCtlArgs::parse();
    apply_config_overrides(&args.overrides);
    execute(args).await
}

/// Applies the pottential overrides on the global config object
fn apply_config_overrides(overrides: &ConfigOverride) {
    let mut config = Config::get_config_lock();
    if let Some(value) = overrides.api_fqdn.as_ref() {
        config.set_orka_url(value)
    }
    if let Some(value) = overrides.api_port {
        config.set_orka_port(value)
    }
}

/// Call the proper handler function
async fn execute(args: OrkaCtlArgs) {
    let handler = Handler::new();
    match args.command {
        crate::args::CommandType::Config(config_type) => match config_type.command {
            crate::args::config::ConfigCommandType::Get(config) => handler.get_config_value(config),
            crate::args::config::ConfigCommandType::Set(config) => handler.set_config_value(config),
        },
        crate::args::CommandType::Create(create_type) => match create_type.command {
            args::crud::CreateCommandType::Workload(workload) => {
                handler.create_workload(workload).await
            }
            args::crud::CreateCommandType::Instance(instance) => {
                handler.create_instance(instance).await
            }
        },
        crate::args::CommandType::Get(get_type) => match get_type.command {
            args::crud::GetCommandType::Workload(workload) => handler.get_workload(workload).await,
            args::crud::GetCommandType::Instance(instance) => handler.get_instance(instance).await,
        },
        crate::args::CommandType::Delete(delete_type) => match delete_type.command {
            args::crud::DeleteCommandType::Workload(workload) => {
                handler.delete_workload(workload).await
            }
            args::crud::DeleteCommandType::Instance(instance) => {
                handler.delete_instance(instance).await
            }
        },
    };
}
