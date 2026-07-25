# OpenHarmony TEEKit tee_client bindings

Low-level bindings to the TEEKit tee_client API on OpenHarmony, exposed by
`libteec.so`. This crate covers only:

- `TEEKit/tee_client/tee_client_api.h`
- `TEEKit/tee_client/tee_client_constants.h`
- `TEEKit/tee_client/tee_client_type.h`

`TEEKit/tee/*` headers are intentionally excluded because those APIs are not
part of this tee_client binding surface.

Available since API-level 20. API-10 through API-19 have no bindings.

## License

Licensed under the Apache-2.0 license, matching the license of OpenHarmony.
