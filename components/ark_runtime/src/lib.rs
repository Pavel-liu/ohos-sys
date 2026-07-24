//! Bindings to the OpenHarmony Ark Runtime (JSVM) API.
//!
//! The Ark Runtime provides the JSVM (JavaScript Virtual Machine) API for
//! embedding and interacting with a JavaScript engine. Available since API-level 11.
//!
//! See also the [JSVM NDK guide] for usage details.
//!
//! [JSVM NDK guide]: https://docs.openharmony.cn/pages/v5.0/en/application-dev/arkcore/jsvm/jsvm-guidelines.md
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "api-11")]
#[link(name = "jsvm")]
unsafe extern "C" {}

#[cfg(feature = "api-11")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-11")))]
mod jsvm_ffi;
#[cfg(feature = "api-11")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-11")))]
pub use jsvm_ffi::*;
