#[cfg(feature = "api-10")]
#[test]
fn link_smoke_api_10() {
    let _f: unsafe extern "C" fn() -> i32 = ohos_usb_sys::OH_Usb_Init;
}

#[cfg(feature = "api-12")]
#[test]
fn link_smoke_api_12_ddk_ashmem() {
    let _f: unsafe extern "C" fn(
        *const ohos_usb_sys::UsbRequestPipe,
        *mut ohos_ddk_sys::DDK_Ashmem,
    ) -> i32 = ohos_usb_sys::OH_Usb_SendPipeRequestWithAshmem;
}
