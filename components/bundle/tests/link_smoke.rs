#[cfg(feature = "api-11")]
#[test]
fn link_smoke_api_11() {
    use ohos_bundle_sys::*;
    let _: OH_BundleInfo = std::ptr::null_mut();
    let _: OH_ApplicationInfo = std::ptr::null_mut();
}
