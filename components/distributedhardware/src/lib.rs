//! Bindings to the `OpenHarmony` `DistributedHardware` `DeviceManager` API.
//!
//! `DeviceManager` provides APIs to obtain trusted-device and local-device
//! information. Available since API-level 20.
//!
//! Official `DeviceManager` C API reference: <https://docs.openharmony.cn/pages/v5.0/en/application-dev/reference/apis-distributedservice-kit/capi-oh-device-manager-h.md>.
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "api-20")]
#[link(name = "devicemanager_ndk")]
unsafe extern "C" {}

#[cfg(feature = "api-20")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-20")))]
mod distributedhardware_ffi;
#[cfg(feature = "api-20")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-20")))]
pub use distributedhardware_ffi::*;
