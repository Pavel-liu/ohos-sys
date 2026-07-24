#[cfg(feature = "api-12")]
#[test]
fn link_smoke_api_12() {
    use ohos_hicollie_sys::*;
    let _ = HICOLLIE_SUCCESS;
    let _ = HICOLLIE_WRONG_THREAD_CONTEXT;
    let _ = HICOLLIE_REMOTE_FAILED;
    let _: OH_HiCollie_Task = None;
    let _: OH_HiCollie_BeginFunc = None;
    let _: OH_HiCollie_EndFunc = None;
    let _ = HiCollie_DetectionParam {
        sampleStackTriggerTime: 0,
        reserved: 0,
    };
}

#[cfg(feature = "api-18")]
#[test]
fn link_smoke_api_18() {
    use ohos_hicollie_sys::*;
    let _ = HICOLLIE_INVALID_TIMER_NAME;
    let _ = HICOLLIE_INVALID_TIMEOUT_VALUE;
    let _ = HICOLLIE_WRONG_PROCESS_CONTEXT;
    let _ = HICOLLIE_WRONG_TIMER_ID_OUTPUT_PARAM;
    let _ = HICOLLIE_FLAG_DEFAULT;
    let _ = HICOLLIE_FLAG_NOOP;
    let _ = HICOLLIE_FLAG_LOG;
    let _ = HICOLLIE_FLAG_RECOVERY;
    let _: OH_HiCollie_Callback = None;
    let _ = HiCollie_SetTimerParam {
        name: std::ptr::null(),
        timeout: 0,
        func: None,
        arg: std::ptr::null_mut(),
        flag: HICOLLIE_FLAG_NOOP,
    };
}
