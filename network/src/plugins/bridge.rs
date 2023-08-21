use derivative::Derivative;
use futures::stream::TryStreamExt;
use rtnetlink::{Error, Handle};
use serde::Deserialize;
// use std::net::IpAddr;
use crate::cni::{skel::CmdArgs, types};

#[derive(Deserialize, Debug)]
struct BridgeArgs {
    mac: String, // `mac,omitempty`
}

#[derive(Deserialize, Debug)]
struct Args {
    cni: BridgeArgs, // `cni,omitempty`
}

#[derive(Deserialize, Debug)]
pub struct NetConf<'a> {
    br_name: String,             // `bridge`
    is_gw: Option<bool>,         // `isGateway`
    is_default_gw: Option<bool>, // `isDefaultGateway`
    force_address: Option<bool>, // `forceAddress`
    ip_mask: Option<bool>,       // `ipMasq`
    mtu: u32,                    // `MTU`
    hairpin_mode: Option<bool>,  // `hairpinMode`
    promisc_mode: Option<bool>,  // `promiscMode`
    vlan: u8,                    // `vlan`
    // vlan_trunk: VlanTrunk,       // `vlanTrunk,omitempty`
    preserve_default_vlan: Option<bool>, // `preserveDefaultVlan`
    mac_spoof_chk: Option<bool>,         // `macspoofchk,omitempty`

    args: Option<Args>,
    // runtime_config: RuntimeConfig
    mac: Option<String>,
    vlans: Option<&'a [u8]>,
}

#[derive(Derivative)]
#[derivative(Debug, Default)]
#[derive(Clone)]
struct LinkAttrs {
    name: String,
    mtu: u32,
    // Let kernel use default txqueuelen; leaving it unset
    // means 0, and a zero-length TX queue messes up FIFO
    // traffic shapers which use TX queue length as the
    // default packet limit
    #[derivative(Default(value = "-1"))]
    txqlen: i8,
}

#[derive(Clone)]
pub struct Bridge {
    linkattrs: LinkAttrs,
    promisc_mode: Option<bool>,
    vlan_filtering: Option<bool>,
}

impl Bridge {
    async fn ensure_bridge(
        handle: &Handle,
        br_name: String,
        mtu: u32,
        promisc_mode: Option<bool>,
        vlan_filtering: Option<bool>,
    ) -> Self {
        let br = Self {
            linkattrs: LinkAttrs {
                name: br_name.to_string(),
                mtu,
                txqlen: -1,
            },
            promisc_mode,
            vlan_filtering,
        };

        Self::link_add(&handle, br.clone()).await;
        if br.promisc_mode == Some(true) {
            Self::link_promisc_on(&handle, br.clone()).await;
        }

        // Re-fetch link to read all attributes and if it already existed,
        // ensure it's really a bridge with similar configuration
        Self::bridge_by_name(handle, br.linkattrs.name.clone()).await;

        Self::link_set_up(&handle, br.clone()).await;

        br
    }

    async fn bridge_by_name(handle: &Handle, br_name: String) -> u32 {
        let mut links = handle.link().get().match_name(br_name.clone()).execute();
        match links.try_next().await {
            Ok(Some(link)) => {
                println!("[ORKANET]: `FLAGS` {}", link.header.flags);
                link.header.index
            }
            Ok(None) => panic!("[ORKANET]: Could not lookup {}.", br_name),
            Err(_) => panic!("[ORKANET]: Could not lookup {}.", br_name),
        }
        // Maybe check if it's bridge
    }

    /// Create a new `Bridge` instance
    pub async fn setup_bridge(n: NetConf<'_>) -> Result<Self, Error> {
        let mut vlan_filtering: bool = false;
        if n.vlan != 0 {
            // || n.vlan_trunk
            vlan_filtering = true;
        }

        let (connection, handle, _) = rtnetlink::new_connection().unwrap();
        tokio::spawn(connection);

        // create bridge if necessary
        Ok(Bridge::ensure_bridge(
            &handle,
            n.br_name,
            n.mtu,
            n.promisc_mode,
            Option::Some(vlan_filtering),
        )
        .await)
    }

    /// Attach ipv4 to interface
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

    async fn link_set_up(handle: &Handle, br: Self) {
        let mut links = handle
            .link()
            .get()
            .match_name(br.linkattrs.name.clone())
            .execute();
        match links.try_next().await {
            Ok(Some(link)) => {
                handle
                    .link()
                    .set(link.header.index)
                    .up()
                    .execute()
                    .await
                    .unwrap();
            }
            Ok(None) => panic!("[ORKANET]: Could not set up {}.", br.linkattrs.name),
            Err(_) => panic!("[ORKANET]: Could not set up {}.", br.linkattrs.name),
        }
    }

    async fn link_add(handle: &Handle, br: Self) {
        let mut links = handle
            .link()
            .get()
            .match_name(br.linkattrs.name.clone())
            .execute();
        match links.try_next().await {
            Ok(Some(_)) => panic!("[ORKANET]: Could not add {}", br.linkattrs.name),
            _ => {
                handle
                    .link()
                    .add()
                    .bridge(br.linkattrs.name.clone())
                    .execute()
                    .await
                    .unwrap();
            }
        };
    }

    async fn link_promisc_on(handle: &Handle, br: Self) {
        let mut links = handle
            .link()
            .get()
            .match_name(br.linkattrs.name.clone())
            .execute();
        match links.try_next().await {
            Ok(Some(link)) => {
                handle
                    .link()
                    .set(link.header.index)
                    .promiscuous(true)
                    .execute()
                    .await
                    .unwrap();
            }
            Ok(None) => panic!(
                "[ORKANET]: Could not set promiscuous mode on {}.",
                br.linkattrs.name
            ),
            Err(_) => panic!(
                "[ORKANET]: Could not set promiscuous mode on {}.",
                br.linkattrs.name
            ),
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
            _ => panic!("[ORKANET]: Error get index master {}.", ifname_bridge),
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
            Ok(None) => panic!(
                "[ORKANET]: Error assign veth {} to master {}.",
                ifname_veth, ifname_bridge
            ),
            Err(_) => panic!(
                "[ORKANET]: Error assign veth {} to master {}.",
                ifname_veth, ifname_bridge
            ),
        }
    }

    fn load_net_conf<'a>(bytes: &'a [u8], env_args: &'a str) -> Result<NetConf<'a>, types::Error> {
        let mut net_conf: NetConf = NetConf {
            br_name: "cni0".to_string(),
            is_gw: None,
            is_default_gw: None,
            force_address: None,
            ip_mask: None,
            mtu: 0, // Set to the appropriate default value
            hairpin_mode: None,
            promisc_mode: None,
            vlan: 1, // Set to the appropriate default value
            preserve_default_vlan: None,
            mac_spoof_chk: None,
            args: None,
            mac: None,
            vlans: None,
        };

        // Deserialize JSON data into the NetConf struct
        let n: NetConf<'_> = serde_json::from_slice(bytes).unwrap();

        // Check VLAN ID
        // if n.vlan < 0 || n.vlan > 4094 {
        //     return Err(types::Error::new(
        //         types::Code::ErrUnsupportedField,
        //         format!("invalid VLAN ID {} (must be between 0 and 4094)", n.vlan),
        //         "".to_string(),
        //     ));
        // }

        // Collect VLAN trunk data
        // n.vlans = collect_vlan_trunk(n.vlan_trunk)?;

        // Currently bridge CNI only supports access port (untagged only) or trunk port (tagged only)
        // if n.vlan > 0 && n.vlans.is_some() {
        //     return Err("cannot set vlan and vlanTrunk at the same time".into());
        // }

        // Deserialize MacEnvArgs from envArgs
        // if !env_args.is_empty() {
        //     let e: MacEnvArgs = serde_json::from_str(env_args)?;
        //     if !e.MAC.is_empty() {
        //         n.mac = e.MAC.clone();
        //     }
        // }

        // // Use values from n.Args.Cni.Mac and n.RuntimeConfig.Mac
        // if !n.args.cni.mac.is_empty() {
        //     n.mac = n.args.cni.mac.clone();
        // }

        // if !n.runtime_config.mac.is_empty() {
        //     n.mac = n.runtime_config.mac.clone();
        // }

        Ok((n))
    }

    async fn cmd_add(args: CmdArgs) {
        let mut success: bool = false;

        // let n = Self::load_net_conf()
    }

    // pub async fn get_namespace_id(container_id: &str) -> Result<i32, nix::Error> {
    //     let channel: Channel = connect("/run/containerd/containerd.sock").await.expect("Connect Failed");

    //     // let mut client: VersionClient<Channel> = VersionClient::new(channel.clone());
    //     let client = TasksClient::new(channel.clone());
    //     let req = !with
    //     client.create(request)
    //     // let resp: Response<VersionResponse> = client.version(()).await?;
    //     // println!("Response: {:?}", resp.get_ref());
    //     // Read the path to the namespace from the container ID
    //     let namespace_path: String = format!("/var/run/netns/{}", container_id);

    //     // Open the namespace file descriptor
    //     let fd: File = File::open(namespace_path).unwrap();
    //     let fd = fd.into_raw_fd();

    //     // Attach to the namespace
    //     setns(fd, CloneFlags::empty())?;

    //     // Get the current process' namespace ID
    //     let namespace_id: i32 = nix::unistd::getpid().into();

    //     // Return the namespace ID
    //     Ok(namespace_id)
    // }
}
