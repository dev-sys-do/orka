use super::link::{Link, LinkAttrs};
use super::utils;
use async_trait::async_trait;
use futures::stream::TryStreamExt;
use rtnetlink::{Error, Handle, NetworkNamespace};
use std::path::PathBuf;

#[derive(Clone)]
pub struct Veth {
    linkattrs: LinkAttrs,
    peer_name: String,
    peer_namespace: PathBuf,
}

#[async_trait]
impl Link for Veth {
    async fn link_add(&self, handle: &Handle) -> Result<(), Error> {
        let mut links = handle
            .link()
            .get()
            .match_name(self.linkattrs.name.clone())
            .execute();
        match links.try_next().await {
            Ok(Some(_)) => Ok(()),
            Ok(None) => {
                handle
                    .link()
                    .add()
                    .bridge(self.linkattrs.name.clone())
                    .execute()
                    .await
            }
            Err(_) => Err(Error::InvalidNla(format!(
                "[ORKANET]: Could not add link {}.",
                self.linkattrs.name
            ))),
        }
    }

    async fn link_promisc_on(&self, handle: &Handle) -> Result<(), Error> {
        let mut links = handle
            .link()
            .get()
            .match_name(self.linkattrs.name.clone())
            .execute();
        match links.try_next().await {
            Ok(Some(link)) => {
                handle
                    .link()
                    .set(link.header.index)
                    .promiscuous(true)
                    .execute()
                    .await
            }
            Ok(None) => Err(Error::InvalidNla(format!(
                "[ORKANET]: Could not set promiscuous mode on {}.",
                self.linkattrs.name
            ))),
            Err(_) => Err(Error::InvalidNla(format!(
                "[ORKANET]: Could not set promiscuous mode on {}.",
                self.linkattrs.name
            ))),
        }
    }

    async fn link_set_up(&self, handle: &Handle) -> Result<(), Error> {
        let mut links = handle
            .link()
            .get()
            .match_name(self.linkattrs.name.clone())
            .execute();
        match links.try_next().await {
            Ok(Some(link)) => handle.link().set(link.header.index).up().execute().await,
            Ok(None) => Err(Error::InvalidNla(format!(
                "[ORKANET]: Could not set up {}.",
                self.linkattrs.name
            ))),
            Err(_) => Err(Error::InvalidNla(format!(
                "[ORKANET]: Could not set up {}.",
                self.linkattrs.name
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
        handle: &Handle,
        cont_veth_name: String,
        mtu: i32,
        cont_veth_mac: String,
        host_ns: PathBuf,
    ) -> Result<((), ()), Error> {
        Self::setup_veth_with_name(
            handle,
            cont_veth_name,
            String::new(),
            mtu,
            cont_veth_mac,
            host_ns,
        )
        .await
    }

    async fn setup_veth_with_name(
        handle: &Handle,
        cont_veth_name: String,
        host_veth_name: String,
        mtu: i32,
        cont_veth_mac: String,
        host_ns: PathBuf,
    ) -> Result<((), ()), Error> {
        let (host_veth_name, cont_veth) = Self::make_veth(
            handle,
            cont_veth_name,
            String::new(),
            mtu,
            cont_veth_mac,
            host_ns.clone(),
        )
        .await
        .unwrap();

        // Lookup host veth
        let host_netns_path = match host_ns.as_os_str().to_os_string().into_string() {
            Ok(path) => path,
            Err(_) => return Err(Error::RequestFailed),
        };
        if let Err(err) = NetworkNamespace::unshare_processing(host_netns_path) {
            return Err(err);
        }
        // let host_veth = ;

        Ok(((), ()))
    }

    pub async fn make_veth(
        handle: &Handle,
        name: String,
        veth_peer_name: String,
        mtu: i32,
        mac: String,
        host_ns: PathBuf,
    ) -> Result<(String, Self), Error> {
        let peer_name: String = veth_peer_name
            .is_empty()
            .then(|| utils::random_veth_name())
            .unwrap_or(veth_peer_name);
        let veth: Veth =
            match Self::make_veth_pair(handle, name, peer_name.clone(), mtu, mac, host_ns).await {
                Ok(veth) => veth,
                Err(err) => {
                    return Err(Error::InvalidNla(format!(
                        "Failed to make veth pair {}",
                        err
                    )))
                }
            };

        Ok((peer_name, veth))
    }

    async fn make_veth_pair(
        handle: &Handle,
        name: String,
        peer: String,
        mtu: i32,
        mac: String,
        host_ns: PathBuf,
    ) -> Result<Self, Error> {
        let mut veth: Self = Veth {
            linkattrs: LinkAttrs {
                name,
                mtu,
                txqlen: -1,
                hardware_addr: Option::None,
            },
            peer_name: peer,
            peer_namespace: host_ns,
        };
        if !mac.is_empty() {
            veth.linkattrs.hardware_addr = Some(mac);
        }

        if let Err(err) = handle
            .link()
            .add()
            .veth(veth.linkattrs.name.clone(), veth.peer_name.clone())
            .execute()
            .await
        {
            return Err(Error::InvalidNla(format!(
                "[ORKANET]: Failed to make veth pair {}",
                err
            )));
        }

        // Re-fetch the container link to get its creation-time parameters, e.g. index and mac
        // veth2, err := netlink.LinkByName(name)
        // let mut links = handle
        //     .link()
        //     .get()
        //     .match_name(veth.linkattrs.name.clone())
        //     .execute();
        // let veth2 = match links.try_next().await {
        //     Ok(Some(link)) => link,
        //     Ok(None) => {
        //         return Err(Error::InvalidNla(format!(
        //             "[ORKANET]: Could not set promiscuous mode on {}.",
        //             veth.linkattrs.name
        //         )))
        //     }
        //     Err(_) => {
        //         return Err(Error::InvalidNla(format!(
        //             "[ORKANET]: Could not set promiscuous mode on {}.",
        //             veth.linkattrs.name
        //         )))
        //     }
        // };

        Ok(veth)
    }
}
