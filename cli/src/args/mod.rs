use clap::{command, Parser, Subcommand};

use self::{
    config::ConfigType,
    crud::{CreateType, DeleteType, GetType},
};

pub mod config;
pub mod crud;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[command(name = "orkactl")]
pub struct OrkaCtlArgs {
    #[clap(subcommand, name = "config")]
    pub command: CommandType,
}

#[derive(Debug, Subcommand)]
pub enum CommandType {
    /// View or edit the config
    Config(ConfigType),

    /// Create a resource from the accepted resource list
    Create(CreateType),

    /// Get a resource from the accepted resource list
    Get(GetType),

    /// Delete a resource from the accepted resource list
    Delete(DeleteType),
}