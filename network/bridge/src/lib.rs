pub mod links;

use cni_plugin::{config::NetworkConfig, error::CniError};
use links::bridge::Bridge;
use std::path::PathBuf;

pub async fn cmd_add(
    container_id: String,
    ifname: String,
    netns: PathBuf,
    path: Vec<PathBuf>,
    mut config: NetworkConfig,
) -> Result<(), CniError> {
    let mut success: bool = false;

    if let Some(v) = config.specific.get(&"isDefaultGateway".to_string()) {
        config.specific.insert("isGateway".to_string(), v.clone());
    }

    let is_hairpin_mode: bool = config
        .specific
        .get("hairpinMode")
        .and_then(|value| value.as_bool())
        .map(|b: bool| b)
        .unwrap_or(false);
    let is_promisc_mode: bool = config
        .specific
        .get("promiscMode")
        .and_then(|value| value.as_bool())
        .map(|b: bool| b)
        .unwrap_or(false);

    if is_hairpin_mode && is_promisc_mode {
        return Err(CniError::Generic(
            "Cannot set hairpin mode and promiscuous mode at the same time".into(),
        ));
    }

    let br: Bridge = match Bridge::setup_bridge(config).await {
        Ok(br) => br,
        Err(err) => return Err(CniError::Generic(format!("{:?}", err))),
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
    // let mut success: bool = false;
    print!(
        "NETWORK CONFIG :\n{}",
        config.specific.get(&"bridge".to_string()).unwrap()
    );
    // let n = Self::load_net_conf()
    Ok(())
}

pub async fn cmd_del(
    container_id: String,
    ifname: String,
    netns: Option<PathBuf>,
    path: Vec<PathBuf>,
    config: NetworkConfig,
) -> Result<(), CniError> {
    // let mut success: bool = false;
    print!(
        "NETWORK CONFIG :\n{}",
        config.specific.get(&"bridge".to_string()).unwrap()
    );
    // let n = Self::load_net_conf()
    Ok(())
}
