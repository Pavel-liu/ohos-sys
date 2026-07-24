//! Cloud disk manager APIs.
//!
//! Provides APIs for managing cloud disk synchronization.
//! Available since API-level 21.

pub mod cloud_disk_error_code;
mod cloud_disk_manager;

pub use cloud_disk_error_code::*;
pub use cloud_disk_manager::*;
