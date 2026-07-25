# OpenHarmony FFRT bindings

Low-level bindings to the Function Flow Runtime (FFRT) C API exposed by
`libffrt.z.so`. FFRT provides task submission, dependency scheduling, queues,
synchronization primitives, timers, loops, sleep/yield APIs, and fibers.

Available since API-level 10. Additional APIs are gated by feature level:
- API-level 12: loop and timer APIs
- API-level 18: shared mutex / rwlock APIs
- API-level 20: fiber APIs

C API reference:

- [FFRT C API reference](https://docs.openharmony.cn/pages/v5.0/en/application-dev/reference/apis-ffrt-kit/capi-ffrt.md)

## License

Licensed under the Apache-2.0 license, matching the license of OpenHarmony.
