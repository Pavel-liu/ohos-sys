//! Rust bindings to OpenHarmony CoreFileKit native APIs.
//!
//! CoreFileKit provides C APIs for file management operations including
//! environment directory access, file I/O, file sharing, URI processing,
//! and cloud disk management.
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(clippy::doc_markdown)]
#![allow(ambiguous_glob_reexports)]

// Re-export the common error types from environment module
#[cfg(feature = "api-12")]
pub use crate::environment::error_code::{FileManagementErrorCode, FileManagementResult};

// Environment module (since api-12)
#[cfg(feature = "api-12")]
#[link(name = "ohenvironment")]
unsafe extern "C" {}

#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
mod environment;

#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
pub use environment::*;

// FileIO module (since api-12)
#[cfg(feature = "api-12")]
#[link(name = "ohfileio")]
unsafe extern "C" {}

#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
mod fileio;

#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
pub use fileio::*;

// FileShare module (since api-12)
#[cfg(feature = "api-12")]
#[link(name = "ohfileshare")]
unsafe extern "C" {}

#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
mod fileshare;

#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
pub use fileshare::*;

// File URI module (since api-12)
#[cfg(feature = "api-12")]
#[link(name = "ohfileuri")]
unsafe extern "C" {}

#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
mod file_uri;

#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
pub use file_uri::*;

// CloudDiskManager module (since api-21)
#[cfg(feature = "api-21")]
#[link(name = "ohclouddiskmanager")]
unsafe extern "C" {}

#[cfg(feature = "api-21")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-21")))]
mod clouddiskmanager;

#[cfg(feature = "api-21")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-21")))]
pub use clouddiskmanager::*;
