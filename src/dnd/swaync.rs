//! SwayNC, controlled over its control-center D-Bus interface.

use anyhow::{Context, Result};
use zbus::{proxy, Connection};

#[proxy(
    interface = "org.erikreider.swaync.cc",
    default_service = "org.erikreider.swaync.cc",
    default_path = "/org/erikreider/swaync/cc"
)]
trait ControlCenter {
    fn set_dnd(&self, state: bool) -> zbus::Result<()>;
}

pub struct SwayNc {
    proxy: ControlCenterProxy<'static>,
}

impl SwayNc {
    pub async fn new(conn: &Connection) -> Result<Self> {
        let proxy = ControlCenterProxy::new(conn)
            .await
            .context("Failed to connect to SwayNC. Is SwayNC running?")?;
        Ok(Self { proxy })
    }

    pub async fn set_dnd(&self, state: bool) -> Result<()> {
        Ok(self.proxy.set_dnd(state).await?)
    }
}
