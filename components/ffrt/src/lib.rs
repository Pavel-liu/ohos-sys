//! Bindings to the OpenHarmony FFRT (Function Flow Runtime) C API.
//!
//! FFRT provides a C API for task-based concurrent programming, including:
//! - Task submission with dependencies and QoS attributes
//! - Serial and concurrent queues
//! - Mutex, condition variable, and shared mutex (rwlock) synchronization
//! - Loops with epoll and timer support
//! - Sleep and yield functions
//! - Fiber (coroutine) support
//!
//! Available since API-level 10, with additional APIs added in later levels.
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(clippy::doc_markdown)]

#[cfg(feature = "api-10")]
#[link(name = "ffrt.z")]
unsafe extern "C" {}

#[cfg(feature = "api-10")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-10")))]
mod type_def;
#[cfg(feature = "api-10")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-10")))]
pub use type_def::*;

#[cfg(feature = "api-10")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-10")))]
mod task;
#[cfg(feature = "api-10")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-10")))]
pub use task::*;

#[cfg(feature = "api-10")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-10")))]
mod mutex;
#[cfg(feature = "api-10")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-10")))]
pub use mutex::*;

#[cfg(feature = "api-10")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-10")))]
mod condition_variable;
#[cfg(feature = "api-10")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-10")))]
pub use condition_variable::*;

#[cfg(feature = "api-10")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-10")))]
mod queue;
#[cfg(feature = "api-10")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-10")))]
pub use queue::*;

#[cfg(feature = "api-10")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-10")))]
mod sleep;
#[cfg(feature = "api-10")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-10")))]
pub use sleep::*;

#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
mod ffrt_loop;
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
pub use ffrt_loop::*;

#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
mod timer;
#[cfg(feature = "api-12")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-12")))]
pub use timer::*;

#[cfg(feature = "api-18")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-18")))]
mod shared_mutex;
#[cfg(feature = "api-18")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-18")))]
pub use shared_mutex::*;

#[cfg(feature = "api-20")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-20")))]
mod fiber;
#[cfg(feature = "api-20")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-20")))]
pub use fiber::*;
