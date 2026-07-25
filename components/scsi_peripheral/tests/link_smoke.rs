#[cfg(feature = "api-18")]
#[test]
fn link_smoke_api_18() {
    let _f: unsafe extern "C" fn() -> i32 = ohos_scsi_peripheral_sys::OH_ScsiPeripheral_Init;
}
