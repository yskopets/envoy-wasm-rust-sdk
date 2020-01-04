# Rust SDK for Envoy Wasm extensions

⚠️ This is NOT the official Envoy Rust SDK.

💡The goal of this repo is to verify that `Envoy ABI` and `Envoy Wasm documentation` provide sufficient information for building new SDKs from scratch.

## Components

* [envoy-abi](./envoy-abi/README.md) - formal definition of `Envoy ABI` in a [WASI](https://github.com/WebAssembly/WASI)-compatible manner
  * [v1alpha](./envoy-abi/v1alpha/) - latest version of `Envoy ABI`
    * [witx](./envoy-abi/v1alpha/witx) - `Envoy ABI` definition in `witx` format
    * [docs](./envoy-abi/v1alpha/docs/envoy_abi_v1alpha.md) - `Envoy ABI` documentation generated out of `witx` definition
