#[cfg(feature = "api-12")]
#[test]
fn link_smoke_api_12() {
    let _get_string: unsafe extern "C" fn(
        *const ohos_resourcemanager_sys::NativeResourceManager,
        u32,
        *mut *mut core::ffi::c_char,
        ...
    )
        -> ohos_resourcemanager_sys::ResourceManager_ErrorCode =
        ohos_resourcemanager_sys::OH_ResourceManager_GetString;
    let _get_bool: unsafe extern "C" fn(
        *const ohos_resourcemanager_sys::NativeResourceManager,
        u32,
        *mut bool,
    )
        -> ohos_resourcemanager_sys::ResourceManager_ErrorCode =
        ohos_resourcemanager_sys::OH_ResourceManager_GetBool;
    let _get_resource_configuration: unsafe extern "C" fn(
        *const ohos_resourcemanager_sys::NativeResourceManager,
        *mut ohos_resourcemanager_sys::ResourceManager_Configuration,
    ) -> ohos_resourcemanager_sys::ResourceManager_ErrorCode =
        ohos_resourcemanager_sys::OH_ResourceManager_GetResourceConfiguration;
}
