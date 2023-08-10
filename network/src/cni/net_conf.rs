use ipnet::Ipv4Net;
use serde::{Deserialize, Serialize};

/// see:
/// [cni/pkg/types/types.go](https://github.com/containernetworking/cni/blob/f6506e215fb27ffbd9b46d9d18aae29dba697a52/pkg/types/types.go#L60)

#[derive(Default, Deserialize, Serialize)]
pub struct NetConfList {
    #[serde(rename = "cniVersion")]
    cni_version: String,
    name: String,
    #[serde(rename = "disableCheck")]
    disable_check: bool,
    plugins: NetConf,
}

#[derive(Default, Deserialize, Serialize)]
pub struct NetConf {
    #[serde(rename = "cniVersion")]
    cni_version: Option<String>,
    name: Option<String>,
    #[serde(rename = "type")]
    binary_name: String,
    subnet: Ipv4Net,
    #[serde(rename = "prevResult")]
    prev_result: Option<Box<NetConf>>,
}
