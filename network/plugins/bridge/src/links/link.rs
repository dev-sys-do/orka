use async_trait::async_trait;
use cni_plugin::error::CniError;
use futures::stream::TryStreamExt;
use rtnetlink::Handle;

#[derive(Clone)]
pub struct LinkAttrs {
    pub name: String,
    pub mtu: i64,
    // Let kernel use default txqueuelen; leaving it unset
    // means 0, and a zero-length TX queue messes up FIFO
    // traffic shapers which use TX queue length as the
    // default packet limit
    // #[derivative(Default(value = "-1"))]
    pub txqlen: i8,
    pub hardware_addr: Option<String>,
}

#[async_trait]
pub trait Link {
    async fn link_add(&self, handle: &Handle) -> Result<(), CniError>;

    async fn link_promisc_on(handle: &Handle, name: String) -> Result<(), CniError> {
        let mut links = handle.link().get().match_name(name.clone()).execute();
        match links.try_next().await {
            Ok(Some(link)) => {
                handle
                    .link()
                    .set(link.header.index)
                    .promiscuous(true)
                    .execute()
                    .await
                    .map_err(|err| {
                        CniError::Generic(format!(
                            "[ORKANET ERROR]: Could not set promiscuous mode on {}. (fn link_promisc_on)\n{}\n",
                            name, err
                        ))
                    })
            }
            _ => Err(CniError::Generic(format!(
                "[ORKANET ERROR]: Could not set promiscuous mode on {}. (fn link_promisc_on)\n",
                name
            ))),
        }
    }

    async fn link_set_up(handle: &Handle, name: String) -> Result<(), CniError> {
        let mut links = handle.link().get().match_name(name.clone()).execute();
        match links.try_next().await {
            Ok(Some(link)) => handle
                .link()
                .set(link.header.index)
                .up()
                .execute()
                .await
                .map_err(|err| {
                    CniError::Generic(format!(
                        "[ORKANET ERROR]: Could not set up {}. (fn link_set_up)\n{}\n",
                        name, err
                    ))
                }),
            _ => Err(CniError::Generic(format!(
                "[ORKANET ERROR]: Could not set up {}. (fn link_set_up)\n",
                name
            ))),
        }
    }
}