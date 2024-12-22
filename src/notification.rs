use std::collections::HashMap;

use zbus::{proxy, zvariant::Value};

/// A trait representing the D-Bus interface for desktop notifications.
///
/// This trait provides a method to send notifications using the
/// `org.freedesktop.Notifications` D-Bus service.
#[proxy(
    default_service = "org.freedesktop.Notifications",
    default_path = "/org/freedesktop/Notifications"
)]
pub trait Notifications {
    /// Sends a notification.
    ///
    /// # Arguments
    ///
    /// * `app_name` - The name of the application sending the notification.
    /// * `replaces_id` - The ID of the notification to replace, or 0 to create a new notification.
    /// * `app_icon` - The icon of the application sending the notification.
    /// * `summary` - The summary text of the notification.
    /// * `body` - The body text of the notification.
    /// * `actions` - A list of actions associated with the notification.
    /// * `hints` - A map of hints to provide additional information about the notification.
    /// * `expire_timeout` - The timeout in milliseconds at which the notification should expire.
    ///
    /// # Returns
    ///
    /// A `zbus::Result` containing the ID of the notification.
    #[allow(clippy::too_many_arguments)]
    fn notify(
        &self,
        app_name: &str,
        replaces_id: u32,
        app_icon: &str,
        summary: &str,
        body: &str,
        actions: &[&str],
        hints: HashMap<&str, &Value<'_>>,
        expire_timeout: i32,
    ) -> zbus::Result<u32>;
}
