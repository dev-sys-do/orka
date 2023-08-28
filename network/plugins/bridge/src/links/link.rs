use async_trait::async_trait;
use cni_plugin::{error::CniError, macaddr::MacAddr};
use futures::stream::TryStreamExt;
use rtnetlink::Handle;
use std::net::IpAddr;

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
                .map_err(|err| {
                    CniError::Generic(format!(
                        "[ORKANET ERROR]: Failed to add address {} to {}. (fn link_addr_add)\n{}\n",
                        addr, name, err
                    ))
                }),
            _ => {
                return Err(CniError::Generic(format!(
                    "[ORKANET ERROR]: Failed to add address {} to {}. (fn link_addr_add)\n",
                    addr, name
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
                    "[ORKANET ERROR]: Cannot get bridge interface id: {}. (fn link_set_master)\n",
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
            Ok(Some(link)) => {
                handle
                    .link()
                    .set(link.header.index)
                    .master(master_index)
                    .execute()
                    .await
                    .map_err(|err| {
                        CniError::Generic(format!(
                            "[ORKANET ERROR]: Failed to connect {} to bridge {}. (fn link_set_master)\n{}\n",
                            veth_peer_name, br_name, err
                        ))
                    })
            }
            _ => Err(CniError::Generic(format!(
                "[ORKANET ERROR]: Failed to connect {} to bridge {}. (fn link_set_master)\n",
                veth_peer_name, br_name
            ))),
        }
    }

    async fn link_promisc_on(handle: &Handle, name: String) -> Result<(), CniError> {
        let mut links = handle.link().get().match_name(name.clone()).execute();
        match links.try_next().await {
            Ok(Some(link)) => {
                handle
                    .link()
                    .set(link.header.index)
                    .promiscuous(true)
                    .execute()
                    .await
                    .map_err(|err| {
                        CniError::Generic(format!(
                            "[ORKANET ERROR]: Could not set promiscuous mode on {}. (fn link_promisc_on)\n{}\n",
                            name, err
                        ))
                    })
            }
            _ => Err(CniError::Generic(format!(
                "[ORKANET ERROR]: Could not set promiscuous mode on {}. (fn link_promisc_on)\n",
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
                .map_err(|err| {
                    CniError::Generic(format!(
                        "[ORKANET ERROR]: Could not set up {}. (fn link_set_up)\n{}\n",
                        name, err
                    ))
                }),
            _ => Err(CniError::Generic(format!(
                "[ORKANET ERROR]: Could not set up {}. (fn link_set_up)\n",
                name
            ))),
        }
    }
}
