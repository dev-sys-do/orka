use crate::plugins::PluginsBin::{Bridge, HostLocal};
use cni_plugin::config::{IpamConfig, NetworkConfig};
use cni_plugin::error::CniError;
use serde_json::Value;
use std::collections::HashMap;

pub fn create_delegation_config(parent_config: NetworkConfig) -> Result<NetworkConfig, CniError> {
    let NetworkConfig {
        cni_version,
        name,
        args,
        prev_result,
        runtime,
        ..
    } = parent_config;

    let bridge_name = parent_config
        .specific
        .get("bridge")
        .ok_or(CniError::MissingField("bridge"))?
        .clone();

    let subnet = parent_config
        .specific
        .get("subnet")
        .ok_or(CniError::MissingField("subnet"))?
        .clone();

    Ok(NetworkConfig {
        cni_version,
        name,
        args,
        prev_result,
        runtime,
        // bridge delegate plugin
        plugin: Bridge.to_string(),
        specific: HashMap::from([
            ("bridge".to_string(), bridge_name),
            // ("isGateway".to_string(), Value::Bool(true)),
            ("isDefaultGateway".to_string(), Value::Bool(true)),
        ]),
        ip_masq: false,
        ipam: Some(IpamConfig {
            plugin: HostLocal.to_string(),
            specific: HashMap::from([("subnet".to_string(), subnet)]),
        }),
        // Not used by the bridge plugin
        dns: None,
    })
}
