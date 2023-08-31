
use std::{fs::{self, File}, path::Path, io::Write, str::FromStr, net::{Ipv4Addr, IpAddr}};
use ipnet::IpNet;
use crate::cni_error;

pub fn get_last_address(subnet: &IpNet) -> Option<IpNet> {

    let last_host_opt = Iterator::max(subnet.hosts());
    last_host_opt?;
    
    let address = IpNet::from(last_host_opt.unwrap());

    Some(address)
}

pub fn find_container_id(data_dir: &String, containerid: &String, cni_version: &String) -> Option<String> {

    let read_result = fs::read_dir(data_dir.clone());
    if let Err(error) = read_result {
        cni_error::output_error(
            &"Failed to read data directory".to_string(),
            &error.to_string(),
            cni_error::CNIErrorCode::IOFailure,
            &cni_version.clone()
        );
        return None;
    }

    let read = read_result.unwrap();
    for path in read {
        if path.is_err() {
            continue;
        }
        let entry = path.unwrap();
        let metadata_result = entry.metadata();
        if let Err(_) = metadata_result {
            continue;
        }
        let metadata = metadata_result.unwrap();
        if !metadata.is_file() {
            continue;
        }

        let os_file_name = entry.file_name();
        let opt_file_name = os_file_name.to_str();
        if opt_file_name.is_none() {
            continue;
        }
        let file_name = opt_file_name.unwrap().to_string();
        let file_path = entry.path();
        let read_result = fs::read_to_string(file_path);
        if read_result.is_err() {
            continue;
        }
        let content = read_result.unwrap();
        if !content.eq(&containerid.clone()) {
            continue;
        }
        
        let ip_addr_result = Ipv4Addr::from_str(&file_name.clone());
        if let Err(error) = ip_addr_result {
            cni_error::output_error(
                &"Invalid ip address registered in data directory".to_string(),
                &error.to_string(),
                cni_error::CNIErrorCode::FailedToDecodeContent,
                &cni_version.clone()
            );
            return None;
        }

        return Some(file_name.clone());
    }

    Some("".to_string())// Used to detect if the container id is registered
}

pub fn remove_file(path: &Path, cni_version: &String) -> Result<(), ()> {

    let remove_result = fs::remove_file(path);
    if let Err(error) = remove_result {
        cni_error::output_error(
            &"Failed to remove ip address file".to_string(),
            &error.to_string(),
            cni_error::CNIErrorCode::IOFailure,
            &cni_version.clone()
        );
        return Err(());
    }

    Ok(())
}

pub fn write_containerid(addr_path: &Path, containerid: &String, cni_version: &String) -> Result<(), ()> {
    let create_result = File::create(addr_path.clone());
    if let Err(error) = create_result {
        cni_error::output_error(
            &"Failed to create registered ip address file".to_string(),
            &error.to_string(),
            cni_error::CNIErrorCode::IOFailure,
            &cni_version.clone()
        );
        return Err(());
    }

    let mut file: File = create_result.unwrap();
    let write_result = file.write_all(containerid.as_bytes());
    if let Err(error) = write_result {
        cni_error::output_error(
            &"Failed to write container id to ip address file".to_string(),
            &error.to_string(),
            cni_error::CNIErrorCode::IOFailure,
            &cni_version.clone()
        );
        return Err(());
    }

    Ok(())
}

pub fn get_new_address(data_dir: &String, subnet: &IpNet) -> Result<IpNet, ()> {

    for host in subnet.hosts() {
        if is_allocated_address(data_dir, &host) {
            continue;
        }
        let ip_address = IpNet::from(host);
        return Ok(ip_address);
    }

    
    Err(())
}

pub fn is_allocated_address(datadir: &String, address: &IpAddr) -> bool {

    let path: String = datadir.clone() + "/" + &address.to_string();
    if fs::metadata(path.clone()).is_err() {
        return false;
    }

    true
}

pub fn init_datadir(data_dir: &String, cni_version: &String) -> Result<(), ()> {
    if fs::metadata(data_dir.clone()).is_err() {
        let createdir_result = fs::create_dir_all(data_dir.clone());
        match createdir_result {
            Ok(()) => {
                return Ok(());
            }
            Err(error) => {
                cni_error::output_error(
                    &"Failed to create dataDir folder".to_string(),
                    &error.to_string(),
                    cni_error::CNIErrorCode::IOFailure,
                    &cni_version.clone()
                );
                return Err(());
            }
        }
    }
    Ok(())
}