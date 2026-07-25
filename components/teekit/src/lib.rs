//! Bindings to the OpenHarmony TEEKit tee_client API.
//!
//! TEEKit provides Rich Execution Environment client APIs for accessing trusted
//! applications in a Trusted Execution Environment. This crate intentionally
//! covers only `TEEKit/tee_client/*` headers and does not bind `TEEKit/tee/*`.
//! Available since API-level 20.
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "api-20")]
#[link(name = "teec")]
unsafe extern "C" {}

#[cfg(feature = "api-20")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-20")))]
mod tee_client_ffi;
#[cfg(feature = "api-20")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-20")))]
pub use tee_client_ffi::*;
