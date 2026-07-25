#[cfg(feature = "api-10")]
#[test]
fn link_smoke_api_10() {
    let _f: unsafe extern "C" fn(u64) -> core::ffi::c_int = ohos_ffrt_sys::ffrt_usleep;
}

#[cfg(feature = "api-12")]
#[test]
fn link_smoke_api_12() {
    let _f: unsafe extern "C" fn(
        ohos_ffrt_sys::ffrt_qos_t,
        u64,
        *mut core::ffi::c_void,
        ohos_ffrt_sys::ffrt_timer_cb,
        bool,
    ) -> ohos_ffrt_sys::ffrt_timer_t = ohos_ffrt_sys::ffrt_timer_start;
}

#[cfg(feature = "api-18")]
#[test]
fn link_smoke_api_18() {
    let _f: unsafe extern "C" fn(
        *mut ohos_ffrt_sys::ffrt_rwlock_t,
        *const ohos_ffrt_sys::ffrt_rwlockattr_t,
    ) -> core::ffi::c_int = ohos_ffrt_sys::ffrt_rwlock_init;
}

#[cfg(feature = "api-20")]
#[test]
fn link_smoke_api_20() {
    let _f: unsafe extern "C" fn(
        *mut ohos_ffrt_sys::ffrt_fiber_t,
        Option<unsafe extern "C" fn(*mut core::ffi::c_void)>,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        usize,
    ) -> core::ffi::c_int = ohos_ffrt_sys::ffrt_fiber_init;
}
