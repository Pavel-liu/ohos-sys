#[cfg(feature = "api-20")]
#[test]
fn link_smoke_api_20() {
    let _is_valid: unsafe extern "C" fn(core::ffi::c_int) -> bool =
        ohos_native_fence_sys::OH_NativeFence_IsValid;
    let _wait: unsafe extern "C" fn(core::ffi::c_int, u32) -> bool =
        ohos_native_fence_sys::OH_NativeFence_Wait;
    let _close: unsafe extern "C" fn(core::ffi::c_int) =
        ohos_native_fence_sys::OH_NativeFence_Close;
}
