#[cfg(feature = "api-11")]
#[test]
fn link_smoke_api_11() {
    use ohos_drm_sys::*;
    let _: OH_DrmSession = std::ptr::null_mut();
    let _: OH_DrmMediaKeySession = std::ptr::null_mut();
}
