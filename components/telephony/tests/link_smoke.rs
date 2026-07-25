#![cfg(feature = "api-13")]

#[test]
fn link_smoke_data_api_13() {
    let _f: unsafe extern "C" fn() -> i32 =
        ohos_telephony_sys::OH_Telephony_GetDefaultCellularDataSlotId;
}

#[test]
fn link_smoke_radio_api_13() {
    let _f: unsafe extern "C" fn(
        *mut ohos_telephony_sys::Telephony_NetworkState,
    ) -> ohos_telephony_sys::Telephony_RadioResult =
        ohos_telephony_sys::OH_Telephony_GetNetworkState;
}
