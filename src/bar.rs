//! Hiding and restoring the status bar during a session.

mod sway;

use anyhow::Result;

use sway::SwayBars;

/// A status bar that can be hidden for the length of a session.
pub enum Bar {
    Sway(SwayBars),
}

impl Bar {
    /// Connects to the bar's controller.
    pub async fn connect() -> Result<Self> {
        Ok(Self::Sway(SwayBars::new().await?))
    }

    /// Hides the bar, remembering how to bring it back.
    pub async fn hide(&mut self) -> Result<()> {
        match self {
            Self::Sway(sway) => sway.hide().await,
        }
    }

    /// Brings the bar back to its state before [`Bar::hide`].
    pub async fn restore(&mut self) -> Result<()> {
        match self {
            Self::Sway(sway) => sway.restore().await,
        }
    }
}
