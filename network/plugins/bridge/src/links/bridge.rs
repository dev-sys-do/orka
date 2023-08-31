use super::{
    link::{Link, LinkAttrs},
    veth::Veth,
};
use crate::{netns, types::NetworkConfigReference::*};
use async_trait::async_trait;
use cni_plugin::{config::NetworkConfig, error::CniError, macaddr::MacAddr, reply::Interface};
use futures::stream::TryStreamExt;
use log::info;
use rtnetlink::Handle;
use std::{net::IpAddr, path::PathBuf};

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
                        "Could not add link {} type bridge. (fn link_add)\n{}\n",
                        self.linkattrs.name, err
                    ))
                }),
        }
    }
}

impl Bridge {
    pub async fn setup_bridge(config: NetworkConfig) -> Result<(Self, Interface), CniError> {
        let (connection, handle, _) = rtnetlink::new_connection().unwrap();
        tokio::spawn(connection);

        let vlan_filtering: bool = config
            .specific
            .get(&Vlan.to_string())
            .and_then(|value| value.as_i64())
            .map(|i| i == 0 || config.specific.contains_key("vlanTrunk"))
            .unwrap_or(false);
        let promisc_mode: bool = config
            .specific
            .get(&PromiscMode.to_string())
            .and_then(|value| value.as_bool())
            .unwrap_or(false);
        let br_name: String = config
            .specific
            .get(&Bridge.to_string())
            .and_then(|value| value.as_str())
            .unwrap()
            .to_string();
        let mtu: i64 = config
            .specific
            .get(&Mtu.to_string())
            .and_then(|value| value.as_i64())
            .unwrap();

        let br: Bridge =
            Bridge::ensure_bridge(&handle, br_name, mtu, promisc_mode, vlan_filtering).await?;

        Ok((
            br.clone(),
            Interface {
                name: br.linkattrs.name,
                mac: br.linkattrs.hardware_addr,
                sandbox: String::new().into(),
            },
        ))
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

    pub async fn setup_veth(
        br_name: String,
        netns: PathBuf,
        ifname: String,
        config: NetworkConfig,
    ) -> Result<(Interface, Interface), CniError> {
        let mtu: i64 = config
            .specific
            .get(&Mtu.to_string())
            .and_then(|value| value.as_i64())
            .unwrap();

        //  config.args.get("MAC");
        let mac: Option<MacAddr> = Option::None;

        // Handle for host namespace
        let (connection_host, handle_host, _) = rtnetlink::new_connection().unwrap();
        tokio::spawn(connection_host);

        let handle_host_for_cont: Handle = handle_host.clone();
        let (host_veth_name, cont_veth) =
            netns::exec::<_, _, (String, Veth)>(netns.clone(), |host_ns_fd| async move {
                // Handle for container namespace
                let (connection_cont, handle_cont, _) = rtnetlink::new_connection().unwrap();
                tokio::spawn(connection_cont);

                // create the veth pair in the container
                Veth::setup_veth(
                    &handle_host_for_cont,
                    &handle_cont,
                    ifname,
                    mtu,
                    mac,
                    host_ns_fd,
                )
                .await
            })
            .await?;

        // connect host veth end to the bridge
        Self::link_set_master(&handle_host, host_veth_name.clone(), br_name).await?;

        // ? set hairpin mode ?
        // ? remove default vlan ?
        // ? Currently bridge CNI only support access port(untagged only) or trunk port(tagged only) ?

        let cont_iface = Interface {
            name: cont_veth.linkattrs.name,
            mac: Option::None,
            sandbox: netns,
        };
        let host_iface = Interface {
            name: host_veth_name,
            mac: Option::None,
            sandbox: PathBuf::from(format!("/proc/self/fd/{}", cont_veth.peer_namespace)),
        };

        Ok((host_iface, cont_iface))
    }

    pub async fn ensure_addr(
        name: String,
        gw_addr: IpAddr,
        prefix_len: u8,
        gw_is_ipv4: bool,
        force_address: bool,
    ) -> Result<(), CniError> {
        let (connection, handle, _) = rtnetlink::new_connection().unwrap();
        tokio::spawn(connection);

        if let Some(current_addr) =
            Self::link_get_addr(&handle, name.clone())
                .await
                .map_err(|_| {
                    CniError::Generic(format!(
                        "Failed to get current IP address for {}. (fn ensure_addr)",
                        name
                    ))
                })?
        {
            info!("CURRENT IP: {}", current_addr);

            if current_addr == gw_addr {
                return Ok(());
            }

            // Multiple IPv6 addresses are allowed on the bridge if the
            // corresponding subnets do not overlap. For IPv4 or for
            // overlapping IPv6 subnets, reconfigure the IP address if
            // forceAddress is true, otherwise throw an error.
            if current_addr.is_ipv4() {
                if force_address {
                    Bridge::link_delete_addr(&handle, name.clone()).await?;
                } else {
                    return Err(CniError::Generic(format!(
                        "{} already has an IP address different from {:?}",
                        name, gw_addr
                    )));
                }
            }
        }

        if gw_is_ipv4 {
            Bridge::link_addr_add(&handle, name, gw_addr, prefix_len).await
        } else {
            Err(CniError::Generic(format!(
                "Gateway address is not ipv4 : {}. (fn ensure_addr)",
                gw_addr
            )))
        }
    }
}
