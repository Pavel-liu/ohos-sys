//! Environment directory access APIs.
//!
//! Provides functions to get user Download, Desktop, and Document directories.

mod environment;
pub mod error_code;

pub use environment::*;
pub use error_code::*;
