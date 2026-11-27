# magnetar-component-api

## Purpose

The contract between a Magnetar **Model Component** and the Runtime that runs
it: every released version of the `magnetar:model-component-graph` WIT package,
and the guest bindings for each, generated once.

A Model Component is a WebAssembly Component that describes a model family's
graph to the Runtime through that contract and never touches a Provider, a
Device or a native handle. This repository is what a Component's author
needs to target the contract, without a checkout of Magnetar:

- `wit/<version>/model-component-graph.wit`: each released version, `1.2.0` to
  `1.16.0`, immutable once released. `wit/SHA256SUMS` records them and
  `wit/PROVENANCE.md` says where each came from.
- the Rust guest bindings of each version, behind one cargo feature per
  version (`v1_16` for contract 1.16.0): a Component enables the one it
  targets, and neither copies the WIT file nor runs the bindings generator.
- the checks that make the contract's promise mechanical: a release only adds
  to the one before it, and a released file never changes.
- `examples/minimal-component`: the smallest Component, a template.

The contract's Runtime side (what serves the `graph-builder` and
`model-config` interfaces) is in [Magnetar](https://github.com/astorise/Magnetar);
the graph description and testing helpers a Component builds on are in
[`magnetar-component-sdk`](https://github.com/astorise/magnetar-component-sdk).

## Using it

While the crate is in development, a Component names it as a git dependency
pinned by commit (never a branch or a tag, which move); its `Cargo.lock`
records the commit, and the crate is published once it is stable.

```toml
[dependencies]
magnetar-component-api = { git = "https://github.com/astorise/magnetar-component-api", rev = "<commit>", features = ["v1_16"] }
```

```rust
use magnetar_component_api::v1_16::{
    Guest, export,
    magnetar::model_component_graph::graph_builder as gb,
};

struct Component;

impl Guest for Component {
    fn build_prefill_graph(sequence_length: u64) -> String { /* call gb::... */ }
    // build_decode_graph, build_prefill_graph_segment, build_decode_graph_segment
}

export!(Component);
```

`examples/minimal-component` is this, complete, and builds for `wasm32-wasip2`.
The SDK's `wit_graph_sink!` macro takes the `gb` alias from the snippet above.

**One contract version per build.** The generated export macros are
crate-level macros named after the world, the same in every version, so two
versions cannot be generated in one crate: enabling two features fails with a
message that says so. A Component targets one version; Components in one
Cargo workspace that target different versions would need separate builds.

**Which version?** The contract is additive: a Component built against an
earlier version keeps linking against a Runtime that serves a later one. Target
the oldest version that has what the Component needs, so it runs on the most
Runtimes.

## The rules of the contract

- **A released version never changes.** `wit/SHA256SUMS` and the tests fail if
  a released file differs from its record, and CI fails a pull request that
  edits or removes a file of a released version.
- **A release only adds to the one before it** (`tests/additive.rs`, checked on
  the WIT itself): no interface, function or type is removed, no signature or
  existing member changes. Only a variant or an enum may gain cases, at its
  end: the Runtime encodes each value against the calling Component's own
  types, so a Component built before the case fails its call instead of
  receiving a case it cannot represent. A breaking change is a new major
  version, in a new package version line.

## Releasing a version

1. Add `wit/<version>/model-component-graph.wit` (versions are `MAJOR.MINOR.0`).
2. Run `python3 scripts/sync.py`: it records the checksum and derives the
   cargo feature and the bindings module. `--check` changes nothing and fails
   if a file is out of date.
3. `cargo test --features v<MAJOR>_<MINOR>`; CI runs every version.

## Measured

On a copy of the Mistral Component moved onto this crate (contract 1.8.0): its
13 tests pass (the seven golden graphs recorded before the move among them, so
the graphs it emits are unchanged), the `.wasm` validates as a Component, and the component's
interface is the same as the one built from its own copy of the WIT (same
imports, same exports, same types, in a different declaration order). The
`.wasm` is 125,217 bytes against 122,630 (+2.1 %). The crate changes where
the bindings come from, not what a Component emits or how the Runtime executes
it, so what it buys is one source of the contract and no per-Component copy,
not inference speed; the size is the measured cost.

## Status

The Runtime still serves its own copy of the WIT (`magnetar-runtime/wit/`) and
the Components still carry theirs; moving them here is the OpenSpec change
`introduce-component-api` in Magnetar, Component by Component, each under
the golden graphs the SDK migration already uses.
