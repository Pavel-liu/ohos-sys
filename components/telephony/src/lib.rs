//! Rust bindings to OpenHarmony TelephonyKit native APIs.
//!
//! TelephonyKit provides C APIs for cellular data and radio network state.
//! Available since API-level 13.
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(clippy::doc_markdown)]
#![cfg(feature = "api-13")]
#![cfg_attr(docsrs, doc(cfg(feature = "api-13")))]

#[cfg(feature = "api-13")]
#[link(name = "telephony_data")]
unsafe extern "C" {}

#[cfg(feature = "api-13")]
#[link(name = "telephony_radio")]
unsafe extern "C" {}

#[cfg(feature = "api-13")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-13")))]
mod telephony_data_ffi;

#[cfg(feature = "api-13")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-13")))]
mod telephony_radio_ffi;

#[cfg(feature = "api-13")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-13")))]
pub use telephony_data_ffi::*;

#[cfg(feature = "api-13")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-13")))]
pub use telephony_radio_ffi::*;
