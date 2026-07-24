//! Bindings to the OpenHarmony BackgroundProcessManager API.
//!
//! BackgroundProcessManager provides APIs for managing background processes.
//! Available since API-level 17.
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "api-17")]
#[link(name = "background_process_manager.z")]
unsafe extern "C" {}

#[cfg(feature = "api-17")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-17")))]
mod background_process_manager_ffi;
#[cfg(feature = "api-17")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-17")))]
pub use background_process_manager_ffi::*;
