//! Bindings to the OpenHarmony HiCollie (thread stuck and jank detection) API.
//!
//! HiCollie provides APIs for detecting thread stuck and jank events.
//! Available since API-level 12.
//!
//! See also the [HiCollie NDK guide] for usage details.
//!
//! [HiCollie NDK guide]: https://docs.openharmony.cn/pages/v5.0/en/application-dev/dfx/hicollie-guidelines-ndk.md
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "api-12")]
#[link(name = "ohhicollie")]
unsafe extern "C" {}

#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
mod hicollie_ffi;
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
pub use hicollie_ffi::*;
