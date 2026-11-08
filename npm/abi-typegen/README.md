# @0xdoublesharp/abi-typegen

<p align="center">
  <img src="https://raw.githubusercontent.com/doublesharp/abi-typegen/main/docs/abi-typegen.png" alt="abi-typegen" width="360" />
</p>

The prebuilt native CLI for
[abi-typegen](https://github.com/doublesharp/abi-typegen#readme). Turn compiled
Foundry or Hardhat artifacts into typed code for your app. Generate for one target
or several, including viem, ethers, Python, Rust, and C++.

## Install

Choose your package manager:

```sh
npm install -D @0xdoublesharp/abi-typegen
yarn add -D @0xdoublesharp/abi-typegen
pnpm add -D @0xdoublesharp/abi-typegen
```

The installer downloads your platform's native executable from the matching
GitHub release and verifies its SHA-256 against `checksums.json` bundled in this
npm package. Missing hashes, mismatches, and incomplete archives fail installation.
Re-running postinstall verifies a fresh archive before replacing the binary.

Installation needs `curl` and `tar`; Linux archives target glibc. Generating code
requires no Rust toolchain or RPC connection. The app using the generated code
needs its target language and SDK. See the
[installation guide](https://github.com/doublesharp/abi-typegen/blob/main/docs/installation.md)
for supported platforms and download troubleshooting.

## Quick start

Compile your contracts, then generate bindings:

```sh
# Foundry
forge build
npx abi-typegen generate --target viem

# Hardhat
npx hardhat compile
npx abi-typegen generate --hardhat --target ethers
```

The default target is viem and the default output is `src/generated`. To generate
for several apps, pass comma-separated targets; each gets its own subdirectory:

```sh
npx abi-typegen generate --target viem,python,rust
```

Save Foundry settings in `foundry.toml`:

```toml
[abi-typegen]
out = "src/generated"
target = ["viem", "python", "rust"]
```

## Keep bindings in sync

```sh
npx abi-typegen watch             # regenerate when compiled artifacts change
npx abi-typegen generate --check  # fail if generated files are stale
npx abi-typegen help generate     # show generation options
```

For automatic generation during Hardhat compilation, use
[@0xdoublesharp/hardhat-abi-typegen](https://www.npmjs.com/package/@0xdoublesharp/hardhat-abi-typegen).
Regenerate existing bindings when upgrading and check the
[changelog](https://github.com/doublesharp/abi-typegen/blob/main/CHANGELOG.md) for
API or naming changes.

See the [main README](https://github.com/doublesharp/abi-typegen#readme) for the
[target list](https://github.com/doublesharp/abi-typegen#choose-your-target), usage
examples, and command reference. The
[configuration guide](https://github.com/doublesharp/abi-typegen/blob/main/docs/configuration.md)
and [native binding guide](https://github.com/doublesharp/abi-typegen/blob/main/docs/native-bindings.md)
cover target settings and runtime dependencies.
