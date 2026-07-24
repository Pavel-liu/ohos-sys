//! Bindings to the OpenHarmony PurgeableMemory API.
//!
//! PurgeableMemory provides APIs for managing purgeable (reclaimable) memory.
//! Available since API-level 10.
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "api-10")]
#[link(name = "purgeable_memory_ndk.z")]
unsafe extern "C" {}

#[cfg(feature = "api-10")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-10")))]
mod purgeable_memory_ffi;
#[cfg(feature = "api-10")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-10")))]
pub use purgeable_memory_ffi::*;
