// use cni_plugin::{Cni, logger};
use orkanet::plugins::bridge::Bridge;

#[tokio::main]
async fn main() {
    let bridge = Bridge::new("orka0").await.unwrap();
    bridge.build().await.unwrap();
    // logger::install(env!("CARGO_PKG_NAME"));

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
