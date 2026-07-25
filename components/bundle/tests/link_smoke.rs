#[cfg(feature = "api-21")]
#[test]
fn link_smoke_api_21() {
    let _f: unsafe extern "C" fn(
        *mut ohos_bundle_sys::OH_NativeBundle_AbilityResourceInfo,
        *mut *mut core::ffi::c_char,
    ) -> ohos_bundle_sys::BundleManager_ErrorCode = ohos_bundle_sys::OH_NativeBundle_GetBundleName;
}
