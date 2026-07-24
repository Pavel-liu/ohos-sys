#[cfg(feature = "api-13")]
#[test]
fn link_smoke_api_13() {
    use ohos_notificationkit_sys::*;
    let _: OH_Notification = std::ptr::null_mut();
    let _: OH_NotificationSlot = std::ptr::null_mut();
}
