# OpenHarmony OHAudioSuite bindings

Low-level bindings to the OHAudioSuite API on OpenHarmony, exposed by
`libohaudiosuite.so`. The crate provides raw FFI only; it does not manage audio
node ownership, callback lifetimes, or pipeline state.

Available since API-level 22. API-10 through API-21 have no bindings.

C API reference: [`OHAudioSuite`](https://docs.openharmony.cn/pages/v5.0/en/application-dev/reference/apis-audio-kit/capi-ohaudiosuite.md).

Generated headers:

- `ohaudiosuite/native_audio_suite_base.h`
- `ohaudiosuite/native_audio_suite_engine.h`

## License

Licensed under the Apache-2.0 license, matching the license of OpenHarmony.
