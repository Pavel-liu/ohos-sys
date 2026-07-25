# OpenHarmony MindSpore Lite bindings

Low-level bindings to the MindSpore Lite NDK API on OpenHarmony, exposed by
`libmindspore_lite_ndk.so`. This crate covers the context, model, tensor,
types, status, data type, and format headers.

The SDK headers document some APIs as available since API-level 9. In this
workspace, API-level features start at `api-10`, so this crate exposes
MindSpore bindings from `api-10` onward.

C API references:

- [`mindspore/context.h`](https://docs.openharmony.cn/pages/v5.0/en/application-dev/reference/apis-mindspore-lite-kit/_mind_spore.md)
- [`mindspore/model.h`](https://docs.openharmony.cn/pages/v5.0/en/application-dev/reference/apis-mindspore-lite-kit/_mind_spore.md)
- [`mindspore/tensor.h`](https://docs.openharmony.cn/pages/v5.0/en/application-dev/reference/apis-mindspore-lite-kit/_mind_spore.md)
- [`mindspore/types.h`](https://docs.openharmony.cn/pages/v5.0/en/application-dev/reference/apis-mindspore-lite-kit/_mind_spore.md)
- [`mindspore/status.h`](https://docs.openharmony.cn/pages/v5.0/en/application-dev/reference/apis-mindspore-lite-kit/_mind_spore.md)
- [`mindspore/data_type.h`](https://docs.openharmony.cn/pages/v5.0/en/application-dev/reference/apis-mindspore-lite-kit/_mind_spore.md)
- [`mindspore/format.h`](https://docs.openharmony.cn/pages/v5.0/en/application-dev/reference/apis-mindspore-lite-kit/_mind_spore.md)

## Ownership notes

- Handles created by `OH_AI_ModelCreate`, `OH_AI_ContextCreate`, `OH_AI_TensorCreate`, and `OH_AI_TrainCfgCreate` are owned by the caller.
- Release owned handles with the matching `OH_AI_ModelDestroy`, `OH_AI_ContextDestroy`, `OH_AI_TensorDestroy`, and `OH_AI_TrainCfgDestroy` functions.
- Model/tensor/context/train-cfg pointers returned by getters remain borrowed unless the OpenHarmony API documents a matching destroy function for that returned handle.

## License

Licensed under the Apache-2.0 license, matching the license of OpenHarmony.
