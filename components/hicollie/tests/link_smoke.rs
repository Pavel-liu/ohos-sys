#[cfg(feature = "api-12")]
#[test]
fn link_smoke_api_12() {
    let _f: unsafe extern "C" fn(
        ohos_hicollie_sys::OH_HiCollie_Task,
    ) -> ohos_hicollie_sys::HiCollie_ErrorCode = ohos_hicollie_sys::OH_HiCollie_Init_StuckDetection;
}

#[cfg(feature = "api-18")]
#[test]
fn link_smoke_api_18() {
    let _f: unsafe extern "C" fn(
        ohos_hicollie_sys::HiCollie_SetTimerParam,
        *mut core::ffi::c_int,
    ) -> ohos_hicollie_sys::HiCollie_ErrorCode = ohos_hicollie_sys::OH_HiCollie_SetTimer;
}
