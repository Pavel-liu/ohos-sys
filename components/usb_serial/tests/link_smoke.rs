#[cfg(feature = "api-18")]
#[test]
fn link_smoke_api_18() {
    let _f: unsafe extern "C" fn() -> i32 = ohos_usb_serial_sys::OH_UsbSerial_Init;
}
