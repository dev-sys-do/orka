use ipnet::IpNet;

pub struct CNICommand {
    pub container_id: String,
    pub ifname: String,
    pub netns: String,

    pub data_dir: String,
    pub subnet: IpNet,
    pub cni_version: String
}