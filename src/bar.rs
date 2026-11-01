//! Hiding and restoring the status bar during a session.

mod sway;
mod waybar;

use anyhow::{bail, Result};
use log::debug;

use sway::SwayBars;
use waybar::Waybar;

/// A status bar that can be hidden for the length of a session.
pub enum Bar {
    Sway(SwayBars),
    Waybar(Waybar),
}

impl Bar {
    /// Connects to the bar's controller: Sway when it runs, otherwise Waybar directly.
    pub async fn connect() -> Result<Self> {
        // Under Sway, bar modes also reach a Waybar with `"ipc": true`.
        if std::env::var_os("SWAYSOCK").is_some() {
            debug!("Using Sway IPC for the bar");
            return Ok(Self::Sway(SwayBars::new().await?));
        }
        if let Some(waybar) = Waybar::find() {
            debug!("Using Waybar signals for the bar");
            return Ok(Self::Waybar(waybar));
        }
        bail!("No supported status bar found, expected Sway or Waybar (use -s to skip hiding)")
    }

    /// Hides the bar, remembering how to bring it back.
    pub async fn hide(&mut self) -> Result<()> {
        match self {
            Self::Sway(sway) => sway.hide().await,
            Self::Waybar(waybar) => waybar.hide(),
        }
    }

    /// Brings the bar back to its state before [`Bar::hide`].
    pub async fn restore(&mut self) -> Result<()> {
        match self {
            Self::Sway(sway) => sway.restore().await,
            Self::Waybar(waybar) => waybar.restore(),
        }
    }
}
