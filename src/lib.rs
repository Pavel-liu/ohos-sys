//! Ohos-sys
//!
//! This crate provides Raw FFI bindings to the native API of OpenHarmonyOS (`target_env = "ohos"`).
//! Each module corresponds to one OpenHarmony API feature, and is gated behind a cargo feature.
//! If you are an application developer, you probably do not want to use this crate directly,
//! and instead want to use a higher-level API built on top of this crate.
//!
//! Note: There are currently still quite a few missing bindings, which will slowly be added.
//!
//! ## Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "arkui")]
#[cfg_attr(docsrs, doc(cfg(feature = "arkui")))]
pub use arkui_sys as arkui;

#[cfg(feature = "asset-store")]
#[cfg_attr(docsrs, doc(cfg(feature = "asset-store")))]
pub use ohos_asset_store_sys as asset_store;

#[cfg(feature = "basic-services-kit")]
#[cfg_attr(docsrs, doc(cfg(feature = "basic-services-kit")))]
pub use ohos_basic_services_kit_sys as basic_services_kit;

#[cfg(feature = "crypto")]
#[cfg_attr(docsrs, doc(cfg(feature = "crypto")))]
pub use ohos_crypto_sys as crypto;

#[cfg(feature = "deviceinfo")]
#[cfg_attr(docsrs, doc(cfg(feature = "deviceinfo")))]
pub use ohos_deviceinfo_sys as deviceinfo;

#[cfg(feature = "distributedhardware")]
#[cfg_attr(docsrs, doc(cfg(feature = "distributedhardware")))]
pub use ohos_distributedhardware_sys as distributedhardware;

#[cfg(feature = "drawing")]
#[cfg_attr(docsrs, doc(cfg(feature = "drawing")))]
pub use ohos_drawing_sys as drawing;

#[cfg(feature = "hilog")]
#[cfg_attr(docsrs, doc(cfg(feature = "hilog")))]
pub use hilog_sys as hilog;

#[cfg(feature = "huks")]
#[cfg_attr(docsrs, doc(cfg(feature = "huks")))]
pub use ohos_huks_sys as huks;

#[cfg(feature = "hitrace")]
#[cfg_attr(docsrs, doc(cfg(feature = "hitrace")))]
pub use hitrace_sys as hitrace;

#[cfg(feature = "inputmethod")]
#[cfg_attr(docsrs, doc(cfg(feature = "inputmethod")))]
pub use ohos_ime_sys as inputmethod;

#[cfg(feature = "ohaudio")]
#[cfg_attr(docsrs, doc(cfg(feature = "ohaudio")))]
pub use ohaudio_sys as ohaudio;

#[cfg(feature = "ohaudiosuite")]
#[cfg_attr(docsrs, doc(cfg(feature = "ohaudiosuite")))]
pub use ohos_ohaudiosuite_sys as ohaudiosuite;

#[cfg(feature = "ohcamera")]
#[cfg_attr(docsrs, doc(cfg(feature = "ohcamera")))]
pub use ohos_ohcamera_sys as ohcamera;

#[cfg(feature = "web")]
#[cfg_attr(docsrs, doc(cfg(feature = "web")))]
pub use arkweb_sys as web;

#[cfg(feature = "ipckit")]
#[cfg_attr(docsrs, doc(cfg(feature = "ipckit")))]
pub use ohos_ipckit_sys as ipckit;

#[cfg(feature = "locationkit")]
#[cfg_attr(docsrs, doc(cfg(feature = "locationkit")))]
pub use ohos_locationkit_sys as locationkit;

#[cfg(feature = "netmanager")]
#[cfg_attr(docsrs, doc(cfg(feature = "netmanager")))]
pub use ohos_netmanager_sys as netmanager;

#[cfg(feature = "net_ssl")]
#[cfg_attr(docsrs, doc(cfg(feature = "net_ssl")))]
pub use ohos_net_ssl_sys as net_ssl;

#[cfg(feature = "netstack")]
#[cfg_attr(docsrs, doc(cfg(feature = "netstack")))]
pub use ohos_netstack_sys as netstack;

#[cfg(feature = "multimodal-input")]
#[cfg_attr(docsrs, doc(cfg(feature = "multimodal-input")))]
pub use ohos_input_sys as multimodal_input;

pub mod multimedia;

#[cfg(feature = "napi")]
#[cfg_attr(docsrs, doc(cfg(feature = "napi")))]
pub mod napi;

#[cfg(feature = "abilitykit")]
#[cfg_attr(docsrs, doc(cfg(feature = "abilitykit")))]
pub use ohos_abilitykit_sys as abilitykit;

#[cfg(feature = "pasteboard")]
#[cfg_attr(docsrs, doc(cfg(feature = "pasteboard")))]
pub use ohos_pasteboard_sys as pasteboard;

#[cfg(feature = "rawfile")]
#[cfg_attr(docsrs, doc(cfg(feature = "rawfile")))]
pub use ohos_rawfile_sys as rawfile;

#[cfg(feature = "resourcemanager")]
#[cfg_attr(docsrs, doc(cfg(feature = "resourcemanager")))]
pub use ohos_resourcemanager_sys as resourcemanager;

#[cfg(feature = "rdb")]
#[cfg_attr(docsrs, doc(cfg(feature = "rdb")))]
pub use ohos_rdb_sys as rdb;

#[cfg(feature = "sensors")]
#[cfg_attr(docsrs, doc(cfg(feature = "sensors")))]
pub use ohos_sensors_sys as sensors;

#[cfg(feature = "udmf")]
#[cfg_attr(docsrs, doc(cfg(feature = "udmf")))]
pub use udmf_sys as udmf;

#[cfg(feature = "native_buffer")]
#[cfg_attr(docsrs, doc(cfg(feature = "native_buffer")))]
pub use ohos_window_sys::native_buffer;

#[cfg(feature = "native_image")]
#[cfg_attr(docsrs, doc(cfg(feature = "native_image")))]
pub use ohos_window_sys::native_image;

#[cfg(feature = "native_window")]
#[cfg_attr(docsrs, doc(cfg(feature = "native_window")))]
pub use ohos_window_sys::native_window;

// It's just one function, so we don't feature guard this.
pub mod syscap;

#[cfg(feature = "vsync")]
#[cfg_attr(docsrs, doc(cfg(feature = "vsync")))]
pub use ohos_vsync_sys as vsync;

#[cfg(feature = "xcomponent")]
#[cfg_attr(docsrs, doc(cfg(feature = "xcomponent")))]
pub use xcomponent_sys as xcomponent;

#[cfg(feature = "window_manager")]
#[cfg_attr(docsrs, doc(cfg(feature = "window_manager")))]
pub use ohos_window_manager_sys as window_manager;

#[cfg(feature = "dataprotectionkit")]
#[cfg_attr(docsrs, doc(cfg(feature = "dataprotectionkit")))]
pub use ohos_dataprotectionkit_sys as dataprotectionkit;

#[cfg(feature = "telephony")]
#[cfg_attr(docsrs, doc(cfg(feature = "telephony")))]
pub use ohos_telephony_sys as telephony;

#[cfg(feature = "usb")]
#[cfg_attr(docsrs, doc(cfg(feature = "usb")))]
pub use ohos_usb_sys as usb;

#[cfg(feature = "ffrt")]
#[cfg_attr(docsrs, doc(cfg(feature = "ffrt")))]
pub use ohos_ffrt_sys as ffrt;

#[cfg(feature = "connectivitykit")]
#[cfg_attr(docsrs, doc(cfg(feature = "connectivitykit")))]
pub use ohos_connectivitykit_sys as connectivitykit;

#[cfg(feature = "filemanagement")]
#[cfg_attr(docsrs, doc(cfg(feature = "filemanagement")))]
pub use ohos_filemanagement_sys as filemanagement;

#[cfg(feature = "hid")]
#[cfg_attr(docsrs, doc(cfg(feature = "hid")))]
pub use ohos_hid_sys as hid;

#[cfg(feature = "native-color-space-manager")]
#[cfg_attr(docsrs, doc(cfg(feature = "native-color-space-manager")))]
pub use ohos_native_color_space_manager_sys as native_color_space_manager;

#[cfg(feature = "native-display-soloist")]
#[cfg_attr(docsrs, doc(cfg(feature = "native-display-soloist")))]
pub use ohos_native_display_soloist_sys as native_display_soloist;

#[cfg(feature = "native-effect")]
#[cfg_attr(docsrs, doc(cfg(feature = "native-effect")))]
pub use ohos_native_effect_sys as native_effect;

#[cfg(feature = "native-fence")]
#[cfg_attr(docsrs, doc(cfg(feature = "native-fence")))]
pub use ohos_native_fence_sys as native_fence;

#[cfg(feature = "purgeable-memory")]
#[cfg_attr(docsrs, doc(cfg(feature = "purgeable-memory")))]
pub use ohos_purgeable_memory_sys as purgeable_memory;

#[cfg(feature = "scsi-peripheral")]
#[cfg_attr(docsrs, doc(cfg(feature = "scsi-peripheral")))]
pub use ohos_scsi_peripheral_sys as scsi_peripheral;

#[cfg(feature = "transient-task")]
#[cfg_attr(docsrs, doc(cfg(feature = "transient-task")))]
pub use ohos_transient_task_sys as transient_task;

#[cfg(feature = "usb-serial")]
#[cfg_attr(docsrs, doc(cfg(feature = "usb-serial")))]
pub use ohos_usb_serial_sys as usb_serial;

#[cfg(feature = "gamecontrollerkit")]
#[cfg_attr(docsrs, doc(cfg(feature = "gamecontrollerkit")))]
pub use ohos_gamecontrollerkit_sys as gamecontrollerkit;

#[cfg(feature = "notificationkit")]
#[cfg_attr(docsrs, doc(cfg(feature = "notificationkit")))]
pub use ohos_notificationkit_sys as notificationkit;

#[cfg(feature = "ark-runtime")]
#[cfg_attr(docsrs, doc(cfg(feature = "ark-runtime")))]
pub use ohos_ark_runtime_sys as ark_runtime;

#[cfg(feature = "bundle")]
#[cfg_attr(docsrs, doc(cfg(feature = "bundle")))]
pub use ohos_bundle_sys as bundle;

#[cfg(feature = "hiappevent")]
#[cfg_attr(docsrs, doc(cfg(feature = "hiappevent")))]
pub use ohos_hiappevent_sys as hiappevent;

#[cfg(feature = "i18n")]
#[cfg_attr(docsrs, doc(cfg(feature = "i18n")))]
pub use ohos_i18n_sys as i18n;

#[cfg(feature = "drm")]
#[cfg_attr(docsrs, doc(cfg(feature = "drm")))]
pub use ohos_drm_sys as drm;

#[cfg(feature = "image-effect")]
#[cfg_attr(docsrs, doc(cfg(feature = "image-effect")))]
pub use ohos_image_effect_sys as image_effect;

#[cfg(feature = "mindspore")]
#[cfg_attr(docsrs, doc(cfg(feature = "mindspore")))]
pub use ohos_mindspore_sys as mindspore;

#[cfg(feature = "neural-network-runtime")]
#[cfg_attr(docsrs, doc(cfg(feature = "neural-network-runtime")))]
pub use ohos_neural_network_runtime_sys as neural_network_runtime;

#[cfg(feature = "teekit")]
#[cfg_attr(docsrs, doc(cfg(feature = "teekit")))]
pub use ohos_teekit_sys as teekit;
