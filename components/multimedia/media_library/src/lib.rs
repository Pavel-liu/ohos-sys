//! MediaLibraryKit media library bindings for OpenHarmony.
//!
//! This crate exposes raw C ABI bindings generated from
//! `multimedia/media_library/*_capi.h`. It does not provide safe wrappers around
//! callback lifetimes or native object ownership.
//!
//! Official MediaLibraryKit C API reference: <https://docs.openharmony.cn/pages/v5.0/en/application-dev/reference/apis-media-library-kit/capi-medialibrary.md>.
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "api-12")]
#[link(name = "media_asset_manager")]
unsafe extern "C" {}

#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
pub mod access_helper;
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
pub mod asset;
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
pub mod asset_base;
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
pub mod asset_change_request;
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
pub mod asset_manager;
#[cfg(feature = "api-13")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-13")))]
pub mod moving_photo;
