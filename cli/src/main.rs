use clap::Parser;
use handler::Handler;
use std::sync::{Arc, Mutex};

//use crate::args::{CommandType, OrkaArgs};
use crate::workloads::file::read_file;
//mod args;
mod workloads;



lazy_static! {
    #[derive(Debug)]
    pub static ref APP_CONFIG: Arc<Mutex<Config>> = Config::new_wrapped();
    pub static ref DISPLAY: Display = Display {};
}

#[tokio::main]
async fn main() {
    println!("Hello, cli!");
    /*
    let args = OrkaArgs::parse();
    println!("{:?}", args);
    
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
    }
    */

    let filepath : String = String::from("examples/c.yaml");

    match read_file(filepath) {
        Ok(json) => println!("{:?}", json),
        Err(error) => println!("{:?}", error)
    }
}