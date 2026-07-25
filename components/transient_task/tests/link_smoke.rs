#[cfg(feature = "api-13")]
#[test]
fn link_smoke_api_13() {
    let _f: unsafe extern "C" fn(i32) -> i32 =
        ohos_transient_task_sys::OH_BackgroundTaskManager_CancelSuspendDelay;
}

#[cfg(feature = "api-20")]
#[test]
fn link_smoke_api_20() {
    let _f: unsafe extern "C" fn(
        *mut ohos_transient_task_sys::TransientTask_TransientTaskInfo,
    ) -> i32 = ohos_transient_task_sys::OH_BackgroundTaskManager_GetTransientTaskInfo;
}
