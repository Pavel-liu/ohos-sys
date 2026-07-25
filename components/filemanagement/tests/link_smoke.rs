#[cfg(feature = "api-12")]
#[test]
fn link_smoke_environment_api_12() {
    let _f: unsafe extern "C" fn(
        *mut *mut core::ffi::c_char,
    ) -> ohos_filemanagement_sys::FileManagementResult =
        ohos_filemanagement_sys::OH_Environment_GetUserDownloadDir;
}

#[cfg(feature = "api-12")]
#[test]
fn link_smoke_fileio_api_12() {
    let _f: unsafe extern "C" fn(
        *mut core::ffi::c_char,
        core::ffi::c_int,
        *mut ohos_filemanagement_sys::FileIO_FileLocation,
    ) -> ohos_filemanagement_sys::FileManagementResult =
        ohos_filemanagement_sys::OH_FileIO_GetFileLocation;
}

#[cfg(feature = "api-12")]
#[test]
fn link_smoke_fileshare_api_12() {
    let _f: unsafe extern "C" fn(
        *const ohos_filemanagement_sys::FileShare_PolicyInfo,
        core::ffi::c_uint,
        *mut *mut ohos_filemanagement_sys::FileShare_PolicyErrorResult,
        *mut core::ffi::c_uint,
    ) -> ohos_filemanagement_sys::FileManagementResult =
        ohos_filemanagement_sys::OH_FileShare_PersistPermission;
}

#[cfg(feature = "api-12")]
#[test]
fn link_smoke_file_uri_api_12() {
    let _f: unsafe extern "C" fn(
        *const core::ffi::c_char,
        core::ffi::c_uint,
        *mut *mut core::ffi::c_char,
    ) -> ohos_filemanagement_sys::FileManagementResult =
        ohos_filemanagement_sys::OH_FileUri_GetUriFromPath;
}

#[cfg(feature = "api-21")]
#[test]
fn link_smoke_clouddisk_api_21() {
    let _f: unsafe extern "C" fn(
        ohos_filemanagement_sys::CloudDisk_SyncFolderPath,
        u64,
        usize,
        *mut *mut ohos_filemanagement_sys::CloudDisk_ChangesResult,
    ) -> ohos_filemanagement_sys::CloudDiskResult =
        ohos_filemanagement_sys::OH_CloudDisk_GetSyncFolderChanges;
}
