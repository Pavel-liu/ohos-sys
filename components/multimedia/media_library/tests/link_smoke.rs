use ohos_media_library_sys as media_library;

fn touch_type<T>() {
    let _ = std::mem::size_of::<T>();
}

#[test]
fn link_smoke() {
    touch_type::<media_library::asset_base::OH_MediaAsset>();
    touch_type::<media_library::asset_base::OH_MediaAssetManager>();

    let _create: unsafe extern "C" fn() -> *mut media_library::asset_base::OH_MediaAssetManager =
        media_library::asset_manager::OH_MediaAssetManager_Create;
    let _request_image_for_path: unsafe extern "C" fn(
        *mut media_library::asset_base::OH_MediaAssetManager,
        *const core::ffi::c_char,
        media_library::asset_base::MediaLibrary_RequestOptions,
        *const core::ffi::c_char,
        media_library::asset_base::OH_MediaLibrary_OnDataPrepared,
    ) -> media_library::asset_base::MediaLibrary_RequestId =
        media_library::asset_manager::OH_MediaAssetManager_RequestImageForPath;
    let _get_uri: unsafe extern "C" fn(
        *mut media_library::asset_base::OH_MediaAsset,
        *mut *const core::ffi::c_char,
    ) -> media_library::asset_base::MediaLibrary_ErrorCode =
        media_library::asset::OH_MediaAsset_GetUri;
    let _apply_changes: unsafe extern "C" fn(
        *mut media_library::asset_base::OH_MediaAssetChangeRequest,
    )
        -> media_library::asset_base::MediaLibrary_ErrorCode =
        media_library::access_helper::OH_MediaAccessHelper_ApplyChanges;

    #[cfg(feature = "api-13")]
    {
        let _request_moving_photo: unsafe extern "C" fn(
            *mut media_library::asset_base::OH_MediaAssetManager,
            *mut media_library::asset_base::OH_MediaAsset,
            media_library::asset_base::MediaLibrary_RequestOptions,
            *mut media_library::asset_base::MediaLibrary_RequestId,
            media_library::asset_base::OH_MediaLibrary_OnMovingPhotoDataPrepared,
        ) -> media_library::asset_base::MediaLibrary_ErrorCode =
            media_library::asset_manager::OH_MediaAssetManager_RequestMovingPhoto;
    }

    #[cfg(feature = "api-23")]
    {
        let _quick_request_image: unsafe extern "C" fn(
            *mut media_library::asset_base::OH_MediaAssetManager,
            *mut media_library::asset_base::OH_MediaAsset,
            media_library::asset_base::MediaLibrary_RequestOptions,
            *mut media_library::asset_base::MediaLibrary_RequestId,
            media_library::asset_base::OH_MediaLibrary_OnQuickImageDataPrepared,
        ) -> media_library::asset_base::MediaLibrary_ErrorCode =
            media_library::asset_manager::OH_MediaAssetManager_QuickRequestImage;
    }
}
