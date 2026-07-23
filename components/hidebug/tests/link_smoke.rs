use std::ptr;

use hidebug_sys as hidebug;

#[test]
fn link_smoke() {
    unsafe {
        let _ = hidebug::OH_HiDebug_GetSystemCpuUsage();
    }
}

#[cfg(feature = "api-20")]
#[test]
fn link_smoke_api20() {
    unsafe {
        let _ = hidebug::OH_HiDebug_CreateBacktraceObject();
    }
}

#[cfg(feature = "api-22")]
#[test]
fn link_smoke_api22() {
    let _ = hidebug::HiDebug_ProcessSamplerConfig {
        tids: ptr::null_mut(),
        size: 0,
        frequency: 0,
        duration: 0,
        reserved: 0,
    };
}

#[cfg(feature = "api-23")]
#[test]
fn link_smoke_api23() {
    let _ = hidebug::HiDebug_CrashObjType::HIDEBUG_CRASHOBJ_STRING;
}
