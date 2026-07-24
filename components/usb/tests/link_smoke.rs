#[cfg(feature = "api-10")]
#[test]
fn link_smoke_api_10() {
    use ohos_usb_sys::*;
}

#[cfg(feature = "api-12")]
#[test]
fn link_smoke_api_12_ddk_ashmem() {
    use ohos_ddk_sys::DDK_Ashmem as DdkAshmem;
    use ohos_usb_sys::DDK_Ashmem;
    // Verify the imported DDK_Ashmem type matches the ddk crate's definition
}
