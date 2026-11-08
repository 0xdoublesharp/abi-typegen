# abi-typegen-runtime

<p align="center">
  <img src="https://raw.githubusercontent.com/doublesharp/abi-typegen/main/docs/abi-typegen.png" alt="abi-typegen" width="360" />
</p>

The shared Rust/Alloy ABI codec behind
[abi-typegen](https://github.com/doublesharp/abi-typegen#readme)'s generated C and
C++ bindings and native bridges. It exposes a C ABI for encoding calls and
constructor data, decoding results, events, and custom errors, and managing ABI
values and result memory.

## Build and link

Requires Rust 1.99 or newer to build. From an abi-typegen source checkout:

```sh
cargo build --release -p abi-typegen-runtime
```

The crate builds Rust, static, and shared libraries (`rlib`, `staticlib`, and
`cdylib`). Link `abi_typegen_runtime` from Cargo's active target directory into
your native application and include the matching
[`abi_typegen.h`](https://github.com/doublesharp/abi-typegen/blob/main/crates/abi-typegen-runtime/include/abi_typegen.h).
Rust projects can add the crate with `cargo add abi-typegen-runtime`.

Install the [generator](https://github.com/doublesharp/abi-typegen#install) to create
contract-specific headers:

```sh
abi-typegen generate --target cpp
```

The `c` target emits `atg_<Name>.h`; `cpp` also emits `atg_<Name>.hpp`. The generator
and runtime must use matching versions and runtime headers. Native CLI archives
and the npm CLI package ship the generator; build and link this runtime separately.

## Use the runtime

Your application supplies networking, signing, and transaction settings through
the generated transport callbacks. ABI encoding and decoding work offline.

- Check `atg_result_error` before reading a result, then release owned results with
  `atg_result_free`.
- Keep results alive while using borrowed strings, arrays, or decoded fields.
  Never free a borrowed `atg_value` child.
- C++ helpers manage result ownership through `Decoded<T>` and return encoded
  data as `std::vector<uint8_t>`.

See the [native binding guide](https://github.com/doublesharp/abi-typegen/blob/main/docs/native-bindings.md#c-and-c)
for build details and ownership rules, the [API reference](https://docs.rs/abi-typegen-runtime)
for exported functions, and the [main README](https://github.com/doublesharp/abi-typegen#readme)
for other targets.
