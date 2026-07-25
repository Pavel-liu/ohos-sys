# OpenHarmony MindSpore Lite bindings

Low-level bindings to the MindSpore Lite NDK API on OpenHarmony, exposed by
`libmindspore_lite_ndk.so`. This crate covers the context, model, tensor,
types, status, data type, and format headers.

The SDK headers document some APIs as available since API-level 9. In this
workspace, API-level features start at `api-10`, so this crate exposes
MindSpore bindings from `api-10` onward.

C API references:

- `mindspore/context.h`
- `mindspore/model.h`
- `mindspore/tensor.h`
- `mindspore/types.h`
- `mindspore/status.h`
- `mindspore/data_type.h`
- `mindspore/format.h`

## License

Licensed under the Apache-2.0 license, matching the license of OpenHarmony.
