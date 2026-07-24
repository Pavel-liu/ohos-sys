//! Bindings to the OpenHarmony SCSI Peripheral DDK API.
//!
//! SCSI Peripheral DDK provides APIs for developing SCSI peripheral device drivers.
//! Available since API-level 18.
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "api-18")]
#[link(name = "scsi.z")]
unsafe extern "C" {}

#[cfg(feature = "api-18")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-18")))]
mod scsi_peripheral_ffi;
#[cfg(feature = "api-18")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-18")))]
pub use scsi_peripheral_ffi::*;
