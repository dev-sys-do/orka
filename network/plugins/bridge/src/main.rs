use cni_plugin::{error::CniError, logger, Cni};

#[tokio::main]
async fn main() -> Result<(), CniError> {
    logger::install("bridge.log");

    match Cni::load() {
        Cni::Add {
            container_id,
            ifname,
            netns,
            path,
            config,
        } => {
            if let Err(e) = bridge::cmd_add(container_id, ifname, netns, path, config).await {
                println!("{}", e);
            }
        }
        Cni::Del {
            container_id,
            ifname,
            netns,
            path,
            config,
        } => {
            if let Err(e) = bridge::cmd_del(container_id, ifname, netns, path, config).await {
                println!("{}", e);
            }
        }
        Cni::Check {
            container_id,
            ifname,
            netns,
            path,
            config,
        } => {
            if let Err(e) = bridge::cmd_check(container_id, ifname, netns, path, config).await {
                println!("{}", e);
            }
        }
        Cni::Version(_) => {}
    }
    Ok(())
}
