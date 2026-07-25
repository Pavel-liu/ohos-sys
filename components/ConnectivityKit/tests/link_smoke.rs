#[cfg(feature = "api-13")]
#[test]
fn link_smoke_bluetooth_api_13() {
    let _f: unsafe extern "C" fn(
        *mut ohos_connectivitykit_sys::bluetooth::Bluetooth_SwitchState,
    ) -> ohos_connectivitykit_sys::bluetooth::Bluetooth_ResultCode =
        ohos_connectivitykit_sys::bluetooth::OH_Bluetooth_GetBluetoothSwitchState;
}

#[cfg(feature = "api-13")]
#[test]
fn link_smoke_wifi_api_13() {
    let _f: unsafe extern "C" fn(*mut bool) -> ohos_connectivitykit_sys::wifi::Wifi_ResultCode =
        ohos_connectivitykit_sys::wifi::OH_Wifi_IsWifiEnabled;
}

#[cfg(feature = "api-21")]
#[test]
fn link_smoke_wifi_api_21() {
    let _f: unsafe extern "C" fn(
        *mut core::ffi::c_char,
        *mut u32,
    ) -> ohos_connectivitykit_sys::wifi::Wifi_ResultCode =
        ohos_connectivitykit_sys::wifi::OH_Wifi_GetDeviceMacAddress;
}
