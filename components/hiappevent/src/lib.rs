//! Bindings to the OpenHarmony HiAppEvent (application event logging) API.
//!
//! HiAppEvent provides APIs for logging application events, including writing,
//! configuring, and managing event data. Available since API-level 12.
//!
//! See also the [HiAppEvent NDK guide] for usage details.
//!
//! [HiAppEvent NDK guide]: https://docs.openharmony.cn/pages/v5.0/en/application-dev/dfx/hiappevent-guidelines-ndk.md
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "api-12")]
#[link(name = "hiappevent_ndk.z")]
unsafe extern "C" {}

#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
mod hiappevent_ffi;
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
pub use hiappevent_ffi::*;
