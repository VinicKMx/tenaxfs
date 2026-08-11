#![no_std]
#![doc = "Core types for TenaxFS."]

#[cfg(test)]
extern crate std;

pub mod config;
pub mod error;
pub mod flash;
pub mod format;
pub mod maintenance;
pub mod object;
pub mod transaction;

pub use config::{Config, Geometry};
pub use error::{StorageError, StorageResult};
pub use flash::Flash;
