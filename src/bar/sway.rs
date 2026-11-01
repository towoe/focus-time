//! Sway's bars, switched through bar modes over Sway IPC.

use anyhow::Result;
use log::{debug, trace};
use swayipc_async::{BarMode, Connection};

pub struct SwayBars {
    connection: Connection,
    /// Bar modes from before [`SwayBars::hide`]; `None` if they couldn't be read.
    saved: Option<Vec<(String, BarMode)>>,
}

impl SwayBars {
    pub async fn new() -> Result<Self> {
        let connection = Connection::new().await?;
        Ok(Self {
            connection,
            saved: None,
        })
    }

    pub async fn hide(&mut self) -> Result<()> {
        self.saved = self.bar_modes().await;
        let ids = self.connection.get_bar_ids().await?;
        debug!("Setting bars invisible: {ids:?}");
        for id in ids {
            self.set_bar_mode(&id, BarMode::Invisible).await?;
        }
        Ok(())
    }

    /// Restores the saved modes, skipping bars whose mode was changed during the session.
    pub async fn restore(&mut self) -> Result<()> {
        let bar_modes = match self.saved.take() {
            Some(modes) => modes,
            None => {
                debug!("No previous bar modes provided, defaulting all bars to Dock mode");
                let ids = self.connection.get_bar_ids().await?;
                ids.into_iter().map(|id| (id, BarMode::Dock)).collect()
            }
        };

        let current_modes = self.bar_modes().await.unwrap_or_default();
        for (bar_id, bar_mode) in bar_modes {
            match current_modes.iter().find(|(id, _)| id == &bar_id) {
                Some((_, BarMode::Invisible)) => {}
                Some(_) => {
                    debug!("Bar mode for {bar_id} changed externally. Not restoring.");
                    continue;
                }
                None => {
                    debug!("Could not determine current bar mode for {bar_id}, assuming Dock");
                    self.set_bar_mode(&bar_id, BarMode::Dock).await?;
                    continue;
                }
            }
            debug!("Restoring bar mode for {bar_id} to {bar_mode:?}");
            self.set_bar_mode(&bar_id, bar_mode).await?;
        }
        Ok(())
    }

    async fn bar_modes(&mut self) -> Option<Vec<(String, BarMode)>> {
        let ids = self.connection.get_bar_ids().await.ok()?;
        let mut bar_modes = Vec::new();
        for id in ids {
            let bar_config = self.connection.get_bar_config(id.clone()).await.ok()?;
            bar_modes.push((id, bar_config.mode));
        }
        trace!("List of bar modes: {bar_modes:?}");
        Some(bar_modes)
    }

    async fn set_bar_mode(&mut self, bar_id: &str, bar_mode: BarMode) -> Result<()> {
        self.connection
            .run_command(format!("bar {bar_id} mode {bar_mode:?}"))
            .await?;
        Ok(())
    }
}
