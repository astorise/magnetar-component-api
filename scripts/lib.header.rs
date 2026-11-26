//! The Magnetar Model Component contract.
//!
//! Every released version of the `magnetar:model-component-graph` WIT package
//! lives in `wit/<version>/` and never changes once released. This crate
//! generates the guest bindings of a version once, in the module of the
//! matching feature (`v1_14` for contract 1.14.0), so that a Model Component
//! neither copies the WIT file nor runs the bindings generator itself.
//!
//! ```ignore
//! // Cargo.toml: magnetar-component-api = { git = "...", rev = "...", features = ["v1_14"] }
//! use magnetar_component_api::v1_14::{Guest, export, magnetar::model_component_graph::graph_builder as gb};
//!
//! struct Component;
//! impl Guest for Component { /* build_prefill_graph, ... */ }
//! export!(Component);
//! ```
