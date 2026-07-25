#[test]
fn link_smoke_api_10() {
    let _model_create: unsafe extern "C" fn() -> ohos_mindspore_sys::OH_AI_ModelHandle =
        ohos_mindspore_sys::OH_AI_ModelCreate;
    let _model_build_from_file: unsafe extern "C" fn(
        ohos_mindspore_sys::OH_AI_ModelHandle,
        *const core::ffi::c_char,
        ohos_mindspore_sys::OH_AI_ModelType,
        ohos_mindspore_sys::OH_AI_ContextHandle,
    ) -> ohos_mindspore_sys::OH_AI_Status = ohos_mindspore_sys::OH_AI_ModelBuildFromFile;
    let _tensor_create: unsafe extern "C" fn(
        *const core::ffi::c_char,
        ohos_mindspore_sys::OH_AI_DataType,
        *const i64,
        usize,
        *const core::ffi::c_void,
        usize,
    ) -> ohos_mindspore_sys::OH_AI_TensorHandle = ohos_mindspore_sys::OH_AI_TensorCreate;
    let _context_create: unsafe extern "C" fn() -> ohos_mindspore_sys::OH_AI_ContextHandle =
        ohos_mindspore_sys::OH_AI_ContextCreate;
}

#[cfg(feature = "api-11")]
#[test]
fn link_smoke_api_11_train_cfg() {
    let _train_cfg_create: unsafe extern "C" fn() -> ohos_mindspore_sys::OH_AI_TrainCfgHandle =
        ohos_mindspore_sys::OH_AI_TrainCfgCreate;
    let _train_cfg_destroy: unsafe extern "C" fn(*mut ohos_mindspore_sys::OH_AI_TrainCfgHandle) =
        ohos_mindspore_sys::OH_AI_TrainCfgDestroy;
}
