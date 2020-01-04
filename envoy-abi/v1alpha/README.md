# Envoy ABI v1alpha

💡`Envoy ABI` defined here corresponds to the `master` branch of [envoyproxy/envoy-wasm](https://github.com/envoyproxy/envoy-wasm).

## General Notes

* `Envoy ABI` was extracted out of [Envoy Wasm C++ SDK](https://github.com/envoyproxy/envoy-wasm/tree/master/api/wasm/cpp)
* Part of `ABI` that is implemented by `Envoy` itself (as a host of Wasm extensions) has been defined as "[envoy](./witx/envoy_host.witx)" Wasm module
  * in contrast, in [envoyproxy/envoy-wasm](https://github.com/envoyproxy/envoy-wasm) these functions belong to "`env`" module
* Part of `ABI` that is implemented by Wasm extensions has been defined as "[envoy_extension](./witx/envoy_host.witx)" Wasm module
  * in contrast, in [envoyproxy/envoy-wasm](https://github.com/envoyproxy/envoy-wasm) these functions do not belong to any module
* `Envoy ABI` was extracted manually; it's meant to stay up-to-date only by keeping an eye on changes to [Envoy Wasm C++ SDK](https://github.com/envoyproxy/envoy-wasm/tree/master/api/wasm/cpp)