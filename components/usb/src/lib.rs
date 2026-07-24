//! Bindings to the OpenHarmony USB DDK (Universal Serial Bus Driver Development Kit) API.
//!
//! USB DDK provides APIs for developing USB device drivers, including opening and
//! closing USB interfaces, performing data transfer over USB pipes, and control transfer.
//! Available since API-level 10.
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "api-10")]
#[link(name = "usb_ndk.z")]
unsafe extern "C" {}

#[cfg(feature = "api-10")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-10")))]
mod usb_ffi;
#[cfg(feature = "api-10")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-10")))]
pub use usb_ffi::*;
