//! Bindings to the OpenHarmony TransientTask API.
//!
//! TransientTask provides APIs for requesting transient task suspension delays.
//! Available since API-level 11 (type definitions) and API-level 13 (request APIs).
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "api-11")]
#[link(name = "transient_task")]
unsafe extern "C" {}

#[cfg(feature = "api-11")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-11")))]
mod transient_task_ffi;
#[cfg(feature = "api-11")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-11")))]
pub use transient_task_ffi::*;
