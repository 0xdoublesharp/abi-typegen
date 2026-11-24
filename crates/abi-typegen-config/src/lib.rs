//! Configuration parsing for abi-typegen (`foundry.toml`).

pub mod config;
pub use config::{
    Config, ConfigError, DEFAULT_PACKAGE, EXPECTED_TARGETS, Target, parse_target, validate_package,
};
