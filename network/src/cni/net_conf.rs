// use serde::{Deserialize, Serialize};

// // see: [cni/pkg/types/types.go](https://github.com/containernetworking/cni/blob/main/pkg/types/types.go)

// #[derive(Serialize, Deserialize, Debug)]
// struct IPAM {
//     r#type: String,
//     subnet: String,
// }

// #[derive(Serialize, Deserialize, Debug)]
// struct NetConf {
//     #[serde(alias = "cniVersion")]
//     cni_version: String,
//     name: String,
//     r#type: String,
//     bridge: String,
//     #[serde(alias = "isGateway")]
//     is_gateway: bool,
//     #[serde(alias = "ipMasq")]
//     ip_masq: bool,
//     ipam: IPAM,
//     // dns: DNS,
//     // raw_prevResult map[string]interface{}
//     // prev_result: Result
// }

// impl crate::cni::result::Result for NetConf {
//     fn version(&self) -> String {
//         self.cni_version.clone()
//     }

//     fn print(&self) -> crate::cni::error::Error {
//         crate::cni::error::Error::new(cni_version, code, msg, details)
//     }
// }
