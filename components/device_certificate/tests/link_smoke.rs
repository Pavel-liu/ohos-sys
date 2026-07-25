#[cfg(feature = "api-22")]
#[test]
fn link_smoke_api_22() {
    let _get_ukey_certificate: unsafe extern "C" fn(
        *const ohos_device_certificate_sys::OH_CM_Blob,
        *const ohos_device_certificate_sys::OH_CM_UkeyInfo,
        *mut ohos_device_certificate_sys::OH_CM_CredentialDetailList,
    ) -> i32 = ohos_device_certificate_sys::OH_CertManager_GetUkeyCertificate;
    let _get_private_certificate: unsafe extern "C" fn(
        *const ohos_device_certificate_sys::OH_CM_Blob,
        *mut ohos_device_certificate_sys::OH_CM_Credential,
    ) -> i32 = ohos_device_certificate_sys::OH_CertManager_GetPrivateCertificate;
    let _get_public_certificate: unsafe extern "C" fn(
        *const ohos_device_certificate_sys::OH_CM_Blob,
        *mut ohos_device_certificate_sys::OH_CM_Credential,
    ) -> i32 = ohos_device_certificate_sys::OH_CertManager_GetPublicCertificate;
    let _free_ukey_certificate: unsafe extern "C" fn(
        *mut ohos_device_certificate_sys::OH_CM_CredentialDetailList,
    ) = ohos_device_certificate_sys::OH_CertManager_FreeUkeyCertificate;
    let _free_credential: unsafe extern "C" fn(*mut ohos_device_certificate_sys::OH_CM_Credential) =
        ohos_device_certificate_sys::OH_CertManager_FreeCredential;
}
