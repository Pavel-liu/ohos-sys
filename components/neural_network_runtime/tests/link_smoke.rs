#[cfg(feature = "api-11")]
#[test]
fn link_smoke_api_11() {
    let _f: unsafe extern "C" fn(
        *const ohos_neural_network_runtime_sys::OH_NNModel,
    ) -> *mut ohos_neural_network_runtime_sys::OH_NNCompilation =
        ohos_neural_network_runtime_sys::OH_NNCompilation_Construct;
}
