//! Bindings to the OpenHarmony `DeviceCertificateKit` certificate manager API.
//!
//! `DeviceCertificateKit` provides application certificate and USB key certificate
//! lookup APIs through `libohcert_manager.z.so`. Available since API-level 22.
//!
//! The upstream header documents `libohcert_manager.so`, but the OpenHarmony
//! native `SDK` ships and links the library as `libohcert_manager.z.so`.
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "api-22")]
#[link(name = "ohcert_manager.z")]
unsafe extern "C" {}

#[cfg(feature = "api-22")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-22")))]
mod device_certificate_ffi;
#[cfg(feature = "api-22")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-22")))]
pub use device_certificate_ffi::*;
