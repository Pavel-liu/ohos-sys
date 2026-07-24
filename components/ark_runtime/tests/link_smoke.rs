#[cfg(feature = "api-11")]
#[test]
fn link_smoke_api_11() {
    use ohos_ark_runtime_sys::*;
    let _ = JSVM_INVALID;
    let _ = JSVM_GENERIC_FAILURE;
    let _: OH_JSVM_Value = std::ptr::null_mut();
    let _: OH_JSVM_Env = std::ptr::null_mut();
}
