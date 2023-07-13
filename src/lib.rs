#[cfg(feature = "browser")]
pub use browser::Browser;


#[cfg(feature = "discord")]
pub mod discord;

#[cfg(feature = "browser")]
pub mod browser;

#[cfg(feature = "system")]
pub mod system;

#[cfg(feature = "ip")]
pub mod ip;

#[cfg(feature = "browser-util")]
pub(crate) mod browser_util;