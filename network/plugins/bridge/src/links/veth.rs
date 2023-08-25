use super::link::{Link, LinkAttrs};
use super::utils;
use async_trait::async_trait;
use cni_plugin::error::CniError;
use futures::stream::TryStreamExt;
use nix::fcntl::{open, OFlag};
use nix::sys::stat::Mode;
use rtnetlink::Handle;
use std::path::PathBuf;

#[derive(Clone)]
pub struct Veth {
    pub linkattrs: LinkAttrs,
    pub peer_name: String,
    peer_namespace: PathBuf,
}

#[async_trait]
impl Link for Veth {
    async fn link_add(&self, handle: &Handle) -> Result<(), CniError> {
        if let Err(err) = handle
            .link()
            .add()
            .veth(self.linkattrs.name.clone(), self.peer_name.clone())
            .execute()
            .await
        {
            return Err(CniError::Generic(format!(
                "[ORKANET]: Failed to make veth pair: {} and {}. (fn link_add)\n{}\n",
                self.linkattrs.name, self.peer_name, err
            )));
        }

        let fd = match open(
            self.peer_namespace.as_path(),
            OFlag::O_RDONLY,
            Mode::empty(),
        ) {
            Ok(raw_fd) => raw_fd,
            Err(err) => {
                return Err(CniError::Generic(format!(
                "[ORKANET]: Failed to convert peer namespace to RawFd: {:?}. (fn link_add)\n{}\n",
                self.peer_namespace, err
            )))
            }
        };

        let mut links = handle
            .link()
            .get()
            .match_name(self.peer_name.clone())
            .execute();
        match links.try_next().await {
            Ok(Some(link)) => handle
                .link()
                .set(link.header.index)
                .setns_by_fd(fd)
                .execute()
                .await
                .map_err(|err| {
                    CniError::Generic(format!(
                        "[ORKANET ERROR]: Failed to set veth peer in host ns: {}. (fn link_add)\n{}\n",
                        self.peer_name, err
                    ))
                }),
            _ => Err(CniError::Generic(format!(
                "[ORKANET]: Failed to set veth peer in host ns: {}. (fn link_add)\n",
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
        cont_veth_mac: Option<&str>,
        host_ns: PathBuf,
    ) -> Result<(String, Self), CniError> {
        Self::setup_veth_with_name(
            handle_host,
            handle_cont,
            cont_veth_name,
            String::new(),
            mtu,
            cont_veth_mac,
            host_ns,
        )
        .await
    }

    async fn setup_veth_with_name(
        handle_host: &Handle,
        handle_cont: &Handle,
        cont_veth_name: String,
        host_veth_name: String,
        mtu: i64,
        cont_veth_mac: Option<&str>,
        host_ns: PathBuf,
    ) -> Result<(String, Self), CniError> {
        let (host_veth_name, cont_veth) = match Self::make_veth(
            handle_cont,
            cont_veth_name,
            host_veth_name,
            mtu,
            cont_veth_mac,
            host_ns.clone(),
        )
        .await
        {
            Ok(res) => res,
            Err(err) => return Err(err),
        };

        if let Err(err) = Veth::link_set_up(&handle_host, host_veth_name.clone()).await {
            return Err(err);
        }

        Ok((host_veth_name, cont_veth))
    }

    pub async fn make_veth(
        handle: &Handle,
        name: String,
        veth_peer_name: String,
        mtu: i64,
        mac: Option<&str>,
        host_ns: PathBuf,
    ) -> Result<(String, Self), CniError> {
        let peer_name: String = if veth_peer_name.is_empty() {
            utils::random_veth_name()
        } else {
            veth_peer_name
        };
        let veth: Veth =
            match Self::make_veth_pair(handle, name.clone(), peer_name.clone(), mtu, mac, host_ns)
                .await
            {
                Ok(veth) => veth,
                Err(err) => return Err(err),
            };

        Ok((peer_name, veth))
    }

    async fn make_veth_pair(
        handle: &Handle,
        name: String,
        peer: String,
        mtu: i64,
        mac: Option<&str>,
        host_ns: PathBuf,
    ) -> Result<Self, CniError> {
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
        if let Some(addr) = mac {
            let _ = veth.linkattrs.hardware_addr.insert(addr.to_owned());
        }

        if let Err(err) = veth.link_add(&handle).await {
            return Err(err);
        }

        // ? Re-fetch the container link to get its creation-time parameters, e.g. index and mac ?

        Ok(veth)
    }
}
