use clap::Parser;

use crate::args::{CommandType, OrkaArgs};

mod args;
mod default_config;

fn main() {
    println!("Hello, cli!");
    let config = default_config::get_config();
    println!("Your config:\n{:?}\n\n", config);
    let args = OrkaArgs::parse();
    println!("{:?}", args);
    match args.command {
        CommandType::Config(_) => print!("config"),
        CommandType::Create(_) => print!("Create"),
        CommandType::Get(_) => print!("get"),
        CommandType::Delete(_) => print!("Delete"),
    }
}
