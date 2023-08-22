use async_trait::async_trait;
use rtnetlink::{Error, Handle};

#[async_trait]
pub trait Link {
    async fn link_promisc_on(&self, handle: &Handle) -> Result<(), Error>;
    async fn link_add(&self, handle: &Handle) -> Result<(), Error>;
    async fn link_set_up(&self, handle: &Handle) -> Result<(), Error>;
}

#[derive(Clone)]
pub struct LinkAttrs {
    pub name: String,
    pub mtu: i32,
    // Let kernel use default txqueuelen; leaving it unset
    // means 0, and a zero-length TX queue messes up FIFO
    // traffic shapers which use TX queue length as the
    // default packet limit
    // #[derivative(Default(value = "-1"))]
    pub txqlen: i8,
    pub hardware_addr: Option<String>,
}
