//! Bindings to the OpenHarmony multimedia Image Effect API.
//!
//! Image Effect provides APIs for applying image filters and effects,
//! including filter chain construction and effect processing.
//! Available since API-level 12.
//!
//! See also the [Image Effect NDK guide] for usage details.
//!
//! [Image Effect NDK guide]: https://docs.openharmony.cn/pages/v5.0/en/application-dev/media/image-effect/image-effect-guidelines.md
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "api-12")]
#[link(name = "image_effect")]
unsafe extern "C" {}

#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
mod image_effect_ffi;
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
pub use image_effect_ffi::*;
