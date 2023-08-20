use std::net::{IpAddr, Ipv4Addr};

// use libc;
use rtnetlink::{AddressAddRequest, Error, LinkAddRequest};
// use std::net::{IpAddr, Ipv4Addr};

pub struct Bridge {
    ifname: String,
}

impl Bridge {
    pub fn new(name: &str) -> Self {
        Bridge {
            ifname: name.to_string(),
        }
    }

    pub async fn build(&self) -> Result<(), Error> {
        // New connection
        let (connection, handle, _) = rtnetlink::new_connection().unwrap();
        tokio::spawn(connection);

        // $ ip link add orka0 type bridge
        let bridge_request: LinkAddRequest = handle.link().add().bridge(self.ifname.clone());
        bridge_request
            .execute()
            .await
            .map_err(|e| println!("[ORKANET]: ERROR {}", e))
            .unwrap();

        // Get index "orka0"
        let index: u32 = unsafe { libc::if_nametoindex(self.ifname.clone().as_ptr() as *const i8) };

        // $ ip address add 10.10.0.1/16 dev orka0
        let address: IpAddr = IpAddr::V4(Ipv4Addr::new(10, 10, 0, 1));
        let address_bridge_request: AddressAddRequest = handle.address().add(index, address, 16);
        address_bridge_request
            .execute()
            .await
            .map_err(|e| println!("[ORKANET]: ERROR {}", e))
            .unwrap();

        // $ ip link set dev orka0 up
        handle
            .link()
            .set(index)
            .up()
            .execute()
            .await
            .map_err(|e| println!("[ORKANET]: ERROR {}", e))
            .unwrap();

        Ok(())
    }
}
