use async_trait::async_trait;
use cni_plugin::{error::CniError, macaddr::MacAddr};
use futures::stream::TryStreamExt;
use log::info;
use netlink_packet_route::{
    address,
    nlas::link::{self, State},
    LinkMessage,
};
use rtnetlink::Handle;
use std::{net::IpAddr, thread::sleep, time::Duration};

use crate::links::utils::convert_to_ip;

#[derive(Clone)]
pub struct LinkAttrs {
    pub name: String,
    pub mtu: i64,
    // Let kernel use default txqueuelen; leaving it unset
    // means 0, and a zero-length TX queue messes up FIFO
    // traffic shapers which use TX queue length as the
    // default packet limit
    // #[derivative(Default(value = "-1"))]
    pub txqlen: i8,
    pub hardware_addr: Option<MacAddr>,
}

#[async_trait]
pub trait Link {
    async fn link_add(&self, handle: &Handle) -> Result<(), CniError>;

    async fn link_by_name(handle: &Handle, name: String) -> Result<LinkMessage, CniError> {
        let mut links = handle.link().get().match_name(name.clone()).execute();
        match links.try_next().await {
            Ok(Some(link)) => Ok(link),
            _ => {
                return Err(CniError::Generic(format!(
                    "Failed to get link {}. (fn link_addr_add)",
                    name
                )))
            }
        }
    }

    async fn link_addr_add(
        handle: &Handle,
        name: String,
        addr: IpAddr,
        prefix_len: u8,
    ) -> Result<(), CniError> {
        let mut links = handle.link().get().match_name(name.clone()).execute();
        match links.try_next().await {
            Ok(Some(link)) => handle
                .address()
                .add(link.header.index, addr, prefix_len)
                .execute()
                .await
                .map_err(|e| {
                    CniError::Generic(format!(
                        "Failed to add address {}/{} to {}. (fn link_addr_add) {}",
                        addr, prefix_len, name, e
                    ))
                }),
            _ => {
                return Err(CniError::Generic(format!(
                    "Failed to add address {}/{} to {}. (fn link_addr_add)",
                    addr, prefix_len, name
                )))
            }
        }
    }

    async fn link_set_master(
        handle: &Handle,
        veth_peer_name: String,
        br_name: String,
    ) -> Result<(), CniError> {
        let mut links = handle.link().get().match_name(br_name.clone()).execute();
        let master_index = match links.try_next().await {
            Ok(Some(link)) => link.header.index,
            _ => {
                return Err(CniError::Generic(format!(
                    "Cannot get link {} type bridge. (fn link_set_master)",
                    br_name
                )))
            }
        };

        let mut links = handle
            .link()
            .get()
            .match_name(veth_peer_name.clone())
            .execute();
        match links.try_next().await {
            Ok(Some(link)) => handle
                .link()
                .set(link.header.index)
                .master(master_index)
                .execute()
                .await
                .map_err(|e| {
                    CniError::Generic(format!(
                        "Failed to connect {} to bridge {}. (fn link_set_master) {}",
                        veth_peer_name, br_name, e
                    ))
                }),
            _ => Err(CniError::Generic(format!(
                "Failed to connect {} to bridge {}. (fn link_set_master)",
                veth_peer_name, br_name
            ))),
        }
    }

    async fn link_promisc_on(handle: &Handle, name: String) -> Result<(), CniError> {
        let mut links = handle.link().get().match_name(name.clone()).execute();
        match links.try_next().await {
            Ok(Some(link)) => handle
                .link()
                .set(link.header.index)
                .promiscuous(true)
                .execute()
                .await
                .map_err(|e| {
                    CniError::Generic(format!(
                        "Could not set promiscuous mode on for {}. (fn link_promisc_on) {}",
                        name, e
                    ))
                }),
            _ => Err(CniError::Generic(format!(
                "Could not set promiscuous mode on for {}. (fn link_promisc_on)",
                name
            ))),
        }
    }

    async fn link_set_up(handle: &Handle, name: String) -> Result<(), CniError> {
        let mut links = handle.link().get().match_name(name.clone()).execute();
        match links.try_next().await {
            Ok(Some(link)) => handle
                .link()
                .set(link.header.index)
                .up()
                .execute()
                .await
                .map_err(|e| {
                    CniError::Generic(format!("Could not set up {}. (fn link_set_up) {}", name, e))
                }),
            _ => Err(CniError::Generic(format!(
                "Could not set up {}. (fn link_set_up)",
                name
            ))),
        }
    }

    async fn link_check_oper_up(name: String) -> Result<(), CniError> {
        let (connection, handle, _) = rtnetlink::new_connection().unwrap();
        tokio::spawn(connection);

        let retries: Vec<i32> = vec![0, 50, 500, 1000, 1000];
        for (_idx, &sleep_duration) in retries.iter().enumerate() {
            sleep(Duration::from_millis(sleep_duration as u64));

            let host_veth: LinkMessage = Self::link_by_name(&handle, name.clone()).await?;
            let option_index: Option<usize> = host_veth
                .nlas
                .iter()
                .position(|nla| nla == &link::Nla::OperState(State::Up));

            if let Some(i) = option_index {
                if let Some(nla) = host_veth.nlas.get(i) {
                    info!(
                        "LINK BY NAME: is present ? {:?} : {:?}",
                        nla,
                        host_veth.nlas.get(i).unwrap()
                    );
                    break;
                } else {
                    info!("LINK BY NAME: is present NON : {:?}", host_veth.nlas);
                }
            } else {
                info!("LINK BY NAME: is present NON : {:?}", host_veth.nlas);
            }
        }

        Ok(())
    }

    async fn del_link_by_name_addr(handle: &Handle, name: String) -> Result<IpAddr, CniError> {
        let mut links = handle.link().get().match_name(name.clone()).execute();
        let addr: IpAddr = match links.try_next().await {
            Ok(Some(link)) => {
                let mut addresses = handle
                    .address()
                    .get()
                    .set_link_index_filter(link.header.index)
                    .execute();
                match addresses.try_next().await {
                    Ok(Some(addr)) => {
                        let addresses: Vec<u8> = addr
                            .nlas
                            .iter()
                            .filter_map(|nla| {
                                if let address::Nla::Address(addr) = nla.to_owned() {
                                    Some(addr)
                                } else {
                                    None
                                }
                            })
                            .flatten()
                            .collect();

                        convert_to_ip(addresses)
                    }
                    _ => Err(CniError::Generic(format!(
                        "Failed to get IP addresses for {}. (fn del_link_by_name_addr)",
                        name
                    ))),
                }
            }
            _ => Err(CniError::Generic(format!(
                "Failed to get IP addresses for {}. (fn del_link_by_name_addr)",
                name
            ))),
        }?;

        let mut links = handle.link().get().match_name(name.clone()).execute();
        match links.try_next().await {
            Ok(Some(link)) => handle
                .link()
                .del(link.header.index)
                .execute()
                .await
                .map_err(|e| {
                    CniError::Generic(format!(
                        "Failed to delete link {}. (fn del_link_by_name_addr) {}",
                        name, e
                    ))
                }),
            _ => Err(CniError::Generic(format!(
                "Failed to delete link {}. (fn del_link_by_name_addr)",
                name
            ))),
        }?;

        Ok(addr)
    }
}
