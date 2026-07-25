#[cfg(feature = "api-13")]
#[test]
fn link_smoke_api_13() {
    let _f: unsafe extern "C" fn(
        ohos_native_color_space_manager_sys::ColorSpaceName,
    ) -> *mut ohos_native_color_space_manager_sys::OH_NativeColorSpaceManager =
        ohos_native_color_space_manager_sys::OH_NativeColorSpaceManager_CreateFromName;
}
