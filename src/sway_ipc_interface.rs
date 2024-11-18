// src/ipc.rs
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
}
