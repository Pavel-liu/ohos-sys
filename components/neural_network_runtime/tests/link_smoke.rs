#[cfg(feature = "api-11")]
#[test]
fn link_smoke_api_11() {
    use ohos_neural_network_runtime_sys::*;
    let _: OH_NNCompilation = std::ptr::null_mut();
    let _: OH_NNExecutor = std::ptr::null_mut();
    let _: OH_NNModel = std::ptr::null_mut();
}
