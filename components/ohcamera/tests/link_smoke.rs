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

#[cfg(feature = "api-12")]
#[test]
fn link_smoke_api_12() {
    let _get_orientation: unsafe extern "C" fn(
        *mut ohos_ohcamera_sys::camera::Camera_Device,
        *mut u32,
    ) -> ohos_ohcamera_sys::camera::Camera_ErrorCode =
        ohos_ohcamera_sys::camera_device::OH_CameraDevice_GetCameraOrientation;
}

#[cfg(feature = "api-13")]
#[test]
fn link_smoke_api_13() {
    let _enable_mirror: unsafe extern "C" fn(
        *mut ohos_ohcamera_sys::photo_output::Camera_PhotoOutput,
        bool,
    ) -> ohos_ohcamera_sys::camera::Camera_ErrorCode =
        ohos_ohcamera_sys::photo_output::OH_PhotoOutput_EnableMirror;
}

#[cfg(feature = "api-20")]
#[test]
fn link_smoke_api_20() {
    let _get_white_balance: unsafe extern "C" fn(
        *mut ohos_ohcamera_sys::capture_session::Camera_CaptureSession,
        *mut i32,
    )
        -> ohos_ohcamera_sys::camera::Camera_ErrorCode =
        ohos_ohcamera_sys::capture_session::OH_CaptureSession_GetWhiteBalance;
}

#[cfg(feature = "api-21")]
#[test]
fn link_smoke_api_21() {
    let _set_photo_quality: unsafe extern "C" fn(
        *mut ohos_ohcamera_sys::photo_output::Camera_PhotoOutput,
        ohos_ohcamera_sys::camera::Camera_PhotoQualityPrioritization,
    )
        -> ohos_ohcamera_sys::camera::Camera_ErrorCode =
        ohos_ohcamera_sys::photo_output::OH_PhotoOutput_SetPhotoQualityPrioritization;
}

#[cfg(feature = "api-22")]
#[test]
fn link_smoke_api_22() {
    let _is_orientation_variable: unsafe extern "C" fn(
        *mut ohos_ohcamera_sys::camera_input::Camera_Input,
        *mut bool,
    ) -> ohos_ohcamera_sys::camera::Camera_ErrorCode =
        ohos_ohcamera_sys::camera_input::OH_CameraInput_IsPhysicalCameraOrientationVariable;
}

#[cfg(feature = "api-23")]
#[test]
fn link_smoke_api_23() {
    let _register_occlusion: unsafe extern "C" fn(
        *mut ohos_ohcamera_sys::camera_input::Camera_Input,
        ohos_ohcamera_sys::camera_input::OH_CameraInput_OnOcclusionDetectionCallback,
    )
        -> ohos_ohcamera_sys::camera::Camera_ErrorCode =
        ohos_ohcamera_sys::camera_input::OH_CameraInput_RegisterOcclusionDetectionCallback;
}
