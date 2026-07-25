# OpenHarmony AVSession bindings

Raw FFI bindings for the AVSessionKit NDK.

The crate exposes generated C ABI bindings only. It intentionally does not add safe wrappers around session ownership, cast-controller ownership, or callback lifetimes.

C API reference: [AVSessionKit C API](https://docs.openharmony.cn/pages/v5.0/en/application-dev/reference/apis-avsession-kit/capi-avsession.md).

## Feature flags

- `api-13` enables the base AV session, metadata, and error bindings.
- `api-14` through `api-22` forward API-level gates for compatibility with the root crate.
- `api-23` enables newer playback-state, device-info, queue-item, and cast-controller bindings where the SDK marks them as API 23.
- `document-features` renders this feature list in docs.rs documentation.

## Linked library

The crate root links against `libohavsession.so` via `#[link(name = "ohavsession")]`.

## License

Licensed under the Apache-2.0 license, matching the license of OpenHarmony.
