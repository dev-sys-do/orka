pub mod delegation;
pub mod ipam;
pub mod links;
pub mod types;

use crate::types::NetworkConfigReference::*;
use cni_plugin::{
    config::NetworkConfig,
    error::CniError,
    reply::{Interface, IpamSuccessReply, SuccessReply},
    Command,
};
use links::bridge::Bridge;
use serde_json::json;
use std::{collections::HashMap, path::PathBuf};

pub async fn cmd_add(
    _container_id: String,
    ifname: String,
    netns: PathBuf,
    _path: Vec<PathBuf>,
    mut config: NetworkConfig,
) -> Result<SuccessReply, CniError> {
    // let mut success: bool = false;

    // Network configuration keys check
    if let Some(json!(true)) = config.specific.get(&IsDefaultGateway.to_string()) {
        config
            .specific
            .entry(IsGateway.to_string())
            .or_insert_with(|| json!(true));
    }

    let hairpin_mode: bool = config
        .specific
        .entry(HairpinMode.to_string())
        .or_insert(json!(false))
        .as_bool()
        .unwrap();
    let promisc_mode: bool = config
        .specific
        .entry(PromiscMode.to_string())
        .or_insert(json!(false))
        .as_bool()
        .unwrap();

    if hairpin_mode && promisc_mode {
        return Err(CniError::Generic(
            "[ORKANET ERROR]: Cannot set hairpin mode and promiscuous mode at the same time. (fn cmd_add)\n".to_string()
        ));
    }

    config
        .specific
        .entry(Mtu.to_string())
        .or_insert(json!(1500));
    config
        .specific
        .entry(Bridge.to_string())
        .or_insert(json!("cni0"));

    // Create bridge if missing
    let (br, br_interface): (Bridge, Interface) = Bridge::setup_bridge(config.clone()).await?;

    config
        .specific
        .entry(PreserveDefaultVlan.to_string())
        .or_insert(json!(false));

    let (host_interface, container_interface) =
        br.setup_veth(netns, ifname, config.clone()).await?;

    let ipam_result: IpamSuccessReply = ipam::exec_cmd(Command::Add, config.clone())
        .await
        .map_err(|e| CniError::Generic(format!("{:?}", e)))?;

    if ipam_result.ips.is_empty() {
        return Err(CniError::Generic(
            "IPAM plugin returned missing IP config".to_string(),
        ));
    }

    // Configure the container IP address(es)
    ipam::configure_iface(container_interface.name.clone(), ipam_result.clone()).await;

    let is_gw: bool = config
        .specific
        .get(&IsGateway.to_string())
        .and_then(|v| v.as_bool())
        .unwrap();

    if is_gw {
        // Bridge::ensure_addr()
    }

    Ok(SuccessReply {
        cni_version: config.cni_version,
        interfaces: Vec::from([br_interface, host_interface, container_interface]),
        ips: ipam_result.ips,
        routes: ipam_result.routes,
        dns: ipam_result.dns,
        specific: HashMap::new(),
    })
}

pub async fn cmd_check(
    _container_id: String,
    _ifname: String,
    _netns: PathBuf,
    _path: Vec<PathBuf>,
    _config: NetworkConfig,
) -> Result<(), CniError> {
    todo!();
}

pub async fn cmd_del(
    _container_id: String,
    _ifname: String,
    _netns: Option<PathBuf>,
    _path: Vec<PathBuf>,
    _config: NetworkConfig,
) -> Result<(), CniError> {
    todo!();
}
