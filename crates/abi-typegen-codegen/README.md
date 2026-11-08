# abi-typegen-codegen

<p align="center">
  <img src="https://raw.githubusercontent.com/doublesharp/abi-typegen/main/docs/abi-typegen.png" alt="abi-typegen" width="360" />
</p>

Generate source files from the parsed Solidity contracts used by
[abi-typegen](https://github.com/doublesharp/abi-typegen#readme). This crate contains
the target renderers, type mappings, naming helpers, and ABI output for TypeScript,
native languages, game engines, and other formats.

## Install

Requires Rust 1.99 or newer. Add the parser and configuration crates to embed the
generation pipeline:

```sh
cargo add abi-typegen-codegen abi-typegen-core abi-typegen-config
```

## Quick start

After compiling a `Token` contract with Foundry:

```rust
use abi_typegen_codegen::generate_contract_files;
use abi_typegen_config::Config;
use abi_typegen_core::parser::parse_artifact;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let artifact = std::fs::read_to_string("out/Token.sol/Token.json")?;
    let contract = parse_artifact("Token", &artifact)?;
    let config = Config::from_toml_str(r#"
        [abi-typegen]
        target = "viem"
    "#)?;
    let files = generate_contract_files(&contract, &config);

    assert!(files.contains_key("Token.abi.ts"));
    assert!(files.contains_key("Token.viem.ts"));
    Ok(())
}
```

Each call returns a map of filenames to source strings for one contract and the
first target in `Config`. Your application chooses how to write those files. For
multiple targets, call it with a separate configuration for each target. The CLI
handles artifact discovery, shared support files, output directories, and watching.

The consuming app needs the runtime or SDK for its chosen target. See the
[target list](https://github.com/doublesharp/abi-typegen#choose-your-target),
[generated output guide](https://github.com/doublesharp/abi-typegen/blob/main/docs/generated-output.md),
and [API reference](https://docs.rs/abi-typegen-codegen). The
[main README](https://github.com/doublesharp/abi-typegen#readme) covers CLI usage.
