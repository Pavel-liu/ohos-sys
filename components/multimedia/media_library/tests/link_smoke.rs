#![cfg(feature = "api-12")]

use std::ptr;

use ohos_media_library_sys as media_library;

fn touch_type<T>() {
    let _ = std::mem::size_of::<T>();
}

#[test]
fn link_smoke() {
    touch_type::<media_library::asset_base::OH_MediaAsset>();
    touch_type::<media_library::asset_base::OH_MediaAssetManager>();

    unsafe {
        let _ = media_library::asset_manager::OH_MediaAssetManager_Create();
        let _ = media_library::asset_manager::OH_MediaAssetManager_RequestImageForPath(
            ptr::null_mut(),
            ptr::null(),
            std::mem::zeroed(),
            ptr::null(),
            None,
        );
        let _ = media_library::asset::OH_MediaAsset_GetUri(ptr::null_mut(), ptr::null_mut());
        let _ = media_library::access_helper::OH_MediaAccessHelper_ApplyChanges(ptr::null_mut());
    }
}
