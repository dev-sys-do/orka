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


pub fn get_subnet_from_config(cni_version: &String, config: &NetworkConfig) -> Option<IpNet> {

    let ranges_value = config.specific.get("ranges");

    if ranges_value.is_none() {
        cni_error::output_error(
            &"Cannot get ranges field from config file".to_string(),
            &"".to_string(),
            cni_error::CNIErrorCode::InvalidNetworkConfig,
            cni_version
        );
        return None;
    }

    let ranges_array = ranges_value.unwrap().as_array();
    if ranges_array.is_none() {
        cni_error::output_error(
            &"Invalid ranges field in config file".to_string(),
            &"".to_string(),
            cni_error::CNIErrorCode::InvalidNetworkConfig,
            cni_version
        );
        return None;
    }

    let subnet_array_value = ranges_array.unwrap().first();
    if subnet_array_value.is_none() {
        cni_error::output_error(
            &"Cannot get first array of subnet in config file".to_string(),
            &"".to_string(),
            cni_error::CNIErrorCode::InvalidNetworkConfig,
            cni_version
        );
        return None;
    }

    let subnet_array = subnet_array_value.unwrap().as_array();
    if subnet_array.is_none() {
        cni_error::output_error(
            &"Invalid array of subnet in config file".to_string(),
            &"".to_string(),
            cni_error::CNIErrorCode::InvalidNetworkConfig,
            cni_version
        );
        return None;
    }

    let subnet_obj = subnet_array.unwrap().first();
    if subnet_obj.is_none() {
        cni_error::output_error(
            &"Cannot get subnet element object in config file".to_string(),
            &"".to_string(),
            cni_error::CNIErrorCode::InvalidNetworkConfig,
            cni_version
        );
        return None;
    }

    let subnet = subnet_obj.unwrap().get("subnet");
    if subnet.is_none() {
        cni_error::output_error(
            &"Cannot get subnet in subnet object in config file".to_string(),
            &"".to_string(),
            cni_error::CNIErrorCode::InvalidNetworkConfig,
            cni_version
        );
        return None;
    }

    let subnet_str = subnet.unwrap().as_str();
    if subnet_str.is_none() {
        cni_error::output_error(
            &"Invalid subnet string provided in config file".to_string(),
            &"".to_string(),
            cni_error::CNIErrorCode::InvalidNetworkConfig,
            cni_version
        );
        return None;
    }

    let subnet: String = subnet_str.unwrap().to_string();
    let from_str_result = IpNet::from_str(&subnet);
    if let Err(error) = from_str_result {
        cni_error::output_error(
            &"Invalid subnet format provided in config file".to_string(),
            &error.to_string(),
            cni_error::CNIErrorCode::InvalidNetworkConfig,
            cni_version
        );
        return None;
    }

    return Some(from_str_result.unwrap());
}

pub fn get_cni_version_from_config(config: &NetworkConfig) -> String {
    let version = config.cni_version.to_string();
    return version;
}