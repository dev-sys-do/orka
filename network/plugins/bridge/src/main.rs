use cni_plugin::config::NetworkConfig;
use cni_plugin::reply::{reply, SuccessReply};
use cni_plugin::{error::CniError, logger, Cni};
use tokio::runtime::Runtime;

fn main() {
    logger::install("bridge.log");

    if let Ok(runtime) = Runtime::new() {
        match Cni::load() {
            Cni::Add {
                ifname,
                netns,
                config,
                ..
            } => {
                runtime.block_on(async move {
                    let result: Result<SuccessReply, CniError> =
                        bridge::cmd_add(ifname, netns, config.clone()).await;
                    into_reply(result, &config).await;
                });
            }
            Cni::Del {
                ifname,
                netns,
                config,
                ..
            } => {
                runtime.block_on(async move {
                    let result: Result<SuccessReply, CniError> =
                        bridge::cmd_del(ifname, netns.unwrap(), config.clone()).await;
                    into_reply(result, &config).await;
                });
            }
            Cni::Check { config, .. } => {
                runtime.block_on(async move {
                    let result: Result<SuccessReply, CniError> = bridge::cmd_check().await;
                    into_reply(result, &config).await;
                });
            }
            Cni::Version(_) => unreachable!(),
        }
    }
}

async fn into_reply(result: Result<SuccessReply, CniError>, config: &NetworkConfig) {
    let NetworkConfig { cni_version, .. } = config;
    match result {
        Ok(success) => reply(success),
        Err(cni_error) => reply(cni_error.into_reply(cni_version.clone())),
    }
}
