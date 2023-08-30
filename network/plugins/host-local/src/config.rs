use std::str::FromStr;

use cni_plugin::config::NetworkConfig;
use ipnet::IpNet;

use crate::cni_error;

pub fn get_datadir_from_config(cni_version: &String, config: &NetworkConfig) -> Result<String, ()> {
    let get_opt = config.specific.get("dataDir");
    if let None = get_opt {
        cni_error::output_error(
            &"No dataDir field in config file".to_string(),
            &"".to_string(),
            cni_error::CNIErrorCode::InvalidNetworkConfig,
            &cni_version.clone()
        );
        return Err(());
    }

    let data_dir_value = get_opt.unwrap();
    let parse_opt = data_dir_value.as_str();
    if let None = parse_opt {
        cni_error::output_error(
            &"Invalid dataDir field in config file".to_string(),
            &"".to_string(),
            cni_error::CNIErrorCode::InvalidNetworkConfig,
            &cni_version.clone()
        );
        return Err(());
    }

    let data_dir: String = parse_opt.unwrap().to_string();
    return Ok(data_dir);
}

// To change this
pub fn get_subnet_from_config(cni_version: &String, config: &NetworkConfig) -> IpNet {
    let range_value = config.specific
        .get("ranges")
        .expect("Cannot get ranges field from config file")
        .as_array()
        .expect("Invalid ranges field in config file")
        .first()
        .expect("Cannot get first array of subnet in config file")
        .as_array()
        .expect("Invalid array of subnet in config file")
        .first()
        .expect("Invalid string provided as a subnet in config file")
        .get("subnet")
        .expect("Cannot get subnet in subnet object in config file")
        .as_str()
        .expect("Invalid subnet value provided in config file");
    let subnet: String = range_value.to_string();
    let net = IpNet::from_str(&subnet).expect("Invalid subnet format provided in config file");
    return net;
}

pub fn get_cni_version_from_config(config: &NetworkConfig) -> String {
    let version = config.cni_version.to_string();
    return version;
}