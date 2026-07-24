use ohos_hidebug_sys as hidebug;

#[test]
fn link_smoke() {
    let _f: unsafe extern "C" fn() -> f64 = hidebug::OH_HiDebug_GetSystemCpuUsage;
}

#[cfg(feature = "api-20")]
#[test]
fn link_smoke_api20() {
    let _f: unsafe extern "C" fn() -> hidebug::HiDebug_Backtrace_Object =
        hidebug::OH_HiDebug_CreateBacktraceObject;
}

#[cfg(feature = "api-22")]
#[test]
fn link_smoke_api22() {
    let _f: unsafe extern "C" fn(
        *mut hidebug::HiDebug_ProcessSamplerConfig,
        hidebug::OH_HiDebug_ThreadLiteSamplingCallback,
    ) -> hidebug::HiDebug_ErrorCode = hidebug::OH_HiDebug_RequestThreadLiteSampling;
}

#[cfg(feature = "api-23")]
#[test]
fn link_smoke_api23() {
    let _set: unsafe extern "C" fn(hidebug::HiDebug_CrashObjType, *mut core::ffi::c_void) -> u64 =
        hidebug::OH_HiDebug_SetCrashObj;
    let _reset: unsafe extern "C" fn(u64) = hidebug::OH_HiDebug_ResetCrashObj;
}
