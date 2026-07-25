#[cfg(feature = "api-14")]
#[test]
fn link_smoke_api_14() {
    let _f: unsafe extern "C" fn(
        *mut ohos_dataprotectionkit_sys::DLP_FileAccess,
        *mut u32,
    ) -> ohos_dataprotectionkit_sys::DLP_ErrCode =
        ohos_dataprotectionkit_sys::OH_DLP_GetDlpPermissionInfo;
}
