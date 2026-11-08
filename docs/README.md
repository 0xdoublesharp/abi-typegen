# Documentation

Install abi-typegen, choose a target, and use the generated bindings in your app.
For version-specific changes, see the [changelog](../CHANGELOG.md).

## Using abi-typegen

- [Installation](installation.md): npm and native installation, supported
  platforms, checksum verification, and download troubleshooting.
- [Configuration](configuration.md): Foundry, Hardhat, CLI overrides, contract
  selection, and importing ABIs.
- [Generated output](generated-output.md): filenames, wrapper behavior, type
  mappings, and what each target provides.
- [Forge integration](forge-integration.md): build commands, CI checks, shell
  integration, and watching artifacts.
- [Choosing a binding tool](comparison.md): output tradeoffs and how to measure
  generation performance for your project.
- [Native contract bindings](native-bindings.md): runtime dependencies, C/C++
  ownership, F# and OCaml wrappers, q event tables, and engine integrations.

## Packages

Use the CLI to generate bindings, the Hardhat plugin to generate during compilation,
or the Rust crates to build your own tooling.

| Package                                                        | Purpose                                         |
| -------------------------------------------------------------- | ----------------------------------------------- |
| [npm CLI](../npm/abi-typegen/README.md)                        | Install and run the prebuilt generator          |
| [Hardhat plugin](../npm/hardhat-abi-typegen/README.md)         | Generate bindings with Hardhat 2 or 3           |
| [abi-typegen-core](../crates/abi-typegen-core/README.md)       | Parse ABI artifacts into contract types         |
| [abi-typegen-config](../crates/abi-typegen-config/README.md)   | Read configuration and choose targets           |
| [abi-typegen-codegen](../crates/abi-typegen-codegen/README.md) | Render contract types into source files         |
| [abi-typegen-runtime](../crates/abi-typegen-runtime/README.md) | Link the shared ABI codec into native consumers |

## Contributing

- [Contributing](../CONTRIBUTING.md): bug reports, code conventions, tests, and
  pull-request guidance.
- [Development](development.md): source builds, repository layout, and validation
  commands, Rust coverage interpretation, and Anvil consumer tests.
- [Build and cache storage](development-storage.md): optional local storage
  configuration, migration, and returning to defaults.
- [Releasing](releasing.md): matching package versions, binary assets, checksum
  manifests, and publication order.
