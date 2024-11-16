use zbus::{proxy, Result};

#[proxy(
    interface = "org.erikreider.swaync.cc",
    default_service = "org.erikreider.swaync.cc",
    default_path = "/org/erikreider/swaync/cc"
)]
pub trait SwayNC {
    async fn set_dnd(&self, state: &bool) -> Result<()>;
}
