# OpenHarmony DeviceCertificateKit bindings

Low-level bindings to the DeviceCertificateKit certificate manager API on
OpenHarmony, exposed by the SDK as `libohcert_manager.z.so`.

This crate covers:

- `device_certificate/certmanager/cm_native_api.h`
- `device_certificate/certmanager/cm_native_type.h`

Available since API-level 22. API-10 through API-21 have no bindings.

The upstream header documents `libohcert_manager.so`, but the OpenHarmony native
SDK ships and links the library as `libohcert_manager.z.so`.

## License

Licensed under the Apache-2.0 license, matching the license of OpenHarmony.
