use rtnetlink::{AddressAddRequest, Error, LinkAddRequest};
use std::net::{IpAddr, Ipv4Addr};

pub struct Bridge {
    _ifname: String,
    index: u32,
}

impl Bridge {
    /// Creates a new `Bridge` instance with the given name.
    pub async fn new(ifname: &str) -> Result<Self, Error> {
        // Establish a new connection to rtnetlink
        let (connection, handle, _) = rtnetlink::new_connection().unwrap();
        tokio::spawn(connection);

        // Create a bridge interface
        let bridge_request: LinkAddRequest = handle.link().add().bridge(ifname.to_string());
        bridge_request
            .execute()
            .await
            .map_err(|e| println!("[ORKANET]: ERROR {}", e))
            .unwrap();

        // Retrieve the index of the bridge interface
        let index: u32 = unsafe { libc::if_nametoindex(ifname.to_string().as_ptr() as *const i8) };

        Ok(Bridge {
            _ifname: ifname.to_string(),
            index,
        })
    }

    /// Add an address
    async fn set_addr(&self, ipaddr: IpAddr) -> Result<(), Error> {
        // Establish a new connection to rtnetlink
        let (connection, handle, _) = rtnetlink::new_connection().unwrap();
        tokio::spawn(connection);

        // Assign an IP address (e.g., 10.10.0.1/16) to the bridge interface
        let address_bridge_request: AddressAddRequest =
            handle.address().add(self.index.clone(), ipaddr, 16);
        address_bridge_request
            .execute()
            .await
            .map_err(|e| println!("[ORKANET]: ERROR {}", e))
            .unwrap();

        Ok(())
    }

    /// Up interface
    async fn up(&self) -> Result<(), Error> {
        // Establish a new connection to rtnetlink
        let (connection, handle, _) = rtnetlink::new_connection().unwrap();
        tokio::spawn(connection);

        // Enable the bridge interface
        handle
            .link()
            .set(self.index.clone())
            .up()
            .execute()
            .await
            .map_err(|e| println!("[ORKANET]: ERROR {}", e))
            .unwrap();

        Ok(())
    }

    /// Build
    pub async fn build(&self) -> Result<(), Error> {
        let ipaddr: IpAddr = IpAddr::V4(Ipv4Addr::new(10, 10, 0, 1));

        self.set_addr(ipaddr).await.unwrap();
        self.up().await.unwrap();

        Ok(())
    }

    /// Add an interface
    pub async fn add_interface(&self, link_index: u32) -> Result<(), Error> {
        // Establish a new connection to rtnetlink
        let (connection, handle, _) = rtnetlink::new_connection().unwrap();
        tokio::spawn(connection);

        // Add
        handle
            .link()
            .set(link_index)
            .master(self.index.clone())
            .execute()
            .await
            .map_err(|e| println!("[ORKANET]: ERROR {}", e))
            .unwrap();

        Ok(())
    }
}
