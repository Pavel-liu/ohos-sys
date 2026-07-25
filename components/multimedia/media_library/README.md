# OpenHarmony Media Library bindings

Raw FFI bindings for the MediaLibraryKit media library NDK.

The crate is intentionally low-level: it exposes generated C ABI bindings only and does not add safe wrappers around media assets, moving photos, image sources, or callbacks.

C API reference: [MediaLibraryKit C API](https://docs.openharmony.cn/pages/v5.0/en/application-dev/reference/apis-media-library-kit/capi-medialibrary.md).

## Ownership notes

- `OH_MediaAssetManager_Create` returns a caller-owned manager; release it with `OH_MediaAssetManager_Release` when available for the selected API level.
- `OH_MediaAssetChangeRequest_Create` returns a caller-owned change request; release it with `OH_MediaAssetChangeRequest_Release`.
- Media assets and moving photos returned by callbacks or getters are opaque native objects; release owned instances with `OH_MediaAsset_Release` or `OH_MovingPhoto_Release` when the API transfers ownership.
- Image-source and picture output pointers from request callbacks are opaque external objects. The callback contract defines their lifetime; do not assume they outlive the callback unless the OpenHarmony API explicitly transfers ownership.

## Feature flags

- `api-12` enables the base media library bindings.
- `api-13` through `api-23` forward the OpenHarmony API-level gates used by the generated bindings.
- `document-features` renders this feature list in docs.rs documentation.

## Linked library

The crate root links against `libmedia_asset_manager.so` via `#[link(name = "media_asset_manager")]`.

## License

Licensed under the Apache-2.0 license, matching the license of OpenHarmony.
