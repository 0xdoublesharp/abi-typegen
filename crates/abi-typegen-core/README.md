# abi-typegen-core

<p align="center">
  <img src="https://raw.githubusercontent.com/doublesharp/abi-typegen/main/docs/abi-typegen.png" alt="abi-typegen" width="360" />
</p>

Parse Solidity ABI artifacts into the contract types used by
[abi-typegen](https://github.com/doublesharp/abi-typegen#readme). Use this crate to
inspect contracts or build tools that work with ABI functions, events, errors,
tuples, and NatSpec documentation.

## Install

Requires Rust 1.99 or newer.

```sh
cargo add abi-typegen-core
```

## Quick start

`parse_artifact` reads a JSON artifact with an `abi` field, as produced by Foundry
or Hardhat, and returns a `ContractIr`.

```rust
use abi_typegen_core::parser::parse_artifact;

fn main() -> Result<(), abi_typegen_core::ParseError> {
    let artifact = r#"{
        "abi": [{
            "type": "function",
            "name": "balanceOf",
            "inputs": [{"name": "owner", "type": "address"}],
            "outputs": [{"name": "balance", "type": "uint256"}],
            "stateMutability": "view"
        }]
    }"#;
    let contract = parse_artifact("Token", artifact)?;

    assert_eq!(contract.name, "Token");
    assert_eq!(contract.functions[0].signature(), "balanceOf(address)");
    Ok(())
}
```

The parser validates Solidity types, including integer widths and fixed-byte
sizes. Contract items expose canonical signatures, function and error selectors,
and event topics. Pair this crate with
[abi-typegen-codegen](https://crates.io/crates/abi-typegen-codegen) to render bindings.

See the [API reference](https://docs.rs/abi-typegen-core) and the
[main README](https://github.com/doublesharp/abi-typegen#readme) for the full generator.
