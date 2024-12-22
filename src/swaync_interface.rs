// src/swaync_interface.rs
use crate::swaync::SwayNCProxy;
use zbus::{Connection, Result};

/// Represents the interface to interact with the SwayNC service.
pub struct SwayNCInterface {
    /// Proxy to communicate with the SwayNC service.
    pub proxy: SwayNCProxy<'static>,
}

impl SwayNCInterface {
    /// Creates a new instance of `SwayNCInterface`.
    ///
    /// This function establishes a session connection and initializes the SwayNC proxy.
    ///
    /// # Returns
    ///
    /// A `Result` containing the new `SwayNCInterface` instance or an error.
    pub async fn new() -> Result<Self> {
        let connection = Connection::session().await?;
        let proxy = SwayNCProxy::new(&connection).await?;
        Ok(Self { proxy })
    }

    /// Sets the Do Not Disturb (DND) state.
    ///
    /// # Arguments
    ///
    /// * `value` - A boolean reference indicating the desired DND state (`true` for enabled, `false` for disabled).
    ///
    /// # Returns
    ///
    /// A `Result` indicating the success or failure of the operation.
    async fn set_dnd(&self, value: &bool) -> Result<()> {
        self.proxy.set_dnd(value).await
    }

    /// Enables Do Not Disturb (DND) mode.
    ///
    /// # Returns
    ///
    /// A `Result` indicating the success or failure of the operation.
    pub async fn enable_dnd(&self) -> Result<()> {
        self.set_dnd(&true).await
    }

    /// Disables Do Not Disturb (DND) mode.
    ///
    /// # Returns
    ///
    /// A `Result` indicating the success or failure of the operation.
    pub async fn disable_dnd(&self) -> Result<()> {
        self.set_dnd(&false).await
    }
}
