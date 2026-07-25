#![cfg(feature = "api-13")]

use std::ptr;

use ohos_av_session_sys as av_session;

fn touch_type<T>() {
    let _ = std::mem::size_of::<T>();
}

#[test]
fn link_smoke() {
    touch_type::<av_session::avsession_base::AVSession_Type>();

    unsafe {
        let _ = av_session::avsession::OH_AVSession_Create(
            av_session::avsession_base::AVSession_Type::SESSION_TYPE_AUDIO,
            ptr::null(),
            ptr::null(),
            ptr::null(),
            ptr::null_mut(),
        );
        let _ = av_session::avsession::OH_AVSession_Destroy(ptr::null_mut());
        let _ = av_session::avmetadata::OH_AVMetadataBuilder_Create(ptr::null_mut());
    }

    #[cfg(feature = "api-23")]
    unsafe {
        let _ = av_session::avcastcontroller::OH_AVCastController_Destroy(ptr::null_mut());
    }
}
