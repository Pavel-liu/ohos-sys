#[cfg(feature = "api-12")]
#[test]
fn link_smoke_api_12() {
    let _f: unsafe extern "C" fn(bool) -> *mut ohos_native_display_soloist_sys::OH_DisplaySoloist =
        ohos_native_display_soloist_sys::OH_DisplaySoloist_Create;
}
