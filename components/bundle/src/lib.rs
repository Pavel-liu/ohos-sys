//! Bindings to the OpenHarmony Bundle (application bundle management) API.
//!
//! The Bundle API provides functions for querying application bundle information,
//! including application info, ability info, and metadata. Available since API-level 11.
//!
//! See also the [Bundle NDK guide] for usage details.
//!
//! [Bundle NDK guide]: https://docs.openharmony.cn/pages/v5.0/en/application-dev/arkapi-ndk-bundle.md
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "api-11")]
#[link(name = "bundle_ndk.z")]
unsafe extern "C" {}

#[cfg(feature = "api-11")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-11")))]
mod bundle_ffi;
#[cfg(feature = "api-11")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-11")))]
pub use bundle_ffi::*;
