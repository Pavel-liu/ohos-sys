//! Bindings to the OpenHarmony Neural Network Runtime API.
//!
//! The Neural Network Runtime provides APIs for neural network model compilation
//! and inference execution. Available since API-level 11.
//!
//! See also the [Neural Network Runtime NDK guide] for usage details.
//!
//! [Neural Network Runtime NDK guide]: https://docs.openharmony.cn/pages/v5.0/en/application-dev/ai/neural-network-runtime/neural-network-runtime-guidelines.md
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "api-11")]
#[link(name = "neural_network_core")]
unsafe extern "C" {}

#[cfg(feature = "api-11")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-11")))]
mod neural_network_runtime_ffi;
#[cfg(feature = "api-11")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-11")))]
pub use neural_network_runtime_ffi::*;
