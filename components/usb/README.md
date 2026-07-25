# OpenHarmony USB DDK bindings

Low-level bindings to the USB DDK APIs exposed by `libusb_ndk.z.so`.
The API provides USB host/DDK operations for opening interfaces, control
transfers, pipe requests, and USB device memory mapping.

Available since API-level 10. API-level 12 adds ashmem-based pipe requests
through the shared `DDK_Ashmem` type from `ohos-ddk-sys`.

Most USB DDK functions require `ohos.permission.ACCESS_DDK_USB`.

C API reference:

- [`usb_ddk_api.h` reference](https://docs.openharmony.cn/pages/v5.0/en/application-dev/reference/apis-driverdevelopment-kit/capi-usb-ddk-api-h.md)

## License

Licensed under the Apache-2.0 license, matching the license of OpenHarmony.
