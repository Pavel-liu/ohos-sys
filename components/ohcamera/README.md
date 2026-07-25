# OpenHarmony CameraKit ohcamera bindings

Low-level bindings to CameraKit's `ohcamera` C API on OpenHarmony, exposed by
`libohcamera.so`. The crate exposes raw FFI only; callbacks, camera object
ownership, and output-session lifetimes remain the caller's responsibility.

Available since API-level 11. API-10 has no bindings.

C API reference: [CameraKit `ohcamera` C API](https://docs.openharmony.cn/pages/v5.0/en/application-dev/reference/apis-camera-kit/capi-ohcamera.md).

Generated headers: `ohcamera/*.h`.

## Opaque external types

The bindings reuse common opaque declarations from `ohos-sys-opaque-types` for
cross-component handles such as `OH_ImageNative`, `OH_PictureNative`, and
`OH_MediaAsset`. Native-buffer color-space values are represented locally with
the same transparent integer layout as the SDK enum to avoid depending on the
window crate.

## License

Licensed under the Apache-2.0 license, matching the license of OpenHarmony.
