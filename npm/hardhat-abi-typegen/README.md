# @0xdoublesharp/hardhat-abi-typegen

<p align="center">
  <img src="https://raw.githubusercontent.com/0xdoublesharp/abi-typegen/main/docs/abi-typegen.png" alt="abi-typegen" width="360" />
</p>

Generate typed contract bindings when Hardhat compiles your Solidity contracts.
This plugin connects Hardhat 2 or 3 to
[abi-typegen](https://github.com/0xdoublesharp/abi-typegen#readme) and installs the
prebuilt native CLI through its npm dependency.

## Install

Choose your package manager:

```sh
npm install -D @0xdoublesharp/hardhat-abi-typegen
yarn add -D @0xdoublesharp/hardhat-abi-typegen
pnpm add -D @0xdoublesharp/hardhat-abi-typegen
```

The CLI installer needs `curl` and `tar`; Linux archives target glibc. Generating
bindings requires no Rust toolchain or RPC connection. Add the SDK for your chosen
target to the app using the generated code. See the
[installation guide](https://github.com/0xdoublesharp/abi-typegen/blob/main/docs/installation.md)
for platforms and download verification.

## Quick start

### Hardhat 3

Add the plugin to `plugins` in `hardhat.config.ts`:

```ts
import { defineConfig } from "hardhat/config";
import abiTypegen from "@0xdoublesharp/hardhat-abi-typegen";

export default defineConfig({
  plugins: [abiTypegen],
  solidity: "0.8.34",
  typegen: {
    out: "src/generated",
    target: "viem",
    contracts: ["Token"],
    exclude: ["*Test", "*Mock"],
  },
});
```

### Hardhat 2

Use the explicit legacy adapter in `hardhat.config.ts`:

```ts
import type { HardhatUserConfig } from "hardhat/config";
import "@0xdoublesharp/hardhat-abi-typegen/hardhat2";

const config: HardhatUserConfig = {
  solidity: "0.8.34",
  typegen: {
    out: "src/generated",
    target: "viem",
  },
};

export default config;
```

Compile to generate bindings:

```sh
npx hardhat compile
```

You can also run the CLI directly with
`npx abi-typegen generate --hardhat --target viem`.

## Configuration

| Option      | Type                 | Default           | What it does                                |
| ----------- | -------------------- | ----------------- | ------------------------------------------- |
| `out`       | `string`             | `"src/generated"` | Choose the output directory                 |
| `target`    | `string \| string[]` | `"viem"`          | Select one or more targets                  |
| `wrappers`  | `boolean`            | `true`            | Emit callable wrappers where supported      |
| `contracts` | `string[]`           | `[]`              | Select contracts by name; empty selects all |
| `exclude`   | `string[]`           | `[]`              | Exclude contracts matching glob patterns    |

For multiple targets, use `target: ["viem", "python", "rust"]` or
`target: "viem,python,rust"`. Each target gets a subdirectory under `out`.
Disabling `wrappers` keeps primary ABI modules, types, and schemas.

See the [main README](https://github.com/0xdoublesharp/abi-typegen#readme) for the
[target list](https://github.com/0xdoublesharp/abi-typegen#choose-your-target) and
examples of the generated code. The
[configuration guide](https://github.com/0xdoublesharp/abi-typegen/blob/main/docs/configuration.md)
and [native binding guide](https://github.com/0xdoublesharp/abi-typegen/blob/main/docs/native-bindings.md)
cover target settings and runtime dependencies. Regenerate bindings after
upgrading and check the
[changelog](https://github.com/0xdoublesharp/abi-typegen/blob/main/CHANGELOG.md) for
API or naming changes.
