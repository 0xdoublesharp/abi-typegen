//! Target-specific renderers for code generation.

/// C# contract bindings backed by Nethereum.
pub mod csharp;
/// Ethers v5 TypeScript contract interfaces.
pub mod ethers5;
/// Ethers v6 TypeScript contract interfaces.
pub mod ethers6;
mod ethers_common;
/// F# records and Nethereum contract helpers.
pub mod fsharp;
/// Go contract bindings backed by go-ethereum.
pub mod go;
/// Godot 4 GDScript bindings backed by the shared C runtime.
pub mod godot;
/// Kotlin contract bindings backed by web3j.
pub mod kotlin;
/// OCaml values and shared-runtime codecs.
pub mod ocaml;
/// Python contract bindings backed by web3.py.
pub mod python;
/// q/kdb+ event tables and decoders.
pub mod q;
/// Rust contract bindings backed by alloy `sol!`.
pub mod rust;
/// Solidity interface reconstruction.
pub mod solidity;
/// Swift contract bindings backed by web3swift.
pub mod swift;
/// Unreal Engine reflected adapters backed by the shared C runtime.
pub mod unreal;
/// Viem TypeScript contract helpers and parameter types.
pub mod viem;
/// Wagmi React hooks for contract reads, writes, and events.
pub mod wagmi;
/// Web3.js v4 TypeScript contract wrappers.
pub mod web3js;
/// YAML ABI descriptions.
pub mod yaml;
/// Zod 4 TypeScript validation schemas.
pub mod zod;

/// C11 contract bindings.
pub mod c;
/// C++17 contract bindings.
pub mod cpp;

/// Java contract bindings.
pub mod java;

/// Dart contract bindings.
pub mod dart;

/// PHP contract bindings.
pub mod php;

/// Experimental GnuCOBOL read bindings.
pub mod cobol;

/// Ruby contract bindings.
pub mod ruby;

/// Shell contract bindings backed by Foundry cast.
pub mod shell;

/// Elixir contract bindings backed by Ethers.
pub mod elixir;
