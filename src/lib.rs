pub mod api;
pub mod convert;
pub mod error;

mod ephemeral;

mod tests;

pub mod prelude {
    pub use crate::api::{KeyValueStore, Scope, scope};
    pub use crate::convert::{InBytes, OutBytes};
}
