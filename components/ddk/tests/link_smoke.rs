#[cfg(feature = "api-12")]
#[test]
fn link_smoke_api_12() {
    let _f: unsafe extern "C" fn(
        *const u8,
        u32,
        *mut *mut ohos_ddk_sys::DDK_Ashmem,
    ) -> ohos_ddk_sys::DDK_RetCode = ohos_ddk_sys::OH_DDK_CreateAshmem;
}
