#[cfg(feature = "api-11")]
#[test]
fn link_smoke_api_11() {
    let _get_manager: unsafe extern "C" fn(
        *mut *mut ohos_ohcamera_sys::camera::Camera_Manager,
    ) -> ohos_ohcamera_sys::camera::Camera_ErrorCode =
        ohos_ohcamera_sys::camera::OH_Camera_GetCameraManager;
    let _delete_manager: unsafe extern "C" fn(
        *mut ohos_ohcamera_sys::camera::Camera_Manager,
    ) -> ohos_ohcamera_sys::camera::Camera_ErrorCode =
        ohos_ohcamera_sys::camera::OH_Camera_DeleteCameraManager;
    let _get_supported: unsafe extern "C" fn(
        *mut ohos_ohcamera_sys::camera::Camera_Manager,
        *mut *mut ohos_ohcamera_sys::camera::Camera_Device,
        *mut u32,
    ) -> ohos_ohcamera_sys::camera::Camera_ErrorCode =
        ohos_ohcamera_sys::camera_manager::OH_CameraManager_GetSupportedCameras;
    let _open_input: unsafe extern "C" fn(
        *mut ohos_ohcamera_sys::camera_input::Camera_Input,
    ) -> ohos_ohcamera_sys::camera::Camera_ErrorCode =
        ohos_ohcamera_sys::camera_input::OH_CameraInput_Open;
}
