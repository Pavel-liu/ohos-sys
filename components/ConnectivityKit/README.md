# OpenHarmony ConnectivityKit bindings

Low-level bindings to ConnectivityKit bluetooth and Wi-Fi switch status APIs,
exposed by `libbluetooth_ndk.so` and `libwifi_ndk.so`.

Available since API-level 13. API-level 21 adds `OH_Wifi_GetDeviceMacAddress`.
Some Wi-Fi APIs require platform permissions such as `ohos.permission.GET_WIFI_INFO`
and `ohos.permission.GET_WIFI_LOCAL_MAC`.

C API reference:

- [ConnectivityKit references](https://docs.openharmony.cn/pages/v5.0/en/application-dev/reference/apis-connectivity-kit/)

## License

Licensed under the Apache-2.0 license, matching the license of OpenHarmony.
