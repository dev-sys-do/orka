use super::link::{Link, LinkAttrs};
use super::utils;
use async_trait::async_trait;
use cni_plugin::error::CniError;
use cni_plugin::macaddr::MacAddr;
use futures::stream::TryStreamExt;
use rtnetlink::Handle;

#[derive(Clone)]
pub struct Veth {
    pub linkattrs: LinkAttrs,
    pub peer_name: String,
    pub peer_namespace: i32,
}

#[async_trait]
impl Link for Veth {
    async fn link_add(&self, handle: &Handle) -> Result<(), CniError> {
        let mut links = handle
            .link()
            .get()
            .match_name(self.linkattrs.name.clone())
            .execute();
        match links.try_next().await {
            Ok(Some(_)) => Err(CniError::Generic(format!(
                "Container veth name provided `{}` already exists. (fn link_add)",
                self.linkattrs.name
            ))),
            _ => handle
                .link()
                .add()
                .veth(self.linkattrs.name.clone(), self.peer_name.clone())
                .execute()
                .await
                .map_err(|e| {
                    CniError::Generic(format!(
                        "Failed to add veth pair: {} and {} (peer). (fn link_add) {}",
                        self.linkattrs.name, self.peer_name, e
                    ))
                }),
        }?;

        let mut links = handle
            .link()
            .get()
            .match_name(self.peer_name.clone())
            .execute();
        match links.try_next().await {
            Ok(Some(link)) => handle
                .link()
                .set(link.header.index)
                .setns_by_fd(self.peer_namespace)
                .execute()
                .await
                .map_err(|e| {
                    CniError::Generic(format!(
                        "Failed to set {} (veth peer) in host namespace. (fn link_add) {}",
                        self.peer_name, e
                    ))
                }),
            _ => Err(CniError::Generic(format!(
                "Failed to set {} (veth peer) in host namespace. (fn link_add)",
                self.peer_name
            ))),
        }
    }
}

impl Veth {
    // SetupVeth sets up a pair of virtual ethernet devices.
    // Call SetupVeth from inside the container netns.  It will create both veth
    // devices and move the host-side veth into the provided hostNS namespace.
    // On success, SetupVeth returns (hostVeth, containerVeth, nil)
    pub async fn setup_veth(
        handle_host: &Handle,
        handle_cont: &Handle,
        cont_veth_name: String,
        mtu: i64,
        cont_veth_mac: Option<MacAddr>,
        host_ns_fd: i32,
    ) -> Result<(String, Self), CniError> {
        Self::setup_veth_with_name(
            handle_host,
            handle_cont,
            cont_veth_name,
            String::new(),
            mtu,
            cont_veth_mac,
            host_ns_fd,
        )
        .await
    }

    async fn setup_veth_with_name(
        handle_host: &Handle,
        handle_cont: &Handle,
        cont_veth_name: String,
        host_veth_name: String,
        mtu: i64,
        cont_veth_mac: Option<MacAddr>,
        host_ns_fd: i32,
    ) -> Result<(String, Self), CniError> {
        let (host_veth_name, cont_veth) = Self::make_veth(
            handle_cont,
            cont_veth_name,
            host_veth_name,
            mtu,
            cont_veth_mac,
            host_ns_fd,
        )
        .await?;

        Veth::link_set_up(handle_host, host_veth_name.clone()).await?;

        Ok((host_veth_name, cont_veth))
    }

    pub async fn make_veth(
        handle: &Handle,
        name: String,
        veth_peer_name: String,
        mtu: i64,
        mac: Option<MacAddr>,
        host_ns_fd: i32,
    ) -> Result<(String, Self), CniError> {
        let peer_name: String = if veth_peer_name.is_empty() {
            utils::random_veth_name()
        } else {
            veth_peer_name
        };
        let veth: Veth = Self::make_veth_pair(
            handle,
            name.clone(),
            peer_name.clone(),
            mtu,
            mac,
            host_ns_fd,
        )
        .await?;

        Ok((peer_name, veth))
    }

    async fn make_veth_pair(
        handle: &Handle,
        name: String,
        peer: String,
        mtu: i64,
        mac: Option<MacAddr>,
        host_ns_fd: i32,
    ) -> Result<Self, CniError> {
        let mut veth: Self = Veth {
            linkattrs: LinkAttrs {
                name,
                mtu,
                txqlen: -1,
                hardware_addr: Option::None,
            },
            peer_name: peer,
            peer_namespace: host_ns_fd,
        };

        // MAC addr is set but not set...
        if let Some(addr) = mac {
            veth.linkattrs.hardware_addr = Some(addr);
        }

        veth.link_add(handle).await?;

        // ? Re-fetch the container link to get its creation-time parameters, e.g. index and mac ?

        Ok(veth)
    }
}
