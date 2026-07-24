//! Bindings to the OpenHarmony DataProtectionKit (DLP permission) API.
//!
//! DataProtectionKit provides APIs for accessing data loss prevention (DLP) files,
//! including getting DLP permission info, checking sandbox status, and managing
//! sandbox application configuration. Available since API-level 14.
//!
//! See also the [DataProtectionKit NDK guide] for usage details.
//!
//! [DataProtectionKit NDK guide]: https://docs.openharmony.cn/pages/v5.0/en/application-dev/security/dlp/dlp-ndk-guidelines.md
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code,
    improper_ctypes,
    clippy::deprecated_semver,
    clippy::doc_markdown
)]

#[cfg(feature = "api-14")]
#[link(name = "ohdlp_permission")]
unsafe extern "C" {}

#[cfg(feature = "api-14")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-14")))]
mod dataprotectionkit_ffi;
#[cfg(feature = "api-14")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-14")))]
pub use dataprotectionkit_ffi::*;
