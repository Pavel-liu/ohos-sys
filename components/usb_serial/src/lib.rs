//! Bindings to the OpenHarmony USB Serial DDK API.
//!
//! USB Serial DDK provides APIs for developing USB serial device drivers.
//! Available since API-level 18.
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "api-18")]
#[link(name = "usb_serial_ndk.z")]
unsafe extern "C" {}

#[cfg(feature = "api-18")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-18")))]
mod usb_serial_ffi;
#[cfg(feature = "api-18")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-18")))]
pub use usb_serial_ffi::*;
