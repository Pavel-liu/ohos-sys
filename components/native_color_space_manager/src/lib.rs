//! Bindings to the OpenHarmony NativeColorSpaceManager API.
//!
//! NativeColorSpaceManager provides APIs for managing color spaces.
//! Available since API-level 13.
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "api-13")]
#[link(name = "native_color_space_manager")]
unsafe extern "C" {}

#[cfg(feature = "api-13")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-13")))]
mod native_color_space_manager_ffi;
#[cfg(feature = "api-13")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-13")))]
pub use native_color_space_manager_ffi::*;
