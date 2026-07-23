//! Bindings to the OpenHarmony HiDebug (debugging and profiling) API.
//!
//! HiDebug provides APIs for collecting CPU usage, memory information, and
//! performing application trace capture. Available since API-level 12.
//!
//! See also the [HiDebug NDK guide] for usage details.
//!
//! [HiDebug NDK guide]: https://docs.openharmony.cn/pages/v5.0/en/application-dev/dfx/hidebug-guidelines-ndk.md
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "api-12")]
#[link(name = "ohhidebug")]
unsafe extern "C" {}

#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
mod hidebug_ffi;
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
pub use hidebug_ffi::*;
