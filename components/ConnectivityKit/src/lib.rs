//! Rust bindings to OpenHarmony ConnectivityKit native APIs.
//!
//! ConnectivityKit provides C APIs for querying bluetooth and wifi switch status.
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
#[link(name = "bluetooth_ndk")]
unsafe extern "C" {}

#[cfg(feature = "api-13")]
#[link(name = "wifi_ndk")]
unsafe extern "C" {}

#[cfg(feature = "api-13")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-13")))]
pub mod bluetooth;

#[cfg(feature = "api-13")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-13")))]
pub mod wifi;
