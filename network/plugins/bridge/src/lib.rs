pub mod links;

use cni_plugin::{config::NetworkConfig, error::CniError};
use links::bridge::Bridge;
use serde_json::json;
use std::iter::Iterator;
use std::path::PathBuf;

pub async fn cmd_add(
    _container_id: String,
    ifname: String,
    netns: PathBuf,
    _path: Vec<PathBuf>,
    mut config: NetworkConfig,
) -> Result<(), CniError> {
    // let mut success: bool = false;

    if let Some(json!(true)) = config.specific.get("isDefaultGateway") {
        config.specific.insert("isGateway".to_string(), json!(true));
    }

    let hairpin_mode: bool = config
        .specific
        .get("hairpinMode")
        .and_then(|value| value.as_bool())
        .unwrap_or(false);
    let promisc_mode: bool = config
        .specific
        .get("promiscMode")
        .and_then(|value| value.as_bool())
        .unwrap_or(false);
    if hairpin_mode && promisc_mode {
        return Err(CniError::Generic(
            "[ORKANET ERROR]: Cannot set hairpin mode and promiscuous mode at the same time. (fn cmd_add)\n".to_string()
        ));
    }

    let mtu: i64 = config
        .specific
        .get("mtu")
        .and_then(|value| value.as_i64())
        .unwrap_or(1500);
    if !config.specific.contains_key("mtu") {
        config.specific.insert("mtu".to_string(), json!(mtu));
    }

    let br: Bridge = match Bridge::setup_bridge(config.clone()).await {
        Ok(br) => br,
        Err(err) => return Err(err),
    };

    let vlan_id: Option<i64> = config.specific.get("vlan").and_then(|value| value.as_i64());
    let vlans = config
        .specific
        .get("vlanTrunk")
        .and_then(|value| value.as_array())
        .map(|array| {
            array
                .iter()
                .filter_map(|v| v.as_i64())
                .collect::<Vec<i64>>()
        });
    let preserve_default_vlan: bool = config
        .specific
        .get("preserveDefaultVlan")
        .and_then(|value| value.as_bool())
        .unwrap_or(false);
    let mac: Option<&str> = config
        .specific
        .get("preserveDefaultVlan")
        .and_then(|value| value.as_str());

    let (_host_interface, _container_interface) = match br
        .setup_veth(
            netns,
            ifname,
            mtu,
            hairpin_mode,
            vlan_id,
            vlans,
            preserve_default_vlan,
            mac,
        )
        .await
    {
        Ok(res) => res,
        Err(err) => return Err(err),
    };

    // netns, err := ns.GetNS(args.Netns)
    // if err != nil {
    // 	return fmt.Errorf("failed to open netns %q: %v", args.Netns, err)
    // }
    // defer netns.Close()

    // hostInterface, containerInterface, err := setupVeth(netns, br, args.IfName, n.MTU, n.HairpinMode, n.Vlan, n.vlans, n.PreserveDefaultVlan, n.mac)
    // if err != nil {
    // 	return err
    // }

    // // Assume L2 interface only
    // result := &current.Result{
    // 	CNIVersion: current.ImplementedSpecVersion,
    // 	Interfaces: []*current.Interface{
    // 		brInterface,
    // 		hostInterface,
    // 		containerInterface,
    // 	},
    // }

    Ok(())
}

pub async fn cmd_check(
    container_id: String,
    ifname: String,
    netns: PathBuf,
    path: Vec<PathBuf>,
    config: NetworkConfig,
) -> Result<(), CniError> {
    todo!();
}

pub async fn cmd_del(
    container_id: String,
    ifname: String,
    netns: Option<PathBuf>,
    path: Vec<PathBuf>,
    config: NetworkConfig,
) -> Result<(), CniError> {
    todo!();
}
