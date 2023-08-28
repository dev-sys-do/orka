use crate::delegation::delegate;
use cni_plugin::{
    config::NetworkConfig,
    error::CniError,
    reply::{ErrorReply, IpamSuccessReply},
    Command,
};
use std::collections::HashMap;

pub async fn exec_cmd<'a>(
    cmd: Command,
    config: NetworkConfig,
) -> Result<IpamSuccessReply, ErrorReply<'a>> {
    let cni_version = config.cni_version.clone();

    delegate::<IpamSuccessReply>(
        "host-local",
        cmd,
        &create_delegation_config(config).map_err(|e| e.into_reply(cni_version.clone()))?,
    )
    .await
    .map_err(|e| e.into_reply(cni_version))
}

pub fn create_delegation_config(parent_config: NetworkConfig) -> Result<NetworkConfig, CniError> {
    let NetworkConfig {
        cni_version,
        name,
        args,
        prev_result,
        runtime,
        ipam,
        ..
    } = parent_config;

    Ok(NetworkConfig {
        cni_version,
        name,
        args,
        prev_result,
        runtime,
        plugin: "host-local".to_string(),
        specific: HashMap::new(),
        ip_masq: false,
        ipam,
        dns: None,
    })
}

pub async fn configure_iface(_ifname: String, _res: IpamSuccessReply) {}
