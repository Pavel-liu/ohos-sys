# OpenHarmony HiCollie bindings

Low-level bindings to the HiCollie (thread stuck and jank detection) API on OpenHarmony,
exposed by `libohhicollie.so`. Provides functions for detecting thread stuck and jank events.

Available since API-level 12. Additional APIs available at later levels:
- API-level 18: `OH_HiCollie_Init_StuckDetectionWithTimeout`, `OH_HiCollie_SetTimer`,
  `OH_HiCollie_CancelTimer`, and related types (`HiCollie_Flag`, `HiCollie_SetTimerParam`)

C API reference:

- [HiCollie NDK guide](https://docs.openharmony.cn/pages/v5.0/en/application-dev/dfx/hicollie-guidelines-ndk.md)
- [`hicollie.h` reference](https://docs.openharmony.cn/pages/v5.0/en/application-dev/reference/apis-performance-analysis-kit/capi-hicollie-h.md)

## License

Licensed under the Apache-2.0 license, matching the license of OpenHarmony.
