# OpenHarmony HiDebug bindings

Low-level bindings to the HiDebug (debugging and profiling) API on OpenHarmony,
exposed by `libohhidebug.so`. Provides functions for collecting CPU usage,
memory information, and performing application trace capture.

Available since API-level 12. Additional APIs available at later levels:
- API-level 14: `OH_HiDebug_GetGraphicsMemory`
- API-level 20: MallocDispatch table, backtrace/symbolic address, `OH_HiDebug_GetAppNativeMemInfoWithCache`
- API-level 21: `OH_HiDebug_GetGraphicsMemorySummary`
- API-level 22: `OH_HiDebug_RequestThreadLiteSampling`
- API-level 23: `OH_HiDebug_SetCrashObj` / `OH_HiDebug_ResetCrashObj`

C API reference:

- [HiDebug NDK guide](https://docs.openharmony.cn/pages/v5.0/en/application-dev/dfx/hidebug-guidelines-ndk.md)
- [`hidebug.h` reference](https://docs.openharmony.cn/pages/v5.0/en/application-dev/reference/apis-performance-analysis-kit/capi-hidebug-h.md)

## License

Licensed under the Apache-2.0 license, matching the license of OpenHarmony.
