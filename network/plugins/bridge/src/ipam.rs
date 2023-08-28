use crate::{
    delegation::delegate,
    links::{link::Link, veth::Veth},
    route,
};
use cni_plugin::{
    config::NetworkConfig,
    error::CniError,
    reply::{ErrorReply, IpamSuccessReply},
    Command,
};
use std::{collections::HashMap, net::IpAddr};

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

pub async fn configure_iface(ifname: String, res: IpamSuccessReply) -> Result<(), CniError> {
    let (connection, handle, _) = rtnetlink::new_connection().unwrap();
    tokio::spawn(connection);

    if let Some(ipc) = res.ips.get(0) {
        Veth::link_addr_add(
            &handle,
            ifname.clone(),
            ipc.address.ip(),
            ipc.address.prefix(),
        )
        .await
        .map_err(CniError::from)?;
    }

    Veth::link_set_up(&handle, ifname.clone()).await?;

    if let Some(ipc) = res.ips.get(0) {
        if let Some(IpAddr::V4(gw_addr)) = ipc.gateway {
            route::route_add_default(&handle, gw_addr)
                .await
                .map_err(CniError::from)?;
        } else {
            return Err(CniError::Generic(format!(
                "Failed to convert IpAddr to Ipv4Addr for adding default route to ifname: {}",
                ifname
            )));
        }
    }

    Ok(())
}
