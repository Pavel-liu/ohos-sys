#[test]
fn link_smoke_api_10() {
    let _f: unsafe extern "C" fn(
        usize,
        ohos_purgeable_memory_sys::OH_PurgeableMemory_ModifyFunc,
        *mut core::ffi::c_void,
    ) -> *mut ohos_purgeable_memory_sys::OH_PurgeableMemory =
        ohos_purgeable_memory_sys::OH_PurgeableMemory_Create;
}
