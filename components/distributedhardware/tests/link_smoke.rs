#[cfg(feature = "api-20")]
#[test]
fn link_smoke_api_20() {
    let _f: unsafe extern "C" fn(*mut *mut core::ffi::c_char, *mut core::ffi::c_uint) -> i32 =
        ohos_distributedhardware_sys::OH_DeviceManager_GetLocalDeviceName;
}
