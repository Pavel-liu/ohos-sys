//! AVSessionKit bindings for OpenHarmony.
//!
//! This crate exposes raw C ABI bindings generated from
//! `multimedia/av_session/native_*.h`. It does not provide safe wrappers around
//! native session handles or callback registration.
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "api-23")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-23")))]
pub mod avcastcontroller;
#[cfg(feature = "api-13")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-13")))]
pub mod avmetadata;
#[cfg(feature = "api-23")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-23")))]
pub mod avplaybackstate;
#[cfg(feature = "api-23")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-23")))]
pub mod avqueueitem;
#[cfg(feature = "api-13")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-13")))]
pub mod avsession;
#[cfg(feature = "api-13")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-13")))]
pub mod avsession_base;
#[cfg(feature = "api-13")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-13")))]
pub mod avsession_errors;
#[cfg(feature = "api-23")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-23")))]
pub mod deviceinfo;
