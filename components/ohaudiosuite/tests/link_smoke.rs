#[cfg(feature = "api-22")]
#[test]
fn link_smoke_api_22() {
    let _create: unsafe extern "C" fn(
        *mut *mut ohos_ohaudiosuite_sys::OH_AudioSuiteEngine,
    ) -> ohos_ohaudiosuite_sys::OH_AudioSuite_Result =
        ohos_ohaudiosuite_sys::OH_AudioSuiteEngine_Create;
    let _destroy: unsafe extern "C" fn(
        *mut ohos_ohaudiosuite_sys::OH_AudioSuiteEngine,
    ) -> ohos_ohaudiosuite_sys::OH_AudioSuite_Result =
        ohos_ohaudiosuite_sys::OH_AudioSuiteEngine_Destroy;
    let _create_node: unsafe extern "C" fn(
        *mut ohos_ohaudiosuite_sys::OH_AudioSuitePipeline,
        *mut ohos_ohaudiosuite_sys::OH_AudioNodeBuilder,
        *mut *mut ohos_ohaudiosuite_sys::OH_AudioNode,
    ) -> ohos_ohaudiosuite_sys::OH_AudioSuite_Result =
        ohos_ohaudiosuite_sys::OH_AudioSuiteEngine_CreateNode;
}
