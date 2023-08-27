use super::{
    link::{Link, LinkAttrs},
    veth::Veth,
};
use async_trait::async_trait;
use cni_plugin::{config::NetworkConfig, error::CniError};
use futures::stream::TryStreamExt;
use rtnetlink::{Handle, NetworkNamespace};
use std::path::PathBuf;

#[derive(Clone)]
pub struct Bridge {
    pub linkattrs: LinkAttrs,
    pub promisc_mode: bool,
    pub vlan_filtering: bool,
}

#[async_trait]
impl Link for Bridge {
    async fn link_add(&self, handle: &Handle) -> Result<(), CniError> {
        let mut links = handle
            .link()
            .get()
            .match_name(self.linkattrs.name.clone())
            .execute();
        match links.try_next().await {
            Ok(Some(_)) => Ok(()),
            _ => handle
                .link()
                .add()
                .bridge(self.linkattrs.name.clone())
                .execute()
                .await
                .map_err(|err| {
                    CniError::Generic(format!(
                        "[ORKANET ERROR]: Could not add bridge {}. (fn link_add)\n{}\n",
                        self.linkattrs.name, err
                    ))
                }),
        }
    }
}

impl Bridge {
    pub async fn setup_bridge(config: NetworkConfig) -> Result<Self, CniError> {
        let (connection, handle, _) = rtnetlink::new_connection().unwrap();
        tokio::spawn(connection);

        let vlan_filtering: bool = config
            .specific
            .get("vlan")
            .and_then(|value| value.as_i64())
            .map(|i| i == 0 || config.specific.contains_key("vlanTrunk"))
            .unwrap_or(false);
        let promisc_mode: bool = config
            .specific
            .get("promiscMode")
            .and_then(|value| value.as_bool())
            .unwrap_or(false);
        let br_name: String = config
            .specific
            .get("bridge")
            .and_then(|value| value.as_str())
            .unwrap_or("cni0")
            .to_string();
        let mtu: i64 = config
            .specific
            .get("mtu")
            .and_then(|value| value.as_i64())
            .unwrap_or(1500);

        // create bridge if necessary
        Bridge::ensure_bridge(&handle, br_name, mtu, promisc_mode, vlan_filtering).await
    }

    async fn ensure_bridge(
        handle: &Handle,
        br_name: String,
        mtu: i64,
        promisc_mode: bool,
        vlan_filtering: bool,
    ) -> Result<Self, CniError> {
        let br: Bridge = Self {
            linkattrs: LinkAttrs {
                name: br_name,
                mtu,
                txqlen: -1,
                hardware_addr: Option::None,
            },
            promisc_mode,
            vlan_filtering,
        };

        br.link_add(handle).await?;

        if br.promisc_mode {
            Bridge::link_promisc_on(handle, br.linkattrs.name.clone()).await?;
        }

        // Re-fetch link to read all attributes and if it already existed,
        // ensure it's really a bridge with similar configuration
        // Self::bridge_by_name(handle, br.linkattrs.name.clone()).await;

        Bridge::link_set_up(handle, br.linkattrs.name.clone()).await?;

        Ok(br)
    }

    // async fn bridge_by_name(handle: &Handle, br_name: String) -> u32 {
    //     let mut links = handle.link().get().match_name(br_name.clone()).execute();
    //     match links.try_next().await {
    //         Ok(Some(link)) => {
    //             println!("[ORKANET]: `FLAGS` {}", link.header.flags);
    //             link.header.index
    //         }
    //         Ok(None) => panic!("[ORKANET]: Could not lookup {}.", br_name),
    //         Err(_) => panic!("[ORKANET]: Could not lookup {}.", br_name),
    //     }
    //     // Maybe check if it's bridge
    // }

    pub async fn setup_veth(
        &self,
        netns: PathBuf,
        ifname: String,
        mtu: i64,
        _hairpin_mode: bool,
        _vlan_id: Option<i64>,
        _vlans: Option<Vec<i64>>,
        _preserve_default_vlan: bool,
        mac: Option<&str>,
    ) -> Result<(String, String), CniError> {
        // Handle for host
        let (connection_host, handle_host, _) = rtnetlink::new_connection().unwrap();
        tokio::spawn(connection_host);

        // WARNING ! [Change namespace from host to container]
        NetworkNamespace::unshare_processing(String::from(netns.to_string_lossy())).map_err(|err| CniError::Generic(format!(
                "[ORKANET ERROR]: Could not unshare processing to netns {:?}. (fn setup_veth)\n{}\n",
                netns, err
            )))?;

        // Handle for container
        let (connection_cont, handle_cont, _) = rtnetlink::new_connection().unwrap();
        tokio::spawn(connection_cont);

        // create the veth pair in the container and move host end into host netns
        let host_ns: PathBuf = PathBuf::from("/proc/1/ns/net");
        let (host_veth_name, cont_veth) =
            Veth::setup_veth(&handle_host, &handle_cont, ifname, mtu, mac, host_ns).await?;
        // connect host veth end to the bridge
        Self::link_set_master(
            &handle_host,
            host_veth_name.clone(),
            self.linkattrs.name.clone(),
        )
        .await?;

        // ? set hairpin mode ?
        // ? remove default vlan ?
        // ? Currently bridge CNI only support access port(untagged only) or trunk port(tagged only) ?

        Ok((host_veth_name, cont_veth.linkattrs.name))
    }

    // Attach ipv4 to interface
    // async fn attach_ip(handle: &Handle, ifname: &str, ipaddr: IpAddr, mask: u8) {
    //     let mut links = handle.link().get().match_name(ifname.to_owned()).execute();
    //     match links.try_next().await {
    //         Ok(Some(link)) => {
    //             handle
    //                 .address()
    //                 .add(link.header.index, ipaddr, mask)
    //                 .execute()
    //                 .await
    //                 // .map_err(|e| println!("[ORKANET]: ERROR {}", e))
    //                 .unwrap();
    //         }
    //         Ok(None) => !panic!("[ORKANET]: Error on on attach ip {}.", ifname),
    //         Err(_) => !panic!("[ORKANET]: Error on on attach ip {}.", ifname),
    //     }
    // }
}
