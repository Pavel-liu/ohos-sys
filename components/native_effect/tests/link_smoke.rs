#[cfg(feature = "api-12")]
#[test]
fn link_smoke_api_12() {
    let _f: unsafe extern "C" fn(
        *mut ohos_native_effect_sys::OH_PixelmapNative,
        *mut *mut ohos_native_effect_sys::OH_Filter,
    ) -> ohos_native_effect_sys::EffectErrorCode = ohos_native_effect_sys::OH_Filter_CreateEffect;
}
