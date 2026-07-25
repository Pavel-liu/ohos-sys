//! Bindings to the `OpenHarmony` `NativeFence` API.
//!
//! `NativeFence` provides APIs for validating, waiting on, and closing native
//! fence file descriptors. Available since `API-level 20`.
//!
//! Official `NativeFence` C API reference: <https://docs.openharmony.cn/pages/v5.0/en/application-dev/reference/apis-arkgraphics2d/capi-native-fence-h.md>.
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "api-20")]
#[link(name = "native_fence")]
unsafe extern "C" {}

#[cfg(feature = "api-20")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-20")))]
mod native_fence_ffi;
#[cfg(feature = "api-20")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-20")))]
pub use native_fence_ffi::*;
