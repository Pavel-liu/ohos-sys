#[cfg(feature = "api-11")]
#[test]
fn link_smoke_api_11() {
    let _f: unsafe extern "C" fn(
        *const ohos_ark_runtime_sys::JSVM_InitOptions,
    ) -> ohos_ark_runtime_sys::JSVM_Status = ohos_ark_runtime_sys::OH_JSVM_Init;
}
