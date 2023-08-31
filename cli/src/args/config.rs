use clap::{Parser, Subcommand};

/// Config related arguments
#[derive(Debug, Parser)]
pub struct ConfigType {
    #[clap(subcommand)]
    pub command: ConfigCommandType,
}

#[derive(Debug, Subcommand)]
pub enum ConfigCommandType {
    /// Get the specified resource
    Get(GetConfig),

    /// Set the specified resource
    Set(SetConfig),
}

#[derive(Debug, Parser)]
pub struct GetConfig {
    /// The resource to get
    pub resource: ConfigResource,
}

#[derive(Debug, Parser, Clone)]
pub struct SetConfig {
    /// The resource to set
    pub resource: ConfigResource,

    /// The value associated to it
    pub value: String,
}

/// List of available config resources
#[derive(Debug, Clone, clap::ValueEnum)]
pub enum ConfigResource {
    ApiFqdn,
    ApiPort,
}

/// Override arguments
#[derive(Debug, Parser, Clone)]
pub struct ConfigOverride {
    /// Config override: apiFqdn resource
    #[arg(long)]
    pub api_fqdn: Option<String>,

    /// Config override: apiPort resource
    #[arg(long)]
    pub api_port: Option<u16>,
}
