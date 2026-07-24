//! Bindings to the OpenHarmony GameControllerKit (game controller management) API.
//!
//! GameControllerKit provides APIs for managing game controllers, including
//! connecting, disconnecting, and reading input from game controllers.
//! Available since API-level 21.
//!
//! See also the [GameControllerKit NDK guide] for usage details.
//!
//! [GameControllerKit NDK guide]: https://docs.openharmony.cn/pages/v5.0/en/application-dev/game/gamecontrollerkit-guidelines.md
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "api-21")]
#[link(name = "ohgame_controller.z")]
unsafe extern "C" {}

#[cfg(feature = "api-21")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-21")))]
mod gamecontrollerkit_ffi;
#[cfg(feature = "api-21")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-21")))]
pub use gamecontrollerkit_ffi::*;
