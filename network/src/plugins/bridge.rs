use futures::stream::TryStreamExt;
use rtnetlink::{Error, Handle};
use std::net::IpAddr;

#[allow(unused_variables)]
pub struct Bridge {
    ifname: String,
    ipaddr: IpAddr,
    mask: u8,
}

impl Bridge {
    /// Create a new `Bridge` instance
    pub async fn build(ifname: &str, ipaddr: IpAddr, mask: u8) -> Result<(Self), Error> {
        let (connection, handle, _) = rtnetlink::new_connection().unwrap();
        tokio::spawn(connection);

        let mut links = handle.link().get().match_name(ifname.to_string()).execute();
        match links.try_next().await {
            Ok(Some(_)) => !panic!("[ORKANET]: Interface {} already exists.", ifname),
            _ => {
                handle
                    .link()
                    .add()
                    .bridge(ifname.to_owned())
                    .execute()
                    .await
                    .map_err(|e| println!("[ORKANET]: ERROR {}", e))
                    .unwrap();
            }
        };

        Bridge::attach_ip(&handle, ifname, ipaddr, mask).await;
        Bridge::set_link_up(&handle, ifname).await;

        Ok(Bridge {
            ifname: ifname.to_string(),
            ipaddr,
            mask,
        })
    }

    /// Attach ipv4 to interface
    async fn attach_ip(handle: &Handle, ifname: &str, ipaddr: IpAddr, mask: u8) {
        let mut links = handle.link().get().match_name(ifname.to_owned()).execute();
        match links.try_next().await {
            Ok(Some(link)) => {
                handle
                    .address()
                    .add(link.header.index, ipaddr, mask)
                    .execute()
                    .await
                    // .map_err(|e| println!("[ORKANET]: ERROR {}", e))
                    .unwrap();
            }
            Ok(None) => !panic!("[ORKANET]: Error on on attach ip {}.", ifname),
            Err(_) => !panic!("[ORKANET]: Error on on attach ip {}.", ifname),
        }
    }

    /// Up interface
    async fn set_link_up(handle: &Handle, ifname: &str) {
        let mut links = handle.link().get().match_name(ifname.to_owned()).execute();
        match links.try_next().await {
            Ok(Some(link)) => {
                handle
                    .link()
                    .set(link.header.index)
                    .up()
                    .execute()
                    .await
                    // .map_err(|e| println!("[ORKANET]: ERROR {}", e))
                    .unwrap();
            }
            Ok(None) => !panic!("[ORKANET]: Error on set link up {}.", ifname),
            Err(_) => !panic!("[ORKANET]: Error on set link up {}.", ifname),
        }
    }

    /// Assign veth
    pub async fn assign_veth(handle: &Handle, ifname_bridge: &str, ifname_veth: &str) {
        let mut links = handle
            .link()
            .get()
            .match_name(ifname_bridge.to_owned())
            .execute();

        let master_index = match links.try_next().await {
            Ok(Some(link)) => link.header.index,
            _ => !panic!("[ORKANET]: Error get index master {}.", ifname_bridge),
        };

        let mut links = handle
            .link()
            .get()
            .match_name(ifname_veth.to_owned())
            .execute();
        match links.try_next().await {
            Ok(Some(link)) => {
                handle
                    .link()
                    .set(link.header.index)
                    .master(master_index)
                    .execute()
                    .await
                    .unwrap();
            }
            Ok(None) => !panic!(
                "[ORKANET]: Error assign veth {} to master {}.",
                ifname_veth, ifname_bridge
            ),
            Err(_) => !panic!(
                "[ORKANET]: Error assign veth {} to master {}.",
                ifname_veth, ifname_bridge
            ),
        }
    }
}
