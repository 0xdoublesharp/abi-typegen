# abi-typegen

<p align="center">
  <img src="docs/abi-typegen.png" alt="abi-typegen" width="360" />
</p>

<p align="center"><strong>One ABI. Your language.</strong></p>
<p align="center">Compile your contracts. Generate typed code for your app.</p>
<p align="center">
  <a href="https://github.com/0xdoublesharp/abi-typegen/actions/workflows/ci.yml"><img alt="CI" src="https://img.shields.io/github/actions/workflow/status/0xdoublesharp/abi-typegen/ci.yml?branch=main&label=ci"></a>
  <a href="https://github.com/0xdoublesharp/abi-typegen/actions/workflows/coverage.yml"><img alt="Coverage workflow" src="https://img.shields.io/github/actions/workflow/status/0xdoublesharp/abi-typegen/coverage.yml?branch=main&label=coverage"></a>
  <a href="https://0xdoublesharp.github.io/abi-typegen/coverage/"><img alt="Coverage report" src="https://img.shields.io/badge/coverage-report-2ea043"></a>
  <a href="https://crates.io/crates/abi-typegen"><img alt="crates.io" src="https://img.shields.io/crates/v/abi-typegen"></a>
  <a href="https://www.npmjs.com/package/@0xdoublesharp/abi-typegen"><img alt="npm" src="https://img.shields.io/npm/v/@0xdoublesharp/abi-typegen?label=npm"></a>
</p>

Generate typed contract bindings from Solidity ABI artifacts. `abi-typegen`
reads Foundry and Hardhat output and generates named arguments, return types,
ABI codecs, and helpers for your Ethereum SDK.

One native binary supports 28 targets across 21 languages and can generate several
targets at once. Your app keeps its own SDK, RPC connection, and signing flow.

<p align="center">
  <a href="#quick-start">Quick start</a> ·
  <a href="#choose-your-target">Targets</a> ·
  <a href="#use-the-generated-code">Examples</a> ·
  <a href="#keep-bindings-in-sync">CI</a> ·
  <a href="docs/README.md">Docs</a>
</p>

## Install

Install the generator once. Add your chosen Ethereum SDK to the app that uses its
output.

```sh
cargo install abi-typegen
```

You can also download a [prebuilt binary](https://github.com/0xdoublesharp/abi-typegen/releases)
or add the CLI to a JavaScript project:

```sh
pnpm add -D @0xdoublesharp/abi-typegen
pnpm exec abi-typegen --help
```

The npm package installs the native executable. Generating code does not require
an RPC connection, and the standalone binary needs neither Node nor Python.

See [installation](docs/installation.md) for platforms and download verification.

## Quick start

Compile your contracts, then point the generator at the files your compiler
created. The default target is viem and the default output is `src/generated`.

### Foundry

Use your existing Foundry project:

```sh
forge build
abi-typegen generate
```

Add your preferences to `foundry.toml`:

```toml
[abi-typegen]
out = "src/generated"
target = "viem"
```

Keep a watcher running while you work:

```sh
abi-typegen watch
```

It regenerates when artifacts change, so run `forge build` after editing Solidity.
For a `forge typegen` shell command, see [Forge integration](docs/forge-integration.md).

### Hardhat

The plugin generates bindings when Hardhat compiles your contracts.

```sh
pnpm add -D @0xdoublesharp/hardhat-abi-typegen
```

For Hardhat 3, add it to `hardhat.config.ts`:

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
    exclude: ["*Test"],
  },
});
```

```sh
pnpm exec hardhat compile
```

Hardhat 2 uses a side-effect import instead of `plugins`:

```ts
import "@0xdoublesharp/hardhat-abi-typegen/hardhat2";
```

You can also use the CLI directly with `abi-typegen generate --hardhat`.

## Choose your target

Pick the library your app already uses. A target determines the generated API and
its runtime dependency; it does not install that dependency for you.

```sh
abi-typegen generate --target go
```

There are 28 CLI targets across 21 languages. Unity uses the `csharp` target
with an adapter package.

### JavaScript and TypeScript

| Library    | Target    | Runtime or SDK                           | What you get                                     |
| ---------- | --------- | ---------------------------------------- | ------------------------------------------------ |
| Viem       | `viem`    | [viem](https://viem.sh/)                 | Typed contract helpers and ABI                   |
| Wagmi      | `wagmi`   | [wagmi](https://wagmi.sh/)               | React hooks for reads, writes, and events        |
| Ethers v6  | `ethers`  | [ethers v6](https://docs.ethers.org/v6/) | Typed contract interfaces and connection helpers |
| Ethers v5  | `ethers5` | [ethers v5](https://docs.ethers.org/v5/) | Typed contract interfaces and connection helpers |
| Web3.js v4 | `web3js`  | [web3.js v4](https://docs.web3js.org/)   | Typed contract methods                           |
| Zod 4      | `zod`     | [Zod 4](https://zod.dev/)                | Validation schemas and ABI                       |

### Native languages

| Language | Target   | Runtime or SDK                                                                                         | What you get                                                      |
| -------- | -------- | ------------------------------------------------------------------------------------------------------ | ----------------------------------------------------------------- |
| C        | `c`      | [Shared Rust runtime](docs/native-bindings.md#c-and-c)                                                 | C11 codecs and client helpers with explicit ownership             |
| C++      | `cpp`    | [Shared Rust runtime](docs/native-bindings.md#c-and-c)                                                 | C++17 codecs and client helpers with automatic cleanup            |
| C#       | `csharp` | [Nethereum](https://docs.nethereum.com/)                                                               | Typed DTOs, contract methods, and deployment                      |
| COBOL    | `cobol`  | [GnuCOBOL](https://gnucobol.sourceforge.io/doc/gnucobol.html)                                          | Reads taking one address and returning one uint256 🤷             |
| Dart     | `dart`   | [web3dart](https://pub.dev/packages/web3dart)                                                          | Typed values, codecs, calls, and transactions                     |
| Elixir   | `elixir` | [Ethers](https://ethers.hexdocs.pm/Ethers.html)                                                        | Transaction data, reads, sends, events, and errors                |
| F#       | `fsharp` | [Nethereum](https://docs.nethereum.com/)                                                               | Records, ABI codecs, and contract helpers                         |
| Go       | `go`     | [go-ethereum](https://geth.ethereum.org/docs/developers)                                               | Typed calls, transactions, deployment, and events                 |
| Java     | `java`   | [web3j](https://docs.web3j.io/latest/)                                                                 | Typed values, codecs, calls, and transactions                     |
| Kotlin   | `kotlin` | [web3j](https://docs.web3j.io/latest/)                                                                 | Typed values, codecs, calls, and transactions                     |
| OCaml    | `ocaml`  | [Zarith](https://github.com/ocaml/Zarith) and shared Rust runtime                                      | Records, ABI codecs, and caller-supplied transport                |
| PHP      | `php`    | [Brick Math](https://github.com/brick/math), [ethereum-tx](https://github.com/web3p/ethereum-tx), cURL | Codecs, RPC reads, signed legacy transactions, receipts, and logs |
| Python   | `python` | [web3.py](https://web3py.readthedocs.io/en/stable/)                                                    | Reads, transaction builders, codecs, events, and errors           |
| q        | `q`      | [kdb+](https://code.kx.com/q/) and shared Rust runtime                                                 | Event table schemas, decoders, and log rows                       |
| Ruby     | `ruby`   | [eth](https://github.com/q9f/eth.rb)                                                                   | Calls, transaction builders, codecs, events, and errors           |
| Rust     | `rust`   | [Alloy](https://alloy.rs/introduction/getting-started/)                                                | `sol!` types, codecs, and contract instances                      |
| Swift    | `swift`  | [web3swift](https://github.com/web3swift-team/web3swift)                                               | Typed values, codecs, and contract operations                     |

### Game engines

| Language | Target   | Runtime or SDK                                                                                                               | What you get                                                    |
| -------- | -------- | ---------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------- |
| C#       | `csharp` | [Unity](https://docs.unity3d.com/Manual/index.html) + [Unity adapter](integrations/unity/com.doublesharp.abi-typegen.unity/) | Typed C# bindings, simulation, receipts, and event polling      |
| GDScript | `godot`  | [Godot](https://docs.godotengine.org/en/stable/)                                                                             | Typed GDScript values, async calls, receipts, and event polling |
| C++      | `unreal` | [Unreal Engine](https://dev.epicgames.com/documentation/en-us/unreal-engine/)                                                | Typed C++/Blueprint codecs, async reads, and event watchers     |

See [engine packages](integrations/README.md) for setup and
[native bindings](docs/native-bindings.md) for supported features and platforms.

### Shell and ABI formats

| Language | Target     | Runtime or SDK                                                                                                | What you get                                                        |
| -------- | ---------- | ------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------- |
| Bash     | `shell`    | [Bash](https://www.gnu.org/software/bash/manual/bash.html) and [Foundry cast](https://www.getfoundry.sh/cast) | Sourceable helpers for encoding, reads, sends, deployment, and logs |
| Solidity | `solidity` | [Solidity](https://docs.soliditylang.org/en/latest/)                                                          | Interfaces, tuple structs, events, and errors                       |
| YAML     | `yaml`     | [YAML](https://yaml.org/)                                                                                     | Readable ABI descriptions                                           |

Zod, Solidity, and YAML describe or validate contracts; they do not submit
transactions. See [native bindings](docs/native-bindings.md) for SDK versions
and [configuration](docs/configuration.md) for target aliases.

### Generate for more than one app

Use the same contract artifacts for your frontend, backend, or native app:

```sh
abi-typegen generate --target viem,python,rust
```

Each target gets its own directory:

```text
src/generated/
├── viem/
├── python/
└── rust/
```

You can save the same selection with `target = ["viem", "python", "rust"]`.

## Use the generated code

Import the generated file, connect your SDK, and call the contract through its
typed API. You still choose the RPC endpoint, wallet, and transaction settings.

### Read with viem

For a generated `Token` binding, set `RPC_URL`, `TOKEN_ADDRESS`, and
`ACCOUNT_ADDRESS` in your environment:

```ts
import { createPublicClient, getAddress, http } from "viem";
import { getTokenContract } from "./generated/Token.viem.js";

const client = createPublicClient({ transport: http(process.env.RPC_URL) });
const token = getTokenContract(getAddress(process.env.TOKEN_ADDRESS!), client);
const owner = getAddress(process.env.ACCOUNT_ADDRESS!);

const balance = await token.read.balanceOf([owner]);
console.log(balance); // bigint
```

Generated TypeScript imports use `.js` extensions for ESM. Tuple fields and
function arguments retain their ABI types.

### Read or encode with Python

The Python wrapper uses your web3.py connection. Encoding a call works offline:

```python
import os
from web3 import Web3
from generated.Token import TokenContract

w3 = Web3(Web3.HTTPProvider(os.environ["RPC_URL"]))
token = TokenContract(Web3.to_checksum_address(os.environ["TOKEN_ADDRESS"]), w3)
owner = Web3.to_checksum_address(os.environ["ACCOUNT_ADDRESS"])

balance = token.balance_of(owner)
calldata = token.encode_transfer(owner, 10**18)
```

Write helpers build transactions for your signing flow. Other targets expose
their SDK's transaction or operation types. A submitted transaction hash is not
proof of success; check the mined receipt.

For C/C++, PHP, F#, OCaml, q/kdb+, and engine examples, see the
[native binding guide](docs/native-bindings.md). C/C++, OCaml, and q consumers
build and link `abi-typegen-runtime` separately; use the same version as the
generator.

## Know what the bindings cover

Typed arguments help catch mistakes before a call reaches the chain. They cannot
prove that a transaction will succeed or recover data the chain did not include.

Generated wrappers cover contract calls, ABI values, and the SDK integrations
listed above. Convenience methods for deployment, receipt handling, event queries,
and subscriptions differ by target.

- Java and Kotlin currently cannot encode fixed-array inputs outside lengths 1–32,
  including arrays nested inside tuples. These inputs fail explicitly.
- Indexed strings, bytes, arrays, and tuples appear in logs as hashes. Their
  original values cannot be decoded from those topics.
- Anonymous events have no signature topic, so the caller must choose the decoder.
- C/C++, OCaml, and q consumers build and link the runtime separately.
- q provides event ingestion helpers. It does not generate RPC clients or contract
  transaction wrappers.

See [current boundaries](docs/native-bindings.md#current-boundaries) for the full
list. Regenerate when upgrading, and check the [changelog](CHANGELOG.md) for API
or naming changes.

## Configure the output

Save your usual settings once. Override them on the command line when you need a
different output directory or contract selection.

```toml
# foundry.toml
[abi-typegen]
out = "src/generated"
target = ["viem", "go"]
wrappers = true
contracts = ["Token", "Vault"]
exclude = ["*Test", "*Mock"]
package = "contracts"
```

An empty `contracts` list selects all contracts. `package` names the Go package or
the Kotlin/Java package. The [configuration guide](docs/configuration.md) covers
Hardhat settings, glob patterns, aliases, and precedence.

```sh
abi-typegen generate \
  --artifacts ./out \
  --out ./types \
  --target java \
  --package com.example.contracts \
  --contracts Token,Vault \
  --clean
```

`--clean` removes stale generated files. `--no-wrappers` keeps primary ABI metadata
and types while omitting callable wrappers where supported. It keeps Zod schemas
and Solidity interfaces. See [generated output](docs/generated-output.md) for
filenames and target-specific behavior.

## Start with an existing contract

You do not need the original Solidity project. Import a local ABI, or fetch one
from an explorer that has verified the contract.

```sh
# Bare ABI arrays and artifact files containing an "abi" field both work.
abi-typegen fetch --name Token --file ./Token.abi.json
```

For an Etherscan-compatible explorer, set `ETHERSCAN_API_KEY` in your environment
or a local `.env` file, then fetch and generate:

```sh
abi-typegen fetch --name WETH --network mainnet \
  0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2
```

The default output includes a saved `out/WETH.sol/WETH.json` artifact and generated
viem files. Network shortcuts include `mainnet`, `sepolia`, `base`, `optimism`, and
`arbitrum`. For another compatible explorer, supply its endpoint:

```sh
abi-typegen fetch --name Token \
  --url "$EXPLORER_API_URL" "$CONTRACT_ADDRESS"
```

## Keep bindings in sync

When a contract changes, its generated code should change with it. A CI check can
catch forgotten updates before they are merged.

For Foundry:

```yaml
- run: forge build
- run: abi-typegen generate --check
```

For Hardhat, the plugin regenerates during compilation:

```yaml
- run: pnpm exec hardhat compile
- run: git diff --exit-code src/generated/
```

Use `--check` when CI should verify files without rewriting them. Use `diff` when
you want to inspect the proposed changes:

```sh
abi-typegen diff
```

## Command reference

| Command         | Example                                                | What it does                                         |
| --------------- | ------------------------------------------------------ | ---------------------------------------------------- |
| `generate`      | `abi-typegen generate --target viem`                   | Write bindings from compiled artifacts               |
| `watch`         | `abi-typegen watch --artifacts out`                    | Regenerate when artifacts change                     |
| `diff`          | `abi-typegen diff --target viem`                       | Preview changes without writing files                |
| `json`          | `abi-typegen json --pretty`                            | Print the parsed ABI as JSON                         |
| `fetch`         | `abi-typegen fetch --name Token --file Token.abi.json` | Import an ABI and generate bindings                  |
| `init`          | `abi-typegen init`                                     | Add an `[abi-typegen]` section to `foundry.toml`     |
| `forge-install` | `abi-typegen forge-install --shell zsh`                | Print shell integration that enables `forge typegen` |
| `help`          | `abi-typegen help generate`                            | Show help for the CLI or a command                   |

Use `abi-typegen --version` to print the installed version and
`abi-typegen help <command>` for command help. Global `--config PATH` selects a
configuration file; `--hardhat` selects the Hardhat artifact layout.

The [configuration guide](docs/configuration.md) covers generation options,
contract filters, and target aliases. `watch` uses the target and output settings
from your configuration. For shell setup, see the
[Forge integration guide](docs/forge-integration.md).

## Upgrading

Regenerate bindings after upgrading and check the [changelog](CHANGELOG.md) for
API or naming changes. Compile the result against your app's SDK before shipping.
C/C++, OCaml, and q consumers must update the shared runtime with the generator.

## Docs and support

Start with the guide for the part you are working on. For a bug report, include a
small ABI that reproduces the problem, your target, and the relevant SDK version.

- [Installation](docs/installation.md) and [configuration](docs/configuration.md)
- [Generated output](docs/generated-output.md) and [native bindings](docs/native-bindings.md)
- [Changelog](CHANGELOG.md)
- [Report an issue](https://github.com/0xdoublesharp/abi-typegen/issues)

The [documentation index](docs/README.md) links the full set of guides.

## Contribute

See [CONTRIBUTING.md](CONTRIBUTING.md) for source builds, tests, code conventions,
and pull requests.
