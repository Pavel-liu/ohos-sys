//! Bindings to the `OpenHarmony` `i18n` timezone API.
//!
//! The i18n timezone API provides timezone offset transition rules through
//! `libohi18n.so`. Function names use the lower-case `OH_i18n_` prefix.
//! Available since API-level 22.
//!
//! Official `timezone` C API reference: <https://docs.openharmony.cn/pages/v5.0/en/application-dev/reference/apis-localization-kit/capi-timezone-h.md>.
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "api-22")]
#[link(name = "ohi18n")]
unsafe extern "C" {}

#[cfg(feature = "api-22")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-22")))]
mod i18n_ffi;
#[cfg(feature = "api-22")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-22")))]
pub use i18n_ffi::*;
