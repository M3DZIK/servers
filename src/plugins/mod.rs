//! Plugin infrastructure.

mod load;
mod manager;
pub mod types;

pub use load::*;
pub use manager::*;

/// Crates and types required in plugins.
pub mod prelude {
    use super::*;

    pub extern crate anyhow;
    pub extern crate async_std;
    pub use async_trait::async_trait;

    pub use self::types::*;
    pub use crate::server::{Client, ClientMapValue};
}
