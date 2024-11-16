// src/swaync_interface.rs
use zbus::{Connection, Result};
use crate::swaync::SwayNCProxy;

pub struct SwayNCInterface {
    pub proxy: SwayNCProxy<'static>,
}

impl SwayNCInterface {
    pub async fn new() -> Result<Self> {
        let connection = Connection::session().await?;
        let proxy = SwayNCProxy::new(&connection).await?;
        Ok(Self { proxy })
    }

    async fn set_dnd(&self, value: &bool) -> Result<()> {
        self.proxy.set_dnd(value).await
    }

    pub async fn enable_dnd(&self) -> Result<()> {
        self.set_dnd(&true).await
    }

    pub async fn disable_dnd(&self) -> Result<()> {
        self.set_dnd(&false).await
    }

}