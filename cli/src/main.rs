use clap::Parser;

use crate::args::{CommandType, OrkaArgs};

mod args;

fn main() {
    println!("Hello, cli!");
    let args = OrkaArgs::parse();
    println!("{:?}", args);
    match args.command {
        CommandType::Config(_) => print!("config"),
        CommandType::Create(_) => print!("Create"),
        CommandType::Get(_) => print!("get"),
        CommandType::Delete(_) => print!("Delete"),
    }
}
