//! Do-not-disturb control for the running notification daemon.

mod dunst;
mod swaync;

use anyhow::{bail, Context, Result};
use log::debug;
use zbus::{fdo::DBusProxy, Connection};

use crate::notification::NotificationsProxy;
use dunst::Dunst;
use swaync::SwayNc;

/// A notification daemon that can hold back notifications during a session.
pub enum Dnd {
    SwayNc(SwayNc),
    Dunst(Dunst),
}

impl Dnd {
    /// Connects to whichever supported notification daemon runs on the session bus.
    pub async fn connect(conn: &Connection) -> Result<Self> {
        // SwayNC owns its own name next to `org.freedesktop.Notifications`.
        let dbus = DBusProxy::new(conn).await?;
        if dbus.name_has_owner(SwayNc::BUS_NAME.try_into()?).await? {
            debug!("Using SwayNC for do-not-disturb");
            return Ok(Self::SwayNc(SwayNc::new(conn).await?));
        }

        let (name, ..) = NotificationsProxy::new(conn)
            .await?
            .get_server_information()
            .await
            .context("No notification daemon running")?;
        match name.as_str() {
            "dunst" => {
                debug!("Using dunst for do-not-disturb");
                Ok(Self::Dunst(Dunst::new(conn).await?))
            }
            _ => bail!("Unsupported notification daemon `{name}`, expected SwayNC or dunst"),
        }
    }

    /// Turns do-not-disturb on, remembering the state it replaces.
    pub async fn enable(&mut self) -> Result<()> {
        match self {
            Self::SwayNc(swaync) => swaync.enable().await,
            Self::Dunst(dunst) => dunst.enable().await,
        }
    }

    /// Puts do-not-disturb back to its state before [`Dnd::enable`].
    pub async fn restore(&mut self) -> Result<()> {
        match self {
            Self::SwayNc(swaync) => swaync.restore().await,
            Self::Dunst(dunst) => dunst.restore().await,
        }
    }
}
