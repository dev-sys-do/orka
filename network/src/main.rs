use cni_plugin::Cni;

fn main() {
    cni_plugin::logger::install("./orka-cni.log");

    let tre = match Cni::load() {
        Cni::Add {
            container_id,
            ifname,
            netns,
            path,
            config,
        } => orka_cni::add(),
        Cni::Del {
            container_id,
            ifname,
            netns,
            path,
            config,
        } => orka_cni::delete(),
        Cni::Check {
            container_id,
            ifname,
            netns,
            path,
            config,
        } => orka_cni::check(),
        Cni::Version(_) => unreachable!(),
    };
}
