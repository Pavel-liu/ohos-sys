#[cfg(feature = "api-22")]
#[test]
fn link_smoke_api_22() {
    let _get_time_zone_rules: unsafe extern "C" fn(
        *const core::ffi::c_char,
        *mut ohos_i18n_sys::TimeZoneRules,
    ) -> ohos_i18n_sys::I18n_ErrorCode = ohos_i18n_sys::OH_i18n_GetTimeZoneRules;
    let _get_start_in_year: unsafe extern "C" fn(
        *mut ohos_i18n_sys::AnnualTimeZoneRule,
        i32,
        *mut ohos_i18n_sys::TimeZoneRuleQuery,
    ) -> ohos_i18n_sys::I18n_ErrorCode = ohos_i18n_sys::OH_i18n_GetStartInYear;
    let _get_start_time_at: unsafe extern "C" fn(
        *mut ohos_i18n_sys::TimeArrayTimeZoneRule,
        i32,
        *mut f64,
    ) -> ohos_i18n_sys::I18n_ErrorCode = ohos_i18n_sys::OH_i18n_GetStartTimeAt;
}
