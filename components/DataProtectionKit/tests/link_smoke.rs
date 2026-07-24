#[cfg(feature = "api-14")]
#[test]
fn link_smoke_api_14() {
    use ohos_dataprotectionkit_sys::*;
    let _: DLP_ErrCode = DLP_ErrCode::ERR_OH_SUCCESS;
    let _: DLP_FileAccess = DLP_FileAccess::NO_PERMISSION;
}
