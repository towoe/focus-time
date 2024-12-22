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

    pub async fn get_bar_mode(&mut self) -> Result<Vec<(String, BarMode)>, swayipc_async::Error> {
        let ids = self.connection.get_bar_ids().await?;
        let mut bar_modes = Vec::new();
        for id in ids {
            let bar_config = self.connection.get_bar_config(id.clone()).await?;
            bar_modes.push((id, bar_config.mode));
        }
        trace!("List of bar modes: {:?}", bar_modes);
        Ok(bar_modes)
    }

    pub async fn restore_bar_mode(
        &mut self,
        bar_modes: Vec<(String, BarMode)>,
    ) -> Result<(), swayipc_async::Error> {
        let current_modes = self.get_bar_mode().await?;
        for (bar_id, bar_mode) in bar_modes {
            if let Some((_, current_mode)) = current_modes.iter().find(|(id, _)| id == &bar_id) {
                if format!("{:?}", current_mode) != format!("{:?}", BarMode::Invisible) {
                    debug!("Bar mode for {} not 'Invisible' anymore, has changed externally. Not restoring.", bar_id);
                    continue;
                }
            }
            debug!("Restoring bar mode for {} to {:?}", bar_id, bar_mode);
            self.set_bar_mode(&bar_id, bar_mode).await?;
        }
        Ok(())
    }
}
