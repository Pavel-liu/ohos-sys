#[cfg(feature = "api-11")]
#[test]
fn link_smoke_api_11() {
    let _f: unsafe extern "C" fn() -> i32 = ohos_hid_sys::OH_Hid_Init;
}
