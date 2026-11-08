# abi-typegen-config

<p align="center">
  <img src="https://raw.githubusercontent.com/doublesharp/abi-typegen/main/docs/abi-typegen.png" alt="abi-typegen" width="360" />
</p>

Read `foundry.toml` settings for
[abi-typegen](https://github.com/doublesharp/abi-typegen#readme). This crate provides
the shared `Config` and `Target` types for artifact paths, output directories,
target selection, contract filters, and wrapper generation.

## Install

Requires Rust 1.99 or newer.

```sh
cargo add abi-typegen-config
```

## Quick start

```rust
use abi_typegen_config::{Config, Target};

fn main() -> Result<(), abi_typegen_config::ConfigError> {
    let config = Config::from_toml_str(r#"
        [profile.default]
        out = "out"

        [abi-typegen]
        out = "src/generated"
        target = ["viem", "rust"]
        contracts = ["Token"]
        exclude = ["*Test", "*Mock"]
    "#)?;

    assert_eq!(config.targets, vec![Target::Viem, Target::Rust]);
    assert_eq!(config.out_dir, std::path::PathBuf::from("src/generated"));
    Ok(())
}
```

Use `Config::from_file` to load a file from disk. Without an `[abi-typegen]`
section, output defaults to `src/generated`, the target to viem, and wrappers to
enabled. Target names accept a single string, comma-separated names, or an array.
Parsing validates target names and target-specific package or namespace rules.

See the [configuration guide](https://github.com/doublesharp/abi-typegen/blob/main/docs/configuration.md),
[API reference](https://docs.rs/abi-typegen-config), and
[main README](https://github.com/doublesharp/abi-typegen#readme).
