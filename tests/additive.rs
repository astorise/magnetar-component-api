//! A release of the contract only adds to the one before it.
//!
//! The contract is versioned additively: a Component built against an earlier
//! version keeps linking against a Runtime that serves a later one. This test
//! checks it on the WIT itself, for every pair of consecutive versions in
//! `wit/`, so that a change which would break a Component is refused here,
//! before it reaches a Runtime.

use std::{collections::BTreeMap, fs, path::Path};
use wit_parser::{Resolve, Type, TypeDefKind, TypeId};

mod versions;

/// A resolved version of the package: interface name to what it declares.
struct Package {
    resolve: Resolve,
    interfaces: BTreeMap<String, wit_parser::InterfaceId>,
}

fn parse(name: &str, source: &str) -> Package {
    let mut resolve = Resolve::default();
    let id = resolve
        .push_source(name, source)
        .unwrap_or_else(|error| panic!("{name}: {error:#}"));
    let interfaces = resolve.packages[id]
        .interfaces
        .iter()
        .map(|(name, id)| (name.clone(), *id))
        .collect();
    Package {
        resolve,
        interfaces,
    }
}

fn load(version: &str) -> Package {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("wit")
        .join(version)
        .join("model-component-graph.wit");
    let source = fs::read_to_string(&path).unwrap_or_else(|error| panic!("{path:?}: {error}"));
    parse(version, &source)
}

/// A type as a string: a named type by its name (its definition is compared
/// on its own), an anonymous one structurally.
fn render(resolve: &Resolve, ty: &Type) -> String {
    match ty {
        Type::Id(id) => {
            let def = &resolve.types[*id];
            if let Some(name) = &def.name {
                return name.clone();
            }
            match &def.kind {
                TypeDefKind::Option(inner) => format!("option<{}>", render(resolve, inner)),
                TypeDefKind::List(inner) => format!("list<{}>", render(resolve, inner)),
                TypeDefKind::Tuple(tuple) => format!(
                    "tuple<{}>",
                    tuple
                        .types
                        .iter()
                        .map(|ty| render(resolve, ty))
                        .collect::<Vec<_>>()
                        .join(",")
                ),
                TypeDefKind::Result(result) => format!(
                    "result<{},{}>",
                    result
                        .ok
                        .as_ref()
                        .map_or("_".into(), |ty| render(resolve, ty)),
                    result
                        .err
                        .as_ref()
                        .map_or("_".into(), |ty| render(resolve, ty)),
                ),
                other => panic!("an anonymous {} is not rendered", other.as_str()),
            }
        }
        primitive => format!("{primitive:?}"),
    }
}

/// The definition of a named type: its kind and, in order, its members.
fn members(resolve: &Resolve, id: TypeId) -> (&'static str, Vec<String>) {
    let def = &resolve.types[id];
    let kind = def.kind.as_str();
    let list = match &def.kind {
        TypeDefKind::Record(record) => record
            .fields
            .iter()
            .map(|f| format!("{}: {}", f.name, render(resolve, &f.ty)))
            .collect(),
        TypeDefKind::Variant(variant) => variant
            .cases
            .iter()
            .map(|c| {
                format!(
                    "{}({})",
                    c.name,
                    c.ty.as_ref()
                        .map_or(String::new(), |ty| render(resolve, ty))
                )
            })
            .collect(),
        TypeDefKind::Enum(en) => en.cases.iter().map(|c| c.name.clone()).collect(),
        TypeDefKind::Flags(flags) => flags.flags.iter().map(|f| f.name.clone()).collect(),
        TypeDefKind::Type(ty) => vec![render(resolve, ty)],
        TypeDefKind::Resource => vec![],
        other => panic!("a named {} is not compared", other.as_str()),
    };
    (kind, list)
}

/// What `next` changes in `previous` other than by adding: one line each.
fn breaks(previous: &Package, next: &Package) -> Vec<String> {
    let mut found = Vec::new();
    for (name, old_id) in &previous.interfaces {
        let Some(new_id) = next.interfaces.get(name) else {
            found.push(format!("interface {name} is removed"));
            continue;
        };
        let old = &previous.resolve.interfaces[*old_id];
        let new = &next.resolve.interfaces[*new_id];
        for (function, old_fn) in &old.functions {
            let Some(new_fn) = new.functions.get(function) else {
                found.push(format!("{name}.{function} is removed"));
                continue;
            };
            let signature = |resolve: &Resolve, f: &wit_parser::Function| {
                let params: Vec<_> = f
                    .params
                    .iter()
                    .map(|p| format!("{}: {}", p.name, render(resolve, &p.ty)))
                    .collect();
                let result = f.result.as_ref().map(|ty| render(resolve, ty));
                (params, result)
            };
            if signature(&previous.resolve, old_fn) != signature(&next.resolve, new_fn) {
                found.push(format!("{name}.{function} changes its signature"));
            }
        }
        for (ty_name, old_ty) in &old.types {
            let Some(new_ty) = new.types.get(ty_name) else {
                found.push(format!("{name}: type {ty_name} is removed"));
                continue;
            };
            let (old_kind, old_members) = members(&previous.resolve, *old_ty);
            let (new_kind, new_members) = members(&next.resolve, *new_ty);
            if old_kind != new_kind {
                found.push(format!(
                    "{name}: {ty_name} goes from {old_kind} to {new_kind}"
                ));
            } else if new_members.get(..old_members.len()) != Some(&old_members[..]) {
                found.push(format!("{name}: {ty_name} changes its existing members"));
            } else if new_members.len() != old_members.len()
                && !matches!(old_kind, "variant" | "enum")
            {
                // A variant or an enum may gain cases at its end: the host
                // encodes each value against the calling Component's own
                // types, so a Component built before the case fails its call
                // instead of receiving a case it cannot represent. Anything
                // else that grows (a record's fields, flags) changes the
                // layout a Component was built against.
                found.push(format!(
                    "{name}: {ty_name} gains {:?}, and a {old_kind} may not",
                    &new_members[old_members.len()..]
                ));
            }
        }
    }
    found
}

#[test]
fn every_release_only_adds_to_the_one_before() {
    let versions = versions::released();
    assert!(versions.len() >= 2, "the contract has released versions");
    for pair in versions.windows(2) {
        let found = breaks(&load(&pair[0]), &load(&pair[1]));
        assert!(found.is_empty(), "{} -> {}: {found:#?}", pair[0], pair[1]);
    }
}

const BASE: &str = r#"
package test:contract@1.0.0;
interface api {
  record shape { dims: list<u64> }
  enum phase { prefill, decode }
  variant scaling { banded(f64) }
  flags caps { a, b }
  size-of: func(s: shape, n: u32) -> option<u64>;
}
"#;

fn next(source: &str) -> Vec<String> {
    breaks(&parse("base", BASE), &parse("next", source))
}

#[test]
fn additions_are_not_breaks() {
    let grown = BASE
        .replace("decode }", "decode, encode }")
        .replace("banded(f64) }", "banded(f64), yarn(f64) }")
        .replace("@1.0.0", "@1.1.0")
        + "interface extra { new-call: func() -> u32; }";
    assert_eq!(next(&grown), Vec::<String>::new());
}

#[test]
fn removals_and_changes_are_breaks() {
    let cases = [
        (
            "a removed function",
            BASE.replace("size-of: func(s: shape, n: u32) -> option<u64>;", ""),
            "size-of is removed",
        ),
        (
            "a changed parameter",
            BASE.replace("n: u32", "n: u64"),
            "changes its signature",
        ),
        (
            "a changed result",
            BASE.replace("option<u64>", "u64"),
            "changes its signature",
        ),
        (
            "a removed interface",
            "package test:contract@1.1.0; interface other { f: func(); }".to_string(),
            "interface api is removed",
        ),
        (
            "a removed type",
            BASE.replace("record shape { dims: list<u64> }", "")
                .replace("s: shape", "s: u32"),
            "type shape is removed",
        ),
        (
            "a record field",
            BASE.replace("dims: list<u64> }", "dims: list<u64>, rank: u32 }"),
            "gains",
        ),
        (
            "a changed field",
            BASE.replace("dims: list<u64>", "dims: list<u32>"),
            "changes its existing members",
        ),
        (
            "a reordered enum",
            BASE.replace("prefill, decode", "decode, prefill"),
            "changes its existing members",
        ),
        (
            "a removed enum case",
            BASE.replace("prefill, decode", "prefill"),
            "changes its existing members",
        ),
        ("a new flag", BASE.replace("a, b", "a, b, c"), "gains"),
    ];
    for (what, source, expected) in cases {
        let found = next(&source);
        assert!(
            found.iter().any(|line| line.contains(expected)),
            "{what}: expected a break containing {expected:?}, found {found:#?}"
        );
    }
}
