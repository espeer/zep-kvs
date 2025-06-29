pub mod api;
pub mod convert;
pub mod error;

mod ephemeral;

#[cfg(target_os = "linux")]
mod directory;

#[cfg(target_os = "linux")]
mod linux;

mod tests;

pub mod prelude {
    pub use crate::api::{KeyValueStore, Scope, scope};
    pub use crate::convert::{InBytes, OutBytes};
}
