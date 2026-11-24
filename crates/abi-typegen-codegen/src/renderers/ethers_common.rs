use crate::type_mapper::{overload_suffix, safe_param_names};
use abi_typegen_core::types::{AbiEvent, AbiFunction, SolType};
use std::collections::{HashMap, HashSet};

/// Allocates overload aliases without hiding real methods or another alias.
///
/// When `reserve_filters` is true (the contract emits a `filters` block for
/// non-anonymous events), a Solidity function named `filters` receives an alias
/// so the generated interface does not declare duplicate `filters` members.
pub(super) fn method_names(functions: &[AbiFunction], reserve_filters: bool) -> Vec<String> {
    let mut counts = HashMap::new();
    let mut used: HashSet<String> = functions.iter().map(|f| f.name.clone()).collect();
    if reserve_filters {
        used.insert("filters".to_string());
    }
    for function in functions {
        *counts.entry(function.name.as_str()).or_insert(0) += 1;
    }
    functions
        .iter()
        .map(|function| {
            let collides_filters = reserve_filters && function.name == "filters";
            if counts[function.name.as_str()] == 1 && !collides_filters {
                return function.name.clone();
            }
            let base = if counts[function.name.as_str()] > 1 {
                format!("{}{}", function.name, overload_suffix(&function.inputs))
            } else {
                format!("{}Fn", function.name)
            };
            let mut candidate = base.clone();
            let mut suffix = 2;
            while !used.insert(candidate.clone()) {
                candidate = format!("{base}_{suffix}");
                suffix += 1;
            }
            candidate
        })
        .collect()
}

/// Canonical ABI event signature (e.g. `Transfer(address,address,uint256)`).
pub(super) fn event_signature(event: &AbiEvent) -> String {
    format!(
        "{}({})",
        event.name,
        event
            .inputs
            .iter()
            .map(|input| canonical_type(&input.ty))
            .collect::<Vec<_>>()
            .join(",")
    )
}

/// Returns whether a tuple output type renders as an intersection (`[...] & { ... }`)
/// and therefore needs parentheses when wrapped in `[]`.
pub(super) fn tuple_output_is_intersection(ty: &SolType) -> bool {
    match ty {
        SolType::Tuple(components) => components
            .iter()
            .any(|c| !c.name.is_empty() && !is_tuple_property_name(&c.name)),
        SolType::Uint(_)
        | SolType::Int(_)
        | SolType::Bool
        | SolType::Address
        | SolType::Bytes
        | SolType::BytesN(_)
        | SolType::StringType
        | SolType::Array(_)
        | SolType::FixedArray(_, _) => false,
    }
}

/// Returns whether `name` collides with a property inherited from `Array.prototype`
/// or `Object.prototype` on an ethers tuple `Result`.
pub(super) fn is_tuple_property_name(name: &str) -> bool {
    matches!(
        name,
        "length"
            | "constructor"
            | "valueOf"
            | "hasOwnProperty"
            | "isPrototypeOf"
            | "propertyIsEnumerable"
            | "toString"
            | "toLocaleString"
            | "at"
            | "fill"
            | "copyWithin"
            | "pop"
            | "push"
            | "concat"
            | "join"
            | "reverse"
            | "shift"
            | "slice"
            | "sort"
            | "splice"
            | "unshift"
            | "indexOf"
            | "lastIndexOf"
            | "every"
            | "some"
            | "forEach"
            | "map"
            | "filter"
            | "reduce"
            | "reduceRight"
            | "find"
            | "findIndex"
            | "findLast"
            | "findLastIndex"
            | "entries"
            | "keys"
            | "values"
            | "includes"
            | "flatMap"
            | "flat"
            | "toReversed"
            | "toSorted"
            | "toSpliced"
            | "with"
    )
}

/// Name for the trailing overrides parameter that no ABI input already uses.
pub(super) fn overrides_param_name(function: &AbiFunction) -> String {
    let mut name = "overrides".to_string();
    let names = safe_param_names(function.inputs.iter().map(|input| input.name.as_str()));
    while names.contains(&name) {
        name.insert(0, '_');
    }
    name
}

/// Canonical ABI signature used to select the actual ethers runtime method.
pub(super) fn function_signature(function: &AbiFunction) -> String {
    format!(
        "{}({})",
        function.name,
        function
            .inputs
            .iter()
            .map(|input| canonical_type(&input.ty))
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn canonical_type(ty: &SolType) -> String {
    match ty {
        SolType::Uint(bits) => format!("uint{bits}"),
        SolType::Int(bits) => format!("int{bits}"),
        SolType::Bool => "bool".into(),
        SolType::Address => "address".into(),
        SolType::Bytes => "bytes".into(),
        SolType::BytesN(size) => format!("bytes{size}"),
        SolType::StringType => "string".into(),
        SolType::Array(inner) => format!("{}[]", canonical_type(inner)),
        SolType::FixedArray(inner, size) => format!("{}[{size}]", canonical_type(inner)),
        SolType::Tuple(fields) => format!(
            "({})",
            fields
                .iter()
                .map(|field| canonical_type(&field.ty))
                .collect::<Vec<_>>()
                .join(",")
        ),
    }
}
