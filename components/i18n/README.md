# OpenHarmony i18n timezone bindings

Low-level bindings to the i18n timezone API on OpenHarmony, exposed by
`libohi18n.so`. This crate covers:

- [`i18n/timezone.h`](https://docs.openharmony.cn/pages/v5.0/en/application-dev/reference/apis-localization-kit/capi-timezone-h.md)
- [`i18n/errorcode.h`](https://docs.openharmony.cn/pages/v5.0/en/application-dev/reference/apis-localization-kit/capi-errorcode-h.md)

Function names follow the SDK's lower-case `OH_i18n_` prefix.

Available since API-level 22. API-10 through API-21 have no bindings.

## Ownership notes

The timezone rule APIs expose raw output buffers. Arrays returned through `TimeZoneRules`, `TimeArrayTimeZoneRule`, and `AnnualTimeZoneRule` output parameters are owned by the caller after a successful call and must be released according to the matching OpenHarmony i18n C API contract.

## License

Licensed under the Apache-2.0 license, matching the license of OpenHarmony.
