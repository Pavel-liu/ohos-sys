use ohos_av_session_sys as av_session;

fn touch_type<T>() {
    let _ = std::mem::size_of::<T>();
}

#[test]
fn link_smoke() {
    touch_type::<av_session::avsession_base::AVSession_Type>();

    let _create: unsafe extern "C" fn(
        av_session::avsession_base::AVSession_Type,
        *const core::ffi::c_char,
        *const core::ffi::c_char,
        *const core::ffi::c_char,
        *mut *mut av_session::avsession::OH_AVSession,
    ) -> av_session::avsession_errors::AVSession_ErrCode =
        av_session::avsession::OH_AVSession_Create;
    let _destroy: unsafe extern "C" fn(
        *mut av_session::avsession::OH_AVSession,
    ) -> av_session::avsession_errors::AVSession_ErrCode =
        av_session::avsession::OH_AVSession_Destroy;
    let _metadata_builder_create: unsafe extern "C" fn(
        *mut *mut av_session::avmetadata::OH_AVMetadataBuilder,
    ) -> av_session::avsession_errors::AVMetadata_Result =
        av_session::avmetadata::OH_AVMetadataBuilder_Create;

    #[cfg(feature = "api-23")]
    {
        let _cast_destroy: unsafe extern "C" fn(
            *mut av_session::avsession::OH_AVCastController,
        )
            -> av_session::avsession_errors::AVSession_ErrCode =
            av_session::avcastcontroller::OH_AVCastController_Destroy;
        let _device_name: unsafe extern "C" fn(
            *mut av_session::deviceinfo::AVSession_DeviceInfo,
            *mut *mut core::ffi::c_char,
        )
            -> av_session::avsession_errors::AVSession_ErrCode =
            av_session::deviceinfo::OH_DeviceInfo_GetDeviceName;
        let _playback_state: unsafe extern "C" fn(
            *mut av_session::avplaybackstate::OH_AVSession_AVPlaybackState,
            *mut av_session::avsession_base::AVSession_PlaybackState,
        ) -> av_session::avsession_errors::AVSession_ErrCode =
            av_session::avplaybackstate::OH_AVSession_GetPlaybackState;
        let _queue_builder_create: unsafe extern "C" fn(
            *mut *mut av_session::avqueueitem::OH_AVSession_AVMediaDescriptionBuilder,
        ) -> av_session::avsession_errors::AVQueueItem_Result =
            av_session::avqueueitem::OH_AVSession_AVMediaDescriptionBuilder_Create;
    }
}
