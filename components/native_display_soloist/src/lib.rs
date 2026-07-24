//! Bindings to the OpenHarmony NativeDisplaySoloist API.
//!
//! NativeDisplaySoloist provides APIs for display soloist scheduling.
//! Available since API-level 12.
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "api-12")]
#[link(name = "native_display_soloist")]
unsafe extern "C" {}

#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
mod native_display_soloist_ffi;
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
pub use native_display_soloist_ffi::*;
