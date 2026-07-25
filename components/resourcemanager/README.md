# OpenHarmony ResourceManager bindings

Low-level bindings to the ResourceManager API on OpenHarmony, exposed by
`libohresmgr.so`. Provides APIs for querying application resources such as
strings, booleans, media, colors, and configuration.

Available since API-level 12. API-10 and API-11 have no bindings.

This crate intentionally generates only `resourcemanager/ohresmgr.h` and
`resourcemanager/resmgr_common.h`. Rawfile APIs with the same
`OH_ResourceManager_*RawFile*` prefix remain in `ohos-rawfile-sys`.

C API reference:

- [`ohresmgr.h` reference](https://docs.openharmony.cn/pages/v5.0/en/application-dev/reference/apis-localization-kit/capi-ohresmgr-h.md)
- [`resmgr_common.h` reference](https://docs.openharmony.cn/pages/v5.0/en/application-dev/reference/apis-localization-kit/capi-resmgr-common-h.md)

## License

Licensed under the Apache-2.0 license, matching the license of OpenHarmony.
