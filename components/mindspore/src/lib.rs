//! Bindings to the `OpenHarmony` `MindSpore Lite` NDK API.
//!
//! `MindSpore Lite` provides model, tensor, context, status, data type, and
//! format APIs through `libmindspore_lite_ndk.so`. The SDK headers document
//! some APIs as available since API-level 9, but this crate follows the root
//! `ohos-sys` feature chain and exposes `MindSpore` from API-level 10 onward.
//!
//! Official `MindSpore Lite` C API reference: <https://docs.openharmony.cn/pages/v5.0/en/application-dev/reference/apis-mindspore-lite-kit/_mind_spore.md>.
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "api-10")]
#[link(name = "mindspore_lite_ndk")]
unsafe extern "C" {}

#[cfg(feature = "api-10")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-10")))]
mod mindspore_ffi;
#[cfg(feature = "api-10")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-10")))]
pub use mindspore_ffi::*;
