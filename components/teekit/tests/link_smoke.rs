#[cfg(feature = "api-20")]
#[test]
fn link_smoke_api_20() {
    let _initialize_context: unsafe extern "C" fn(
        *const core::ffi::c_char,
        *mut ohos_teekit_sys::TEEC_Context,
    ) -> ohos_teekit_sys::TEEC_Result = ohos_teekit_sys::TEEC_InitializeContext;
    let _finalize_context: unsafe extern "C" fn(*mut ohos_teekit_sys::TEEC_Context) =
        ohos_teekit_sys::TEEC_FinalizeContext;
    let _open_session: unsafe extern "C" fn(
        *mut ohos_teekit_sys::TEEC_Context,
        *mut ohos_teekit_sys::TEEC_Session,
        *const ohos_teekit_sys::TEEC_UUID,
        u32,
        *const core::ffi::c_void,
        *mut ohos_teekit_sys::TEEC_Operation,
        *mut u32,
    ) -> ohos_teekit_sys::TEEC_Result = ohos_teekit_sys::TEEC_OpenSession;
    let _invoke_command: unsafe extern "C" fn(
        *mut ohos_teekit_sys::TEEC_Session,
        u32,
        *mut ohos_teekit_sys::TEEC_Operation,
        *mut u32,
    ) -> ohos_teekit_sys::TEEC_Result = ohos_teekit_sys::TEEC_InvokeCommand;
}
