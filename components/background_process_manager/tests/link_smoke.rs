#[cfg(feature = "api-17")]
#[test]
fn link_smoke_api_17() {
    let _f: unsafe extern "C" fn(
        core::ffi::c_int,
        ohos_background_process_manager_sys::BackgroundProcessManager_ProcessPriority,
    ) -> core::ffi::c_int =
        ohos_background_process_manager_sys::OH_BackgroundProcessManager_SetProcessPriority;
}
