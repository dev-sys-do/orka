#[tokio::main]
async fn main() {
    // let ipv4: IpAddr = IpAddr::V4(Ipv4Addr::new(10, 10, 0, 1));
    // let _bridge = Bridge::setup_bridge("orka0", ipv4, 16).await.unwrap();

    // match Cni::load() {
    //     Cni::Add {
    //         container_id,
    //         ifname,
    //         netns,
    //         path,
    //         config,
    //     } => orka_cni::add(),
    //     Cni::Del {
    //         container_id,
    //         ifname,
    //         netns,
    //         path,
    //         config,
    //     } => orka_cni::delete(),
    //     Cni::Check {
    //         container_id,
    //         ifname,
    //         netns,
    //         path,
    //         config,
    //     } => orka_cni::check(),
    //     Cni::Version(_) => unreachable!()
    // };
}
