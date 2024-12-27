use log::{debug, trace};
use swayipc_async::{BarMode, Connection};

pub struct SwayIpcInterface {
    connection: Connection,
}

impl SwayIpcInterface {
    pub async fn new() -> Result<Self, swayipc_async::Error> {
        let connection = Connection::new().await?;
        Ok(Self { connection })
    }

    async fn run_command(&mut self, command: &str) -> Result<(), swayipc_async::Error> {
        self.connection.run_command(command).await?;
        Ok(())
    }

    pub async fn set_bars_invisible(&mut self) -> Result<(), swayipc_async::Error> {
        let ids = self.connection.get_bar_ids().await?;
        debug!("Setting bars invisible: {:?}", ids);
        for id in ids {
            self.set_bar_mode(&id, BarMode::Invisible).await?;
        }
        Ok(())
    }

    async fn set_bar_mode(
        &mut self,
        bar_id: &str,
        bar_mode: BarMode,
    ) -> Result<(), swayipc_async::Error> {
        self.run_command(&format!("bar {} mode {:?}", bar_id, bar_mode))
            .await
    }

    pub async fn get_bar_mode(&mut self) -> Option<Vec<(String, BarMode)>> {
        let ids = self.connection.get_bar_ids().await.ok()?;
        let mut bar_modes = Vec::new();
        for id in ids {
            let bar_config = self.connection.get_bar_config(id.clone()).await.ok()?;
            bar_modes.push((id, bar_config.mode));
        }
        trace!("List of bar modes: {:?}", bar_modes);
        Some(bar_modes)
    }

    pub async fn restore_bar_mode(
        &mut self,
        bar_modes: Option<Vec<(String, BarMode)>>,
    ) -> Result<(), swayipc_async::Error> {
        let bar_modes = match bar_modes {
            Some(modes) => modes,
            None => {
                debug!("No previous bar modes provided, defaulting all bars to Dock mode");
                let ids = self.connection.get_bar_ids().await?;
                ids.into_iter().map(|id| (id, BarMode::Dock)).collect()
            }
        };

        let current_modes = self.get_bar_mode().await.unwrap_or_default();
        for (bar_id, bar_mode) in bar_modes {
            if let Some((_, current_mode)) = current_modes.iter().find(|(id, _)| id == &bar_id) {
                if format!("{:?}", current_mode) != format!("{:?}", BarMode::Invisible) {
                    debug!("Bar mode for {} not 'Invisible' anymore, has changed externally. Not restoring.", bar_id);
                    continue;
                }
            } else {
                debug!(
                    "Could not determine current bar mode for {}, assuming Dock",
                    bar_id
                );
                self.set_bar_mode(&bar_id, BarMode::Dock).await?;
                continue;
            }
            debug!("Restoring bar mode for {} to {:?}", bar_id, bar_mode);
            self.set_bar_mode(&bar_id, bar_mode).await?;
        }
        Ok(())
    }
}
