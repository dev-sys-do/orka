use std::net::Ipv4Addr;

use cni_plugin::error::CniError;
use rtnetlink::Handle;

pub async fn route_add_default(handle: &Handle, addr: Ipv4Addr) -> Result<(), CniError> {
    handle
        .route()
        .add()
        .v4()
        .gateway(addr)
        .execute()
        .await
        .map_err(|err| 
             CniError::Generic(format!(
                "[ORKANET ERROR]: Failed to route add default via {} (fn route_add_default)\n{:?}\n",
                addr, err
            ))
        )
}
