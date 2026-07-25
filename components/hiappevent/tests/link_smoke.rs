#[cfg(feature = "api-12")]
#[test]
fn link_smoke_api_12() {
    let _f: unsafe extern "C" fn() -> ohos_hiappevent_sys::ParamList =
        ohos_hiappevent_sys::OH_HiAppEvent_CreateParamList;
}
