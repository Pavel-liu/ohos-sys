//! Bindings to the OpenHarmony NotificationKit (notification management) API.
//!
//! NotificationKit provides APIs for publishing, canceling, and managing
//! notifications. Available since API-level 13.
//!
//! See also the [Notification NDK guide] for usage details.
//!
//! [Notification NDK guide]: https://docs.openharmony.cn/pages/v5.0/en/application-dev/notification/notification-ndk-guidelines.md
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "api-13")]
#[link(name = "ohnotification")]
unsafe extern "C" {}

#[cfg(feature = "api-13")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-13")))]
mod notificationkit_ffi;
#[cfg(feature = "api-13")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-13")))]
pub use notificationkit_ffi::*;
