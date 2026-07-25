#[cfg(feature = "api-12")]
#[test]
fn link_smoke_api_12() {
    let _f: unsafe extern "C" fn() -> *mut ohos_image_effect_sys::OH_EffectFilterInfo =
        ohos_image_effect_sys::OH_EffectFilterInfo_Create;
}
