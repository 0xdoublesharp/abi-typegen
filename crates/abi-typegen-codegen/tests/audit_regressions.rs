use abi_typegen_codegen::{
    c, cobol, cpp, csharp, dart, elixir, ethers, ethers5, fsharp, go, godot, java, kotlin, ocaml,
    php, python, ruby, rust, solidity, swift, type_mapper, unreal, viem, wagmi, web3js, yaml, zod,
};
use abi_typegen_core::parser::parse_artifact;
use abi_typegen_core::types::{ContractIr, NatSpec, SolType, TupleComponent};
use serde_json::{Value, json};
use std::collections::HashSet;

fn parse(name: &str, entries: Value) -> ContractIr {
    parse_artifact(name, &json!({"abi": entries}).to_string()).expect("valid ABI")
}

#[test]
fn yaml_emits_valid_block_mappings_tuple_array_components_and_specials() {
    let mut ir = parse(
        "Vault",
        json!([
            {
                "type": "function",
                "name": "batch",
                "inputs": [{
                    "name": "items",
                    "type": "tuple[]",
                    "components": [
                        {"name": "owner", "type": "address"},
                        {"name": "amount", "type": "uint256"}
                    ]
                }],
                "outputs": [],
                "stateMutability": "nonpayable"
            },
            {"type": "fallback", "stateMutability": "payable"},
            {"type": "receive", "stateMutability": "payable"}
        ]),
    );
    ir.natspec = Some(NatSpec {
        notice: Some("Line 1\nLine 2\t\"quoted\"".into()),
        dev: Some("Dev note".into()),
        ..NatSpec::default()
    });
    let out = yaml::render_yaml_file(&ir);
    assert!(
        !out.contains(", type:"),
        "YAML sequences must use block mappings:\n{out}"
    );
    assert!(
        out.contains("- name: \"items\"\n        type: \"tuple[]\""),
        "{out}"
    );
    assert!(out.contains("components:"), "{out}");
    assert!(
        out.contains("notice: \"Line 1\\nLine 2\\t\\\"quoted\\\"\""),
        "{out}"
    );
    assert!(out.contains("dev: \"Dev note\""), "{out}");
    assert!(
        out.contains("fallback:\n  stateMutability: \"payable\""),
        "{out}"
    );
    assert!(out.contains("receive: true"), "{out}");
}

#[test]
fn solidity_avoids_struct_collisions_sanitizes_identifiers_and_wraps_multiline_natspec() {
    let mut ir = parse(
        "Vault",
        json!([
            {
                "type": "function",
                "name": "deposit",
                "inputs": [
                    {
                        "name": "a",
                        "type": "tuple",
                        "internalType": "struct Vault.IVault",
                        "components": [{"name": "x", "type": "uint256"}]
                    },
                    {
                        "name": "function",
                        "type": "tuple",
                        "internalType": "struct Vault.$",
                        "components": [{"name": "const", "type": "bool"}]
                    }
                ],
                "outputs": [{"name": "a", "type": "uint256"}],
                "stateMutability": "nonpayable"
            }
        ]),
    );
    let mut returns = std::collections::HashMap::new();
    returns.insert("a".into(), "shadowed return doc".into());
    ir.functions[0].natspec = Some(NatSpec {
        notice: Some("First line\nSecond line".into()),
        returns,
        ..NatSpec::default()
    });
    let out = solidity::render_solidity_file(&ir);
    assert!(out.contains("struct IVault_2 {"), "{out}");
    assert!(out.contains("struct DepositFunction {"), "{out}");
    assert!(out.contains("bool const_;"), "{out}");
    assert!(out.contains("DepositFunction calldata function_"), "{out}");
    assert!(
        out.contains("/// @notice First line\n    /// Second line\n"),
        "{out}"
    );
    assert!(
        !out.contains("@return a"),
        "stripped output name must not emit @return tag:\n{out}"
    );
}

#[test]
fn ts_viem_zod_and_type_mapper_escape_jsdoc_and_deduplicate_tuple_fields() {
    let tuple = SolType::Tuple(vec![
        TupleComponent {
            name: String::new(),
            ty: SolType::Uint(256),
            internal_type: None,
        },
        TupleComponent {
            name: "arg0".into(),
            ty: SolType::Bool,
            internal_type: None,
        },
    ]);
    let ts_ty_viem = type_mapper::sol_type_to_ts(&tuple, type_mapper::Target::Viem);
    assert_eq!(ts_ty_viem, "readonly [bigint, boolean]", "{ts_ty_viem}");
    let ts_ty_ethers = type_mapper::sol_type_to_ts(&tuple, type_mapper::Target::Ethers);
    assert!(
        ts_ty_ethers.contains("arg0: bigint") && ts_ty_ethers.contains("arg02: boolean"),
        "{ts_ty_ethers}"
    );

    let mut ir = parse(
        "Token",
        json!([
            {
                "type": "function",
                "name": "noop",
                "inputs": [],
                "outputs": [],
                "stateMutability": "view"
            },
            {
                "type": "function",
                "name": "read",
                "inputs": [{
                    "name": "item",
                    "type": "tuple",
                    "components": [
                        {"name": "", "type": "uint256"},
                        {"name": "arg0", "type": "bool"}
                    ]
                }],
                "outputs": [{"name": "", "type": "uint256"}],
                "stateMutability": "nonpayable"
            }
        ]),
    );
    let mut returns = std::collections::HashMap::new();
    returns.insert("_0".into(), "Break */ comment\nLine 2".into());
    ir.functions[1].natspec = Some(NatSpec {
        notice: Some("Notice */ end\nNext line".into()),
        returns,
        ..NatSpec::default()
    });

    let viem_out = viem::render_viem_file(&ir);
    assert!(
        viem_out.contains("Notice *\\/ end\n * Next line"),
        "{viem_out}"
    );
    assert!(!viem_out.contains("Notice */ end"), "{viem_out}");

    let zod_out = zod::render_zod_file(&ir);
    assert!(zod_out.contains("arg02: z.boolean()"), "{zod_out}");
    assert!(zod_out.contains("Break *\\/ comment"), "{zod_out}");
    assert!(!zod_out.contains("noopParamsSchema"), "{zod_out}");
}

#[test]
fn ethers_wagmi_and_web3js_handle_filters_base_connect_anonymous_and_punctuation() {
    let ir = parse(
        "Base",
        json!([
            {"type": "function", "name": "filters", "inputs": [], "outputs": [], "stateMutability": "view"},
            {"type": "function", "name": "deposit", "inputs": [{"name": "a", "type": "uint256"}], "outputs": [], "stateMutability": "payable"},
            {"type": "function", "name": "deposit", "inputs": [{"name": "a", "type": "address"}], "outputs": [], "stateMutability": "payable"},
            {"type": "event", "name": "Ping", "anonymous": false, "inputs": [{"name": "x", "type": "uint256", "indexed": true}]},
            {"type": "event", "name": "Ping", "anonymous": false, "inputs": [{"name": "x", "type": "address", "indexed": true}]},
            {"type": "event", "name": "Secret", "anonymous": true, "inputs": [{"name": "x", "type": "uint256", "indexed": true}]}
        ]),
    );
    let e6 = ethers::render_ethers_file(&ir);
    assert!(e6.contains("BaseContract as EthersBaseContract"), "{e6}");
    assert!(e6.contains("filtersFn("), "{e6}");
    assert!(
        e6.contains("connect: (nextRunner: ContractRunner | null) => attachAliases("),
        "{e6}"
    );
    assert!(e6.contains("\"Ping(uint256)\"("), "{e6}");
    assert!(e6.contains("\"Ping(address)\"("), "{e6}");
    assert!(e6.contains("Secret("), "{e6}");

    let e5 = ethers5::render_ethers5_file(&ir);
    assert!(e5.contains("filtersFn("), "{e5}");
    assert!(e5.contains("\"Ping(uint256)\"("), "{e5}");
    assert!(e5.contains("Secret("), "{e5}");

    let wagmi_out = wagmi::render_wagmi_file(&ir);
    assert!(wagmi_out.contains("eventName: 'Secret'"), "{wagmi_out}");

    let odd = parse(
        "_",
        json!([{
            "type": "function",
            "name": "f",
            "inputs": [{
                "name": "t",
                "type": "tuple",
                "components": [{"name": "", "type": "uint256"}, {"name": "arg0", "type": "bool"}]
            }],
            "outputs": [],
            "stateMutability": "view"
        }]),
    );
    let w3 = web3js::render_web3js_file(&odd);
    assert!(w3.contains("export function create_("), "{w3}");
    assert!(w3.contains("arg02: boolean"), "{w3}");
}

#[test]
fn c_cpp_rust_go_and_swift_handle_edge_identifiers_and_escaping() {
    let ir = parse(
        "Type",
        json!([
            {
                "type": "function",
                "name": "store",
                "inputs": [
                    {"name": "const", "type": "uint256[0]"},
                    {"name": "a", "type": "tuple", "internalType": "struct T.A$B", "components": [{"name": "x", "type": "uint256"}]},
                    {"name": "b", "type": "tuple", "internalType": "struct T.A$$B", "components": [{"name": "y", "type": "bool"}]}
                ],
                "outputs": [],
                "stateMutability": "payable"
            },
            {
                "type": "event",
                "name": "Log",
                "anonymous": false,
                "inputs": [{"name": "", "type": "address", "indexed": true}]
            }
        ]),
    );
    let c_out = c::render_c_file(&ir, true);
    assert!(c_out.contains("tuple_atg_TA_B;"), "{c_out}");
    assert!(c_out.contains("tuple_atg_TA_B2;"), "{c_out}");
    assert!(!c_out.contains("for (size_t i = 0; i < 0;"), "{c_out}");
    let cpp_out = cpp::render_cpp_file(&ir, true);
    assert!(
        cpp_out.contains("static constexpr const char *abi ="),
        "{cpp_out}"
    );

    let rust_out = rust::render_rust_file(&ir, true);
    assert!(
        rust_out.contains("uint256[0] calldata const_"),
        "{rust_out}"
    );
    assert!(rust_out.contains("struct TA_B {"), "{rust_out}");
    assert!(rust_out.contains("struct TA__B {"), "{rust_out}");
    assert!(rust::render_mod_file(&["$".to_string()]).contains("pub mod contract_;"));
    assert!(rust::render_mod_file(&["token".to_string()]).contains("pub mod token_mod;"));

    let go_out = go::render_go_file_with_wrappers(&ir, "contracts", true);
    assert!(go_out.contains("DecodeLogEvent"), "{go_out}");
    assert!(go_out.contains("abi.Arguments{{"), "{go_out}");

    assert_eq!(swift::namespace_name("Type"), "Type2");
    let swift_out = swift::render_swift_file_with_wrappers(&ir, true);
    assert!(swift_out.contains("value: BigUInt? = nil"), "{swift_out}");
    assert!(
        swift_out.contains("if let value { tx.value = value }"),
        "{swift_out}"
    );
}

#[test]
fn kotlin_java_csharp_and_fsharp_handle_zero_arrays_anonymous_events_and_dollars() {
    let ir = parse(
        "Token",
        json!([
            {"type": "constructor", "inputs": [{"name": "supply", "type": "uint256"}], "stateMutability": "nonpayable"},
            {
                "type": "function",
                "name": "read",
                "inputs": [
                    {"name": "client", "type": "address"},
                    {"name": "empty", "type": "string[0]"},
                    {"name": "t", "type": "tuple", "internalType": "struct Token.Foo$Bar", "components": [{"name": "x", "type": "uint256"}]}
                ],
                "outputs": [{"name": "a", "type": "uint256"}, {"name": "b", "type": "bool"}],
                "stateMutability": "view"
            },
            {
                "type": "event",
                "name": "Secret",
                "anonymous": true,
                "inputs": [{"name": "", "type": "address", "indexed": true}]
            }
        ]),
    );
    let kt = kotlin::render_kotlin_file_with_wrappers(&ir, "contracts", true);
    assert!(
        kt.contains("fixedSize == null || (fixedSize > 0 && element.dynamic)"),
        "{kt}"
    );
    assert!(
        kt.contains("fun callRead(client: Client, client2: String"),
        "{kt}"
    );
    assert!(kt.contains("address2: String? = null"), "{kt}");

    let jv = java::render_java_file(&ir, "contracts", true);
    assert!(
        jv.contains("fixedSize == null || (fixedSize > 0 && element.dynamic())"),
        "{jv}"
    );
    assert!(
        jv.contains("callRead(Client client, String client2"),
        "{jv}"
    );
    assert!(jv.contains("String address2)"), "{jv}");

    let cs_ctor_only = parse(
        "Deploy",
        json!([{"type": "constructor", "inputs": [{"name": "supply", "type": "uint256"}], "stateMutability": "nonpayable"}]),
    );
    let cs = csharp::render_csharp_file_with_wrappers(&cs_ctor_only, false);
    assert!(cs.contains("using System.Numerics;"), "{cs}");

    let cs_anon = csharp::render_csharp_file_with_wrappers(&ir, true);
    assert!(cs_anon.contains("[Event(\"Secret\", true)]"), "{cs_anon}");
    assert!(
        cs_anon.contains("filter.Topics = Array.Empty<object>();"),
        "{cs_anon}"
    );
    assert!(cs_anon.contains("public class TokenFoo_Bar"), "{cs_anon}");

    let fs = fsharp::render_fsharp_file(&ir, true);
    assert!(!fs.contains("TupleFoo$Bar"), "{fs}");
    assert!(fs.contains("type TupleFooBar ="), "{fs}");
}

#[test]
fn ruby_python_php_dart_elixir_ocaml_and_cobol_regressions() {
    let ir = parse(
        "Token",
        json!([
            {
                "type": "function",
                "name": "for",
                "inputs": [
                    {"name": "function", "type": "uint256"},
                    {"name": "items", "type": "tuple[]", "internalType": "struct Token.Row[]", "components": [{"name": "to_abi", "type": "uint256"}]}
                ],
                "outputs": [],
                "stateMutability": "view"
            },
            {
                "type": "event",
                "name": "Moved",
                "anonymous": false,
                "inputs": [
                    {"name": "fromBlock", "type": "uint256", "indexed": true},
                    {"name": "topics", "type": "address", "indexed": true}
                ]
            },
            {"type": "error", "name": "bad_error", "inputs": [{"name": "x", "type": "uint"}]}
        ]),
    );

    let rb = ruby::render_ruby_file_with_wrappers(&ir, true);
    assert!(rb.contains("self[:to_abi]"), "{rb}");
    assert!(rb.contains("_abi_function = _function("), "{rb}");

    let py_meta = python::render_python_file_with_wrappers(&ir, false);
    assert!(!py_meta.contains("from web3 import Web3"), "{py_meta}");
    assert!(py_meta.contains("class TokenRow(TypedDict):"), "{py_meta}");

    let php_out = php::render_php_file(&ir, "Contracts", true);
    assert!(
        php_out.contains("$layout['size'] > 0 && self::layoutDynamic($layout['element'])"),
        "{php_out}"
    );

    let dart_out = dart::render_dart_file(&ir, true);
    assert!(dart_out.contains("for_Signature"), "{dart_out}");
    assert!(dart_out.contains("BigInt? fromBlock2"), "{dart_out}");
    assert!(dart_out.contains("EthereumAddress? topics2"), "{dart_out}");
    assert!(
        dart_out.contains("topics[0]?.toLowerCase() != movedTopic0"),
        "{dart_out}"
    );

    let elixir_problems = elixir::unsupported_sdk_collisions(&ir);
    assert!(
        elixir_problems
            .iter()
            .any(|p| p.contains("invalid error module alias")),
        "{elixir_problems:?}"
    );

    let ocaml_art = ocaml::render_ocaml_artifacts(&ir, true);
    assert!(
        ocaml_art.bridge.contains("ATG_OCAML_MAX_DEPTH"),
        "{}",
        ocaml_art.bridge
    );

    let cb = cobol::render_cobol_artifacts(&ir, false);
    assert!(cb.cobol.contains("*> FUNCTION: for("), "{}", cb.cobol);
}

#[test]
fn godot_and_unreal_regressions_avoid_key_mismatch_and_delegate_collisions() {
    let ir = parse(
        "Game",
        json!([
            {"type": "function", "name": "deferred", "inputs": [], "outputs": [], "stateMutability": "view"},
            {"type": "function", "name": "fooEventWatch", "inputs": [{"name": "fString", "type": "string"}], "outputs": [], "stateMutability": "view"},
            {"type": "event", "name": "Foo", "anonymous": false, "inputs": [{"name": "", "type": "string", "indexed": true}]}
        ]),
    );

    let gd = godot::render_godot_file(&ir, true);
    assert!(gd.contains("FOO_EVENT_NAMES := [\"string\"]"), "{gd}");
    assert!(gd.contains("data[\"string\"]"), "{gd}");
    assert!(!gd.contains("static func call_deferred("), "{gd}");
    assert!(gd.contains("AbiTypegenOperations.failed_watcher("), "{gd}");

    let ue = unreal::render_unreal_artifacts(&ir, true, "HOST_API");
    let symbols = unreal::declared_symbol_names(&ir, true);
    let lower: HashSet<String> = symbols.iter().map(|s| s.to_ascii_lowercase()).collect();
    assert_eq!(
        symbols.len(),
        lower.len(),
        "duplicate Unreal symbols: {symbols:?}"
    );
    assert!(ue.header.contains("FString FString2"), "{}", ue.header);
    assert_eq!(
        ue.header.matches("FGameFooEventWatchFailure").count(),
        2, // DECLARE_DYNAMIC_MULTICAST_DELEGATE_OneParam + UPROPERTY on the read node only
        "{}",
        ue.header
    );
}
