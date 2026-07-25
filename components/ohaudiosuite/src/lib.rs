//! Bindings to the `OpenHarmony` `OHAudioSuite` API.
//!
//! `OHAudioSuite` provides raw audio-suite engine and audio-node FFI. Available
//! since API-level 22.
//!
//! Official `OHAudioSuite` C API reference: <https://docs.openharmony.cn/pages/v5.0/en/application-dev/reference/apis-audio-kit/capi-ohaudiosuite.md>.
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "api-22")]
#[link(name = "ohaudiosuite")]
unsafe extern "C" {}

#[cfg(feature = "api-22")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-22")))]
mod native_audio_suite_base_ffi;
#[cfg(feature = "api-22")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-22")))]
pub use native_audio_suite_base_ffi::*;

#[cfg(feature = "api-22")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-22")))]
mod native_audio_suite_engine_ffi;
#[cfg(feature = "api-22")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-22")))]
pub use native_audio_suite_engine_ffi::*;
