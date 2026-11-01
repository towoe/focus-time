//! Do-not-disturb control for the running notification daemon.

mod dunst;
mod swaync;

use anyhow::{anyhow, bail, Context, Result};
use log::debug;
use std::time::Duration;
use zbus::{fdo::DBusProxy, names::BusName, Connection};

use crate::notification::NotificationsProxy;
use dunst::Dunst;
use swaync::SwayNc;

/// Name any freedesktop notification daemon owns on the session bus.
const NOTIFICATIONS_BUS_NAME: &str = "org.freedesktop.Notifications";

/// How long to wait for an activated notification daemon to claim its name.
const ACTIVATION_TIMEOUT: Duration = Duration::from_secs(10);

/// A notification daemon that can hold back notifications during a session.
pub enum Dnd {
    SwayNc(SwayNc),
    Dunst(Dunst),
}

impl Dnd {
    /// Connects to whichever supported notification daemon runs on the session bus.
    pub async fn connect(conn: &Connection) -> Result<Self> {
        let dbus = DBusProxy::new(conn).await?;
        let notifications = BusName::try_from(NOTIFICATIONS_BUS_NAME)?;

        // Let the bus start an installed daemon, but don't wait forever: with several service
        // files for the name, the bus may pick one that never starts (e.g. Plasma's under Sway).
        if !dbus.name_has_owner(notifications.clone()).await? {
            if !dbus
                .list_activatable_names()
                .await?
                .iter()
                .any(|n| *n == notifications)
            {
                bail!("No notification daemon running or installed, expected SwayNC or dunst");
            }
            debug!("Starting the notification daemon through D-Bus activation");
            tokio::time::timeout(
                ACTIVATION_TIMEOUT,
                dbus.start_service_by_name(notifications.try_into()?, 0),
            )
            .await
            .map_err(|_| {
                anyhow!(
                    "Timed out starting the notification daemon. If several packages install \
                     a D-Bus service for `{NOTIFICATIONS_BUS_NAME}`, the bus may have picked \
                     one that doesn't work here; start SwayNC or dunst yourself"
                )
            })?
            .context("Failed to start the notification daemon")?;
        }

        // SwayNC owns its own name next to `org.freedesktop.Notifications`.
        if dbus.name_has_owner(SwayNc::BUS_NAME.try_into()?).await? {
            debug!("Using SwayNC for do-not-disturb");
            return Ok(Self::SwayNc(SwayNc::new(conn).await?));
        }

        let (name, ..) = NotificationsProxy::new(conn)
            .await?
            .get_server_information()
            .await
            .context("Failed to query the notification daemon")?;
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
