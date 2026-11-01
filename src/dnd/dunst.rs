//! dunst, paused through its `org.dunstproject.cmd0` D-Bus interface.

use anyhow::Result;
use zbus::{proxy, proxy::CacheProperties, Connection};

#[proxy(
    interface = "org.dunstproject.cmd0",
    default_service = "org.freedesktop.Notifications",
    default_path = "/org/freedesktop/Notifications"
)]
trait DunstCmd {
    /// 0 shows everything; `paused` raises it to 100.
    #[zbus(property, name = "pauseLevel")]
    fn pause_level(&self) -> zbus::Result<u32>;
    #[zbus(property, name = "pauseLevel")]
    fn set_pause_level(&self, level: u32) -> zbus::Result<()>;
    #[zbus(property, name = "paused")]
    fn set_paused(&self, paused: bool) -> zbus::Result<()>;
}

pub struct Dunst {
    proxy: DunstCmdProxy<'static>,
    /// Pause level from before [`Dunst::enable`], so a manual pause survives the session.
    saved: u32,
}

impl Dunst {
    pub async fn new(conn: &Connection) -> Result<Self> {
        // Uncached: the level can change from `dunstctl` at any time.
        let proxy = DunstCmdProxy::builder(conn)
            .cache_properties(CacheProperties::No)
            .build()
            .await?;
        Ok(Self { proxy, saved: 0 })
    }

    pub async fn enable(&mut self) -> Result<()> {
        self.saved = self.proxy.pause_level().await?;
        Ok(self.proxy.set_paused(true).await?)
    }

    pub async fn restore(&mut self) -> Result<()> {
        Ok(self.proxy.set_pause_level(self.saved).await?)
    }
}
