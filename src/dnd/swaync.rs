//! SwayNC, controlled over its control-center D-Bus interface.

use anyhow::{Context, Result};
use zbus::{proxy, Connection};

#[proxy(
    interface = "org.erikreider.swaync.cc",
    default_service = "org.erikreider.swaync.cc",
    default_path = "/org/erikreider/swaync/cc"
)]
trait ControlCenter {
    fn get_dnd(&self) -> zbus::Result<bool>;
    fn set_dnd(&self, state: bool) -> zbus::Result<()>;
}

pub struct SwayNc {
    proxy: ControlCenterProxy<'static>,
    /// DND state from before [`SwayNc::enable`].
    saved: bool,
}

impl SwayNc {
    pub async fn new(conn: &Connection) -> Result<Self> {
        let proxy = ControlCenterProxy::new(conn)
            .await
            .context("Failed to connect to SwayNC. Is SwayNC running?")?;
        Ok(Self {
            proxy,
            saved: false,
        })
    }

    pub async fn enable(&mut self) -> Result<()> {
        self.saved = self.proxy.get_dnd().await?;
        Ok(self.proxy.set_dnd(true).await?)
    }

    pub async fn restore(&mut self) -> Result<()> {
        Ok(self.proxy.set_dnd(self.saved).await?)
    }
}
