use log::debug;
use swayipc_async::Connection;

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

    pub async fn set_bar_mode_invisible(&mut self) -> Result<(), swayipc_async::Error> {
        self.run_command("bar mode invisible").await
    }

    pub async fn set_bar_mode_dock(&mut self) -> Result<(), swayipc_async::Error> {
        self.run_command("bar mode dock").await
    }

    pub async fn set_bar_mode_hide(&mut self) -> Result<(), swayipc_async::Error> {
        self.run_command("bar mode hide").await
    }

    pub async fn get_bar_mode(&mut self) -> Result<swayipc_async::BarMode, swayipc_async::Error> {
        let ids = self.connection.get_bar_ids().await?;
        if let Some(first_id) = ids.first() {
            let bar_config = self.connection.get_bar_config(first_id.to_string()).await?;
            Ok(bar_config.mode)
        } else {
            Ok(swayipc_async::BarMode::Dock)
        }
    }

    pub async fn restore_bar_mode(
        &mut self,
        bar_mode: swayipc_async::BarMode,
    ) -> Result<(), swayipc_async::Error> {
        // Do not restore if the mode has change since the timer was started
        // swayipc_async::BarMode does not implement PartialEq
        if format!("{:?}", self.get_bar_mode().await?)
            != format!("{:?}", swayipc_async::BarMode::Invisible)
        {
            debug!("Bar mode not 'Invisible' anymore, has changed externally. Not restoring.");
            return Ok(());
        }
        debug!("Restoring bar mode to {:?}", bar_mode);
        match bar_mode {
            swayipc_async::BarMode::Dock => self.set_bar_mode_dock().await,
            swayipc_async::BarMode::Invisible => self.set_bar_mode_invisible().await,
            swayipc_async::BarMode::Hide => self.set_bar_mode_hide().await,
            _ => Ok(()),
        }
    }
}
