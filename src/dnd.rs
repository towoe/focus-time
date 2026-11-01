//! Do-not-disturb control for the running notification daemon.

mod swaync;

use anyhow::Result;
use zbus::Connection;

use swaync::SwayNc;

/// A notification daemon that can hold back notifications during a session.
pub enum Dnd {
    SwayNc(SwayNc),
}

impl Dnd {
    /// Connects to the notification daemon on the session bus.
    pub async fn connect(conn: &Connection) -> Result<Self> {
        Ok(Self::SwayNc(SwayNc::new(conn).await?))
    }

    /// Turns do-not-disturb on.
    pub async fn enable(&mut self) -> Result<()> {
        match self {
            Self::SwayNc(swaync) => swaync.set_dnd(true).await,
        }
    }

    /// Turns do-not-disturb off again.
    pub async fn restore(&mut self) -> Result<()> {
        match self {
            Self::SwayNc(swaync) => swaync.set_dnd(false).await,
        }
    }
}
