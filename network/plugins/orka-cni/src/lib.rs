mod delegate_conf;
mod delegation;
mod plugins;

use crate::delegate_conf::create_delegation_config;
use crate::delegation::delegate;
use crate::plugins::PluginsBin::Bridge;
use cni_plugin::config::NetworkConfig;
use cni_plugin::reply::{reply, ErrorReply, SuccessReply};
use cni_plugin::{Cni, Command};

pub async fn run() {
    let result: Option<Result<SuccessReply, ErrorReply>> = match Cni::load() {
        Cni::Add { config, .. } => Some(command_handler(Command::Add, config).await),
        Cni::Del { config, .. } => Some(command_handler(Command::Del, config).await),
        Cni::Check { config, .. } => Some(command_handler(Command::Check, config).await),
        // already included with `load()` method
        Cni::Version(_) => None,
    };

    match result {
        Some(Ok(success)) => reply(success),
        Some(Err(error)) => reply(error),
        _ => {}
    };
}

/// Generic command handler
/// (only the command needs to change for now)
async fn command_handler<'a>(
    command: Command,
    config: NetworkConfig,
) -> Result<SuccessReply, ErrorReply<'a>> {
    let cni_version = config.cni_version.clone();

    delegate::<SuccessReply>(
        &Bridge.to_string(),
        command,
        &create_delegation_config(config).map_err(|e| e.into_reply(cni_version.clone()))?,
    )
    .await
    .map_err(|e| e.into_reply(cni_version))
}
