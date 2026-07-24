//! Bindings to the OpenHarmony multimedia DRM Framework API.
//!
//! The DRM Framework provides APIs for digital rights management, including
//! media key session management and DRM module configuration.
//! Available since API-level 11.
//!
//! See also the [DRM NDK guide] for usage details.
//!
//! [DRM NDK guide]: https://docs.openharmony.cn/pages/v5.0/en/application-dev/media/drm/drm-guidelines.md
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "api-11")]
#[link(name = "native_drm")]
unsafe extern "C" {}

#[cfg(feature = "api-11")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-11")))]
mod drm_ffi;
#[cfg(feature = "api-11")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-11")))]
pub use drm_ffi::*;
