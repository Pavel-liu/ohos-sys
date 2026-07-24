#![cfg(feature = "api-13")]

use ohos_telephony_sys as telephony;

#[test]
fn link_smoke() {
    let mut state: telephony::Telephony_NetworkState = unsafe { core::mem::zeroed() };
    let _ = unsafe { telephony::OH_Telephony_GetNetworkState(&mut state) };
}
