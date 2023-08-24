use crate::delegation::delegate;
use crate::plugins::PluginsBin::{Bridge, HostLocal};
use cni_plugin::config::IpamConfig;
use cni_plugin::error::CniError;
use cni_plugin::{
    config::NetworkConfig,
    reply::{ErrorReply, SuccessReply},
    Command,
};
use serde_json::Value;
use std::collections::HashMap;

pub async fn add_handler<'a>(config: NetworkConfig) -> Result<SuccessReply, ErrorReply<'a>> {
    let cni_version = config.cni_version;

    // get subnet value from 10-orknet.conf
    let subnet = config
        .specific
        .get("subnet")
        .ok_or(CniError::MissingField("subnet").into_reply(cni_version.clone()))?;

    let bridge_name = config
        .specific
        .get("bridge")
        .ok_or(CniError::MissingField("bridge").into_reply(cni_version.clone()))?;

    let result_bridge = delegate::<SuccessReply>(
        &Bridge.to_string(),
        Command::Add,
        &NetworkConfig {
            cni_version: cni_version.clone(),
            name: config.name,
            plugin: Bridge.to_string(),
            specific: HashMap::from([
                ("bridge".to_string(), bridge_name.clone()),
                // ("isGateway".to_string(), Value::Bool(true)),
                ("isDefaultGateway".to_string(), Value::Bool(true)),
            ]),
            ip_masq: false,
            ipam: Some(IpamConfig {
                plugin: HostLocal.to_string(),
                specific: HashMap::from([("subnet".to_string(), subnet.clone())]),
            }),
            prev_result: None,
            args: config.args,
            // Not used by the bridge plugin
            dns: None,
            runtime: None,
        },
    )
    .await
    .map_err(|e| e.into_reply(cni_version.clone()))?;

    Ok(result_bridge)
}
