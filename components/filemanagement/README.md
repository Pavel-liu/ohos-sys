# OpenHarmony CoreFileKit bindings

Low-level bindings to CoreFileKit file management APIs. This crate covers
multiple native libraries: `libohenvironment.so`, `libohfileio.so`,
`libohfileshare.so`, `libohfileuri.so`, and `libohclouddiskmanager.so`.

Environment, file I/O, file share, and file URI APIs are available since
API-level 12. Cloud disk manager APIs are available since API-level 21.
APIs returning allocated strings generally follow the ownership rules documented
by the corresponding C header.

C API reference:

- [CoreFileKit references](https://docs.openharmony.cn/pages/v5.0/en/application-dev/reference/apis-core-file-kit/)

## License

Licensed under the Apache-2.0 license, matching the license of OpenHarmony.
