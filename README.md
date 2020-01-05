# Rust SDK for Envoy Wasm extensions

⚠️ This is NOT the official Envoy Rust SDK.

💡The goal of this repo is to verify that `Envoy ABI` and `Envoy Wasm documentation` provide sufficient information for building new SDKs from scratch.

## Components

* [envoy-abi](./envoy-abi/) - formal definition of `Envoy ABI` in a [WASI](https://github.com/WebAssembly/WASI)-compatible manner
  * [v1alpha](./envoy-abi/v1alpha/) - latest version of `Envoy ABI`
    * [witx](./envoy-abi/v1alpha/witx) - `Envoy ABI` definition in `witx` format
    * [docs](./envoy-abi/v1alpha/docs/envoy_abi_v1alpha.md) - `Envoy ABI` documentation generated out of `witx` definition
* [envoy-rust-abi](./envoy-rust-abi/) - Rust bindings to `Envoy ABI`
* [envoy-rust-examples](./envoy-rust-examples/) - Examples of how to use Rust bindings
  * [hello-world](./envoy-rust-examples/hello-world/) - Call `proxy_log` from various handlers, e.g. `proxy_on_start`, `proxy_on_configure`, `proxy_on_create`, `proxy_on_request_headers`, etc