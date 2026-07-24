#[cfg(feature = "api-21")]
#[test]
fn link_smoke_api_21() {
    use ohos_gamecontrollerkit_sys::*;
    let _: OH_GameController = std::ptr::null_mut();
    let _: OH_GameControllerKeyEvent = std::ptr::null_mut();
}
