mod commands;
mod delegation;
mod plugins;

use crate::commands::add::add_handler;
use crate::commands::del::del_handler;
use cni_plugin::error::CniError;
use cni_plugin::reply::{reply, ErrorReply, SuccessReply};
use cni_plugin::Cni;

pub async fn cni_run() {
    let result: Option<Result<SuccessReply, ErrorReply>> = match Cni::load() {
        Cni::Add { config, .. } => Some(add_handler(config).await),
        Cni::Del { config, .. } => Some(del_handler(config).await),
        // TODO check command
        Cni::Check { config, .. } => Some(Err(
            CniError::Debug(Box::new(config.clone())).into_reply(config.cni_version)
        )),
        // already included with `load()` method
        Cni::Version(_) => None,
    };

    match result {
        Some(Ok(success)) => reply(success),
        Some(Err(error)) => reply(error),
        _ => println!("{:?}", result),
    };
}
