pub mod delegation;
pub mod ipam;
pub mod links;
pub mod netns;
pub mod route;
pub mod types;

use crate::types::NetworkConfigReference::*;
use cni_plugin::{
    config::NetworkConfig,
    error::CniError,
    reply::{Dns, Interface, SuccessReply},
    Command,
};
use links::{bridge::Bridge, link::Link, veth::Veth};
use serde_json::json;
use std::{collections::HashMap, net::IpAddr, path::PathBuf};

pub async fn cmd_add(
    ifname: String,
    netns: PathBuf,
    mut config: NetworkConfig,
) -> Result<SuccessReply, CniError> {
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
            "Cannot set hairpin mode and promiscuous mode at the same time. (fn cmd_add)"
                .to_string(),
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
    config
        .specific
        .entry(PreserveDefaultVlan.to_string())
        .or_insert(json!(false));

    // Create bridge only if missing
    let (br, br_interface): (Bridge, Interface) = Bridge::setup_bridge(config.clone()).await?;

    // Setup veth pair in container and in host
    let (host_interface, container_interface): (Interface, Interface) = Bridge::setup_veth(
        br.linkattrs.name.clone(),
        netns.clone(),
        ifname,
        config.clone(),
    )
    .await?;

    // Delegate to `host-local` plugin
    let ipam_result: SuccessReply = ipam::exec_cmd(Command::Add, config.clone()).await?;

    if ipam_result.ips.is_empty() {
        return Err(CniError::Generic(
            "IPAM plugin returned missing IP config.".to_string(),
        ));
    }

    // Last configuration for container interface
    netns::exec::<_, _, ()>(netns, |_| async {
        ipam::configure_iface(container_interface.name.clone(), ipam_result.clone()).await
    })
    .await?;

    let is_gw: bool = config
        .specific
        .get(&IsGateway.to_string())
        .and_then(|v| v.as_bool())
        .unwrap();

    if is_gw {
        // Bridge::ensure_addr().await?;
    }

    // Controle oper state is up
    Veth::link_check_oper_up(host_interface.name.clone()).await?;

    Ok(SuccessReply {
        cni_version: config.cni_version,
        interfaces: Vec::from([br_interface, host_interface, container_interface]),
        ips: ipam_result.ips,
        routes: ipam_result.routes,
        dns: ipam_result.dns,
        specific: HashMap::new(),
    })
}

pub async fn cmd_check() -> Result<SuccessReply, CniError> {
    todo!();
}

pub async fn cmd_del(
    ifname: String,
    netns: PathBuf,
    config: NetworkConfig,
) -> Result<SuccessReply, CniError> {
    if netns == PathBuf::from("") {
        let _: SuccessReply = ipam::exec_cmd(Command::Del, config.clone()).await?;
    }

    // There is a netns so try to clean up. Delete can be called multiple times
    // so don't return an error if the device is already removed.
    // If the device isn't there then don't try to clean up IP masq either.
    let _: IpAddr = match netns::exec::<_, _, IpAddr>(netns, |_| async {
        let (connection, handle, _) = rtnetlink::new_connection().unwrap();
        tokio::spawn(connection);

        Veth::del_link_by_name_addr(&handle, ifname).await
    })
    .await
    {
        Ok(addr) => addr,
        Err(e) => {
            let _: SuccessReply = ipam::exec_cmd(Command::Del, config.clone()).await?;
            return Err(e);
        }
    };

    // call ipam.ExecDel after clean up device in netns
    let _: SuccessReply = ipam::exec_cmd(Command::Del, config.clone()).await?;

    // if ipMasq {
    //     ipnet
    // }

    Ok(SuccessReply {
        cni_version: config.cni_version,
        interfaces: Vec::from([]),
        ips: Vec::new(),
        routes: Vec::new(),
        dns: Dns {
            nameservers: Vec::new(),
            domain: None,
            search: Vec::new(),
            options: Vec::new(),
        },
        specific: HashMap::new(),
    })
}
