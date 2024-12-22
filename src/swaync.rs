use zbus::{proxy, Result};

/// Proxy interface for interacting with the SwayNC service.
///
/// This trait defines the methods available for communication with the SwayNC service
/// over D-Bus. The service is identified by the interface `org.erikreider.swaync.cc`,
/// the default service name `org.erikreider.swaync.cc`, and the default object path
/// `/org/erikreider/swaync/cc`.
#[proxy(
    interface = "org.erikreider.swaync.cc",
    default_service = "org.erikreider.swaync.cc",
    default_path = "/org/erikreider/swaync/cc"
)]
pub trait SwayNC {
    /// Asynchronously sets the Do Not Disturb (DND) state.
    ///
    /// # Arguments
    ///
    /// * `state` - A boolean reference indicating the desired DND state (`true` for enabled, `false` for disabled).
    ///
    /// # Returns
    ///
    /// A `Result` indicating the success or failure of the operation.
    async fn set_dnd(&self, state: &bool) -> Result<()>;
}
