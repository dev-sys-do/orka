use std::path::Path;

use ipnet::IpNet;
use json::object;
use crate::types;
use crate::allocator;
use crate::cni_error;


pub fn exec(command: &types::CNICommand) -> Result<String, ()> {
    let init_result = allocator::init_datadir(&command.data_dir.clone(), &command.cni_version.clone());
    if let Err(()) = init_result {
        return Err(());
    }

    let find_result = allocator::find_container_id(&command.data_dir.clone(), &command.container_id.clone(), &command.cni_version.clone());
    if let None = find_result {
        return Err(());
    }
    let already_exists = !find_result.unwrap().eq("");
    if already_exists {
        cni_error::output_error(
            &"This containerid is already registered".to_string(),
            &"".to_string(),
            cni_error::CNIErrorCode::InvalidNecessaryEnvVars,
            &command.cni_version.clone()
        );
        return Err(());
    }

    let subnet = command.subnet.clone();
    
    let get_result = allocator::get_new_address(&command.data_dir.clone(), &subnet.clone());
    if let Err(()) = get_result {
        return Err(());
    }
    let new_addr: IpNet = get_result.unwrap();
    let new_addr_str: String = new_addr.addr().to_string();
    let new_addr_path_str = command.data_dir.clone() + "/" + &new_addr_str;
    let new_addr_path = Path::new(&new_addr_path_str);
    
    let write_result = allocator::write_containerid(&new_addr_path, &command.container_id.clone(), &command.cni_version.clone());
    if let Err(()) = write_result {
        return Err(());
    }

    let success_result = get_success_result(&new_addr, &command);
    if let Err(()) = success_result {
        return Err(());
    }
    let success = success_result.unwrap();

    return Ok(success);
}

fn get_success_result(new_addr: &IpNet, command: &types::CNICommand) -> Result<String, ()> {

    let gateway_opt = allocator::get_last_address(&command.subnet.clone());
    if let None = gateway_opt {
        return Err(());
    }
    let gateway = gateway_opt.unwrap();

    let result = object! {
        cni_version: command.cni_version.clone(),
        ips: [
            object! {
                version: "4",
                address: new_addr.addr().to_string(),
                gateway: gateway.addr().to_string()
            }
        ],
        routes: [
            "0.0.0.0/0"
        ],
        dns: object!{
            nameservers: [ "8.8.8.8", "8.8.4.4" ]
        }
    };

    return Ok(result.to_string());
}