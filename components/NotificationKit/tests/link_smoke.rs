#[cfg(feature = "api-13")]
#[test]
fn link_smoke_api_13() {
    let _f: unsafe extern "C" fn() -> bool =
        ohos_notificationkit_sys::OH_Notification_IsNotificationEnabled;
}
