// src/notification_bus.rs
use crate::notification::NotificationsProxy;
use std::collections::HashMap;
use zbus::zvariant::Value;
use zbus::{Connection, Result};

pub struct NotificationInterface {
    pub proxy: NotificationsProxy<'static>,
}

impl NotificationInterface {
    pub async fn new() -> Result<Self> {
        let connection = Connection::session().await?;
        let proxy = NotificationsProxy::new(&connection).await?;
        Ok(Self { proxy })
    }

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
