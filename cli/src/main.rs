use clap::Parser;
use handler::{
    create_instance, create_workload, delete_instance, delete_workload, get_config_value,
    get_instance, get_workload, set_config_value,
};

use crate::args::{CommandType, OrkaCtlArgs};

mod args;
mod config;
mod handler;

#[tokio::main]
async fn main() {
    println!("Hello, cli!");
    let config = config::get_config();
    println!("Your config:\n{:?}\n\n", config);
    let args = OrkaCtlArgs::parse();
    println!("{:?}", args);
    execute(args).await
}

/// Call the proper handler function
pub async fn execute(args: OrkaCtlArgs) {
    match args.command {
        crate::args::CommandType::Config(config_type) => match config_type.command {
            crate::args::config::ConfigCommandType::Get(get_config) => get_config_value(get_config),
            crate::args::config::ConfigCommandType::Set(set_config) => set_config_value(set_config),
        },
        crate::args::CommandType::Create(create_type) => match create_type.command {
            args::crud::CreateCommandType::Workload(workload) => create_workload(workload).await,
            args::crud::CreateCommandType::Instance(instance) => create_instance(instance).await,
        },
        crate::args::CommandType::Get(get_type) => match get_type.command {
            args::crud::GetCommandType::Workload(workload) => get_workload(workload).await,
            args::crud::GetCommandType::Instance(instance) => get_instance(instance).await,
        },
        crate::args::CommandType::Delete(delete_type) => match delete_type.command {
            args::crud::DeleteCommandType::Workload(workload) => delete_workload(workload).await,
            args::crud::DeleteCommandType::Instance(instance) => delete_instance(instance).await,
        },
    }
}
