pub mod allocator;
pub mod cni_error;
pub mod config;
pub mod types;
pub mod commands {
    pub mod cni_add;
    pub mod cni_del;
}


use cni_plugin::Cni;
use ipnet::{IpNet};

fn main() {
    match Cni::load() {
        Cni::Add { container_id, ifname, netns, path, config } => {
            let cni_version = config::get_cni_version_from_config(&config);

            let subnet: IpNet = config::get_subnet_from_config(&cni_version.clone(), &config);
            let get_result = config::get_datadir_from_config(&cni_version.clone(), &config);
            if let Err(()) = get_result {
                return;
            }
            let data_dir = get_result.unwrap();

            let command = types::CNICommand {
                container_id: container_id,
                ifname: ifname,
                netns: netns.to_str().unwrap().to_string(),
                data_dir: data_dir,
                subnet: subnet,
                cni_version: cni_version
            };
            
            let alloc_result = commands::cni_add::exec(&command);
            if let Ok(result) = alloc_result {
                print!("{}", result);
            }
        }
        Cni::Del { container_id, ifname, netns, path, config } => {
            let cni_version = config::get_cni_version_from_config(&config);

            let subnet: IpNet = config::get_subnet_from_config(&cni_version.clone(), &config);
            let get_result = config::get_datadir_from_config(&cni_version.clone(), &config);
            if let Err(()) = get_result {
                return;
            }
            let data_dir = get_result.unwrap();
            
            let mut netns_value = "".to_string();
            if let Some(pathbuf) = netns {
                netns_value = pathbuf.to_str().unwrap().to_string();
            }

            let command = types::CNICommand {
                container_id: container_id,
                ifname: ifname,
                netns: netns_value,
                data_dir: data_dir,
                subnet: subnet,
                cni_version: cni_version
            };

            let cmd_result = commands::cni_del::exec(&command);
            if let Err(()) = cmd_result {
                return;
            }

            //TODO return success json
        }
        Cni::Check { container_id, ifname, netns, path, config } => {
            println!("check");
        }
        Cni::Version(_) => unreachable!()
    }

}
