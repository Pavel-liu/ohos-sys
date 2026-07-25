# OpenHarmony Media Library bindings

Raw FFI bindings for the MediaLibraryKit media library NDK.

The crate is intentionally low-level: it exposes generated C ABI bindings only and does not add safe wrappers around media assets, moving photos, image sources, or callbacks.

## Feature flags

- `api-12` enables the base media library bindings.
- `api-13` through `api-23` forward the OpenHarmony API-level gates used by the generated bindings.
- `document-features` renders this feature list in docs.rs documentation.

## Linked library

All modules link against `libmedia_asset_manager.so` via `#[link(name = "media_asset_manager")]`.

## License

Licensed under the Apache-2.0 license, matching the license of OpenHarmony.
