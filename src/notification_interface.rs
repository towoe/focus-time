// src/notification_interface.rs
use crate::notification::NotificationsProxy;
use std::collections::HashMap;
use zbus::zvariant::Value;
use zbus::{Connection, Result};

/// A struct representing the notification interface.
pub struct NotificationInterface {
    pub proxy: NotificationsProxy<'static>,
}

impl NotificationInterface {
    /// Creates a new instance of `NotificationInterface`.
    ///
    /// This function establishes a D-Bus session connection and creates a new
    /// `NotificationsProxy` for interacting with the notification service.
    ///
    /// # Returns
    ///
    /// A `Result` containing the new `NotificationInterface` instance.
    pub async fn new() -> Result<Self> {
        let connection = Connection::session().await?;
        let proxy = NotificationsProxy::new(&connection).await?;
        Ok(Self { proxy })
    }

    /// Sends a notification.
    ///
    /// # Arguments
    ///
    /// * `summary` - The summary text of the notification.
    /// * `body` - The body text of the notification.
    /// * `hints` - A map of hints to provide additional information about the notification.
    ///
    /// # Returns
    ///
    /// A `Result` containing the ID of the notification.
    pub async fn notify(
        &self,
        summary: &str,
        body: &str,
        hints: HashMap<&str, &Value<'_>>,
    ) -> Result<u32> {
        self.proxy
            .notify(
                "focus-time",
                0,
                "selection-mode",
                summary,
                body,
                &[],
                hints,
                0,
            )
            .await
    }
}
