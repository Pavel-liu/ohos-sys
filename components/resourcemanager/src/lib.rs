//! Bindings to the OpenHarmony ResourceManager API.
//!
//! ResourceManager provides APIs for querying application resources such as
//! strings, booleans, media, colors, and configuration. Available since
//! API-level 12.
//!
//! Rawfile APIs are provided by `ohos-rawfile-sys`; this crate only exposes the
//! ResourceManager headers to avoid mixing the rawfile `OH_ResourceManager_*RawFile*`
//! functions into these bindings.
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "api-12")]
#[link(name = "ohresmgr")]
unsafe extern "C" {}

#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
mod resourcemanager_ffi;
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
pub use resourcemanager_ffi::*;
