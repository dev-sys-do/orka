use rtnetlink::{AddressAddRequest, Error, LinkAddRequest};
use std::net::{IpAddr, Ipv4Addr};

pub struct Bridge {
    ifname: String,
}

impl Bridge {
    /// Creates a new `Bridge` instance with the given name.
    pub fn new(name: &str) -> Self {
        Bridge {
            ifname: name.to_string(),
        }
    }

    /// Builds and configures the bridge network.
    pub async fn build(&self) -> Result<(), Error> {
        // Establish a new connection to rtnetlink
        let (connection, handle, _) = rtnetlink::new_connection().unwrap();
        tokio::spawn(connection);

        // Create a bridge interface
        let bridge_request: LinkAddRequest = handle.link().add().bridge(self.ifname.clone());
        bridge_request
            .execute()
            .await
            .map_err(|e| println!("[ORKANET]: ERROR {}", e))
            .unwrap();

        // Retrieve the index of the bridge interface
        let index: u32 = unsafe { libc::if_nametoindex(self.ifname.clone().as_ptr() as *const i8) };

        // Assign an IP address (e.g., 10.10.0.1/16) to the bridge interface
        let address: IpAddr = IpAddr::V4(Ipv4Addr::new(10, 10, 0, 1));
        let address_bridge_request: AddressAddRequest = handle.address().add(index, address, 16);
        address_bridge_request
            .execute()
            .await
            .map_err(|e| println!("[ORKANET]: ERROR {}", e))
            .unwrap();

        // Enable the bridge interface
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
