use cni_plugin::error::CniError;
use rtnetlink::Handle;
use std::{fs, net::Ipv4Addr};

pub async fn route_add_default(handle: &Handle, addr: Ipv4Addr) -> Result<(), CniError> {
    handle
        .route()
        .add()
        .v4()
        .gateway(addr)
        .execute()
        .await
        .map_err(|err| CniError::Generic(format!(
                "[ORKANET ERROR]: Failed to route add default via {} (fn route_add_default)\n{:?}\n",
                addr, err
            ))
        )
}

pub fn enable_ip_forward(is_ipv4: bool) -> Result<(), CniError> {
    if is_ipv4 {
        Ok(enable_ip4_forward()?)
    } else {
        Err(CniError::Generic(
            "Cannot enable ip6 forward for the moment.".to_string(),
        ))
    }
}

fn enable_ip4_forward() -> std::io::Result<()> {
    echo1("/proc/sys/net/ipv4/ip_forward")
}

fn echo1(f: &str) -> std::io::Result<()> {
    if let Ok(content) = fs::read(f) {
        if content.iter().map(|b| b.is_ascii_whitespace()).all(|x| x) && content == [b'1'] {
            return Ok(());
        }
    }

    fs::write(f, [b'1'])?;
    Ok(())
}
