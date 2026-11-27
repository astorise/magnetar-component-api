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

// The generated export macros are crate-level macros named after the
// world, the same in every version, so two versions cannot be generated
// in one crate. A build targets one contract version.
#[rustfmt::skip]
#[cfg(any(
    all(feature = "v1_2", any(feature = "v1_3", feature = "v1_4", feature = "v1_5", feature = "v1_6", feature = "v1_7", feature = "v1_8", feature = "v1_9", feature = "v1_10", feature = "v1_11", feature = "v1_12", feature = "v1_13", feature = "v1_14", feature = "v1_15")),
    all(feature = "v1_3", any(feature = "v1_2", feature = "v1_4", feature = "v1_5", feature = "v1_6", feature = "v1_7", feature = "v1_8", feature = "v1_9", feature = "v1_10", feature = "v1_11", feature = "v1_12", feature = "v1_13", feature = "v1_14", feature = "v1_15")),
    all(feature = "v1_4", any(feature = "v1_2", feature = "v1_3", feature = "v1_5", feature = "v1_6", feature = "v1_7", feature = "v1_8", feature = "v1_9", feature = "v1_10", feature = "v1_11", feature = "v1_12", feature = "v1_13", feature = "v1_14", feature = "v1_15")),
    all(feature = "v1_5", any(feature = "v1_2", feature = "v1_3", feature = "v1_4", feature = "v1_6", feature = "v1_7", feature = "v1_8", feature = "v1_9", feature = "v1_10", feature = "v1_11", feature = "v1_12", feature = "v1_13", feature = "v1_14", feature = "v1_15")),
    all(feature = "v1_6", any(feature = "v1_2", feature = "v1_3", feature = "v1_4", feature = "v1_5", feature = "v1_7", feature = "v1_8", feature = "v1_9", feature = "v1_10", feature = "v1_11", feature = "v1_12", feature = "v1_13", feature = "v1_14", feature = "v1_15")),
    all(feature = "v1_7", any(feature = "v1_2", feature = "v1_3", feature = "v1_4", feature = "v1_5", feature = "v1_6", feature = "v1_8", feature = "v1_9", feature = "v1_10", feature = "v1_11", feature = "v1_12", feature = "v1_13", feature = "v1_14", feature = "v1_15")),
    all(feature = "v1_8", any(feature = "v1_2", feature = "v1_3", feature = "v1_4", feature = "v1_5", feature = "v1_6", feature = "v1_7", feature = "v1_9", feature = "v1_10", feature = "v1_11", feature = "v1_12", feature = "v1_13", feature = "v1_14", feature = "v1_15")),
    all(feature = "v1_9", any(feature = "v1_2", feature = "v1_3", feature = "v1_4", feature = "v1_5", feature = "v1_6", feature = "v1_7", feature = "v1_8", feature = "v1_10", feature = "v1_11", feature = "v1_12", feature = "v1_13", feature = "v1_14", feature = "v1_15")),
    all(feature = "v1_10", any(feature = "v1_2", feature = "v1_3", feature = "v1_4", feature = "v1_5", feature = "v1_6", feature = "v1_7", feature = "v1_8", feature = "v1_9", feature = "v1_11", feature = "v1_12", feature = "v1_13", feature = "v1_14", feature = "v1_15")),
    all(feature = "v1_11", any(feature = "v1_2", feature = "v1_3", feature = "v1_4", feature = "v1_5", feature = "v1_6", feature = "v1_7", feature = "v1_8", feature = "v1_9", feature = "v1_10", feature = "v1_12", feature = "v1_13", feature = "v1_14", feature = "v1_15")),
    all(feature = "v1_12", any(feature = "v1_2", feature = "v1_3", feature = "v1_4", feature = "v1_5", feature = "v1_6", feature = "v1_7", feature = "v1_8", feature = "v1_9", feature = "v1_10", feature = "v1_11", feature = "v1_13", feature = "v1_14", feature = "v1_15")),
    all(feature = "v1_13", any(feature = "v1_2", feature = "v1_3", feature = "v1_4", feature = "v1_5", feature = "v1_6", feature = "v1_7", feature = "v1_8", feature = "v1_9", feature = "v1_10", feature = "v1_11", feature = "v1_12", feature = "v1_14", feature = "v1_15")),
    all(feature = "v1_14", any(feature = "v1_2", feature = "v1_3", feature = "v1_4", feature = "v1_5", feature = "v1_6", feature = "v1_7", feature = "v1_8", feature = "v1_9", feature = "v1_10", feature = "v1_11", feature = "v1_12", feature = "v1_13", feature = "v1_15")),
    all(feature = "v1_15", any(feature = "v1_2", feature = "v1_3", feature = "v1_4", feature = "v1_5", feature = "v1_6", feature = "v1_7", feature = "v1_8", feature = "v1_9", feature = "v1_10", feature = "v1_11", feature = "v1_12", feature = "v1_13", feature = "v1_14"))
))]
compile_error!("enable exactly one contract-version feature of magnetar-component-api (a build targets one version of the contract)");

/// Guest bindings of contract 1.2.0.
#[rustfmt::skip]
#[cfg(all(feature = "v1_2", not(any(feature = "v1_3", feature = "v1_4", feature = "v1_5", feature = "v1_6", feature = "v1_7", feature = "v1_8", feature = "v1_9", feature = "v1_10", feature = "v1_11", feature = "v1_12", feature = "v1_13", feature = "v1_14", feature = "v1_15"))))]
pub mod v1_2 {
    wit_bindgen::generate!({
        path: "wit/1.2.0",
        world: "model-component-graph-producer",
        pub_export_macro: true,
        default_bindings_module: "magnetar_component_api::v1_2",
    });
}

/// Guest bindings of contract 1.3.0.
#[rustfmt::skip]
#[cfg(all(feature = "v1_3", not(any(feature = "v1_2", feature = "v1_4", feature = "v1_5", feature = "v1_6", feature = "v1_7", feature = "v1_8", feature = "v1_9", feature = "v1_10", feature = "v1_11", feature = "v1_12", feature = "v1_13", feature = "v1_14", feature = "v1_15"))))]
pub mod v1_3 {
    wit_bindgen::generate!({
        path: "wit/1.3.0",
        world: "model-component-graph-producer",
        pub_export_macro: true,
        default_bindings_module: "magnetar_component_api::v1_3",
    });
}

/// Guest bindings of contract 1.4.0.
#[rustfmt::skip]
#[cfg(all(feature = "v1_4", not(any(feature = "v1_2", feature = "v1_3", feature = "v1_5", feature = "v1_6", feature = "v1_7", feature = "v1_8", feature = "v1_9", feature = "v1_10", feature = "v1_11", feature = "v1_12", feature = "v1_13", feature = "v1_14", feature = "v1_15"))))]
pub mod v1_4 {
    wit_bindgen::generate!({
        path: "wit/1.4.0",
        world: "model-component-graph-producer",
        pub_export_macro: true,
        default_bindings_module: "magnetar_component_api::v1_4",
    });
}

/// Guest bindings of contract 1.5.0.
#[rustfmt::skip]
#[cfg(all(feature = "v1_5", not(any(feature = "v1_2", feature = "v1_3", feature = "v1_4", feature = "v1_6", feature = "v1_7", feature = "v1_8", feature = "v1_9", feature = "v1_10", feature = "v1_11", feature = "v1_12", feature = "v1_13", feature = "v1_14", feature = "v1_15"))))]
pub mod v1_5 {
    wit_bindgen::generate!({
        path: "wit/1.5.0",
        world: "model-component-graph-producer",
        pub_export_macro: true,
        default_bindings_module: "magnetar_component_api::v1_5",
    });
}

/// Guest bindings of contract 1.6.0.
#[rustfmt::skip]
#[cfg(all(feature = "v1_6", not(any(feature = "v1_2", feature = "v1_3", feature = "v1_4", feature = "v1_5", feature = "v1_7", feature = "v1_8", feature = "v1_9", feature = "v1_10", feature = "v1_11", feature = "v1_12", feature = "v1_13", feature = "v1_14", feature = "v1_15"))))]
pub mod v1_6 {
    wit_bindgen::generate!({
        path: "wit/1.6.0",
        world: "model-component-graph-producer",
        pub_export_macro: true,
        default_bindings_module: "magnetar_component_api::v1_6",
    });
}

/// Guest bindings of contract 1.7.0.
#[rustfmt::skip]
#[cfg(all(feature = "v1_7", not(any(feature = "v1_2", feature = "v1_3", feature = "v1_4", feature = "v1_5", feature = "v1_6", feature = "v1_8", feature = "v1_9", feature = "v1_10", feature = "v1_11", feature = "v1_12", feature = "v1_13", feature = "v1_14", feature = "v1_15"))))]
pub mod v1_7 {
    wit_bindgen::generate!({
        path: "wit/1.7.0",
        world: "model-component-graph-producer",
        pub_export_macro: true,
        default_bindings_module: "magnetar_component_api::v1_7",
    });
}

/// Guest bindings of contract 1.8.0.
#[rustfmt::skip]
#[cfg(all(feature = "v1_8", not(any(feature = "v1_2", feature = "v1_3", feature = "v1_4", feature = "v1_5", feature = "v1_6", feature = "v1_7", feature = "v1_9", feature = "v1_10", feature = "v1_11", feature = "v1_12", feature = "v1_13", feature = "v1_14", feature = "v1_15"))))]
pub mod v1_8 {
    wit_bindgen::generate!({
        path: "wit/1.8.0",
        world: "model-component-graph-producer",
        pub_export_macro: true,
        default_bindings_module: "magnetar_component_api::v1_8",
    });
}

/// Guest bindings of contract 1.9.0.
#[rustfmt::skip]
#[cfg(all(feature = "v1_9", not(any(feature = "v1_2", feature = "v1_3", feature = "v1_4", feature = "v1_5", feature = "v1_6", feature = "v1_7", feature = "v1_8", feature = "v1_10", feature = "v1_11", feature = "v1_12", feature = "v1_13", feature = "v1_14", feature = "v1_15"))))]
pub mod v1_9 {
    wit_bindgen::generate!({
        path: "wit/1.9.0",
        world: "model-component-graph-producer",
        pub_export_macro: true,
        default_bindings_module: "magnetar_component_api::v1_9",
    });
}

/// Guest bindings of contract 1.10.0.
#[rustfmt::skip]
#[cfg(all(feature = "v1_10", not(any(feature = "v1_2", feature = "v1_3", feature = "v1_4", feature = "v1_5", feature = "v1_6", feature = "v1_7", feature = "v1_8", feature = "v1_9", feature = "v1_11", feature = "v1_12", feature = "v1_13", feature = "v1_14", feature = "v1_15"))))]
pub mod v1_10 {
    wit_bindgen::generate!({
        path: "wit/1.10.0",
        world: "model-component-graph-producer",
        pub_export_macro: true,
        default_bindings_module: "magnetar_component_api::v1_10",
    });
}

/// Guest bindings of contract 1.11.0.
#[rustfmt::skip]
#[cfg(all(feature = "v1_11", not(any(feature = "v1_2", feature = "v1_3", feature = "v1_4", feature = "v1_5", feature = "v1_6", feature = "v1_7", feature = "v1_8", feature = "v1_9", feature = "v1_10", feature = "v1_12", feature = "v1_13", feature = "v1_14", feature = "v1_15"))))]
pub mod v1_11 {
    wit_bindgen::generate!({
        path: "wit/1.11.0",
        world: "model-component-graph-producer",
        pub_export_macro: true,
        default_bindings_module: "magnetar_component_api::v1_11",
    });
}

/// Guest bindings of contract 1.12.0.
#[rustfmt::skip]
#[cfg(all(feature = "v1_12", not(any(feature = "v1_2", feature = "v1_3", feature = "v1_4", feature = "v1_5", feature = "v1_6", feature = "v1_7", feature = "v1_8", feature = "v1_9", feature = "v1_10", feature = "v1_11", feature = "v1_13", feature = "v1_14", feature = "v1_15"))))]
pub mod v1_12 {
    wit_bindgen::generate!({
        path: "wit/1.12.0",
        world: "model-component-graph-producer",
        pub_export_macro: true,
        default_bindings_module: "magnetar_component_api::v1_12",
    });
}

/// Guest bindings of contract 1.13.0.
#[rustfmt::skip]
#[cfg(all(feature = "v1_13", not(any(feature = "v1_2", feature = "v1_3", feature = "v1_4", feature = "v1_5", feature = "v1_6", feature = "v1_7", feature = "v1_8", feature = "v1_9", feature = "v1_10", feature = "v1_11", feature = "v1_12", feature = "v1_14", feature = "v1_15"))))]
pub mod v1_13 {
    wit_bindgen::generate!({
        path: "wit/1.13.0",
        world: "model-component-graph-producer",
        pub_export_macro: true,
        default_bindings_module: "magnetar_component_api::v1_13",
    });
}

/// Guest bindings of contract 1.14.0.
#[rustfmt::skip]
#[cfg(all(feature = "v1_14", not(any(feature = "v1_2", feature = "v1_3", feature = "v1_4", feature = "v1_5", feature = "v1_6", feature = "v1_7", feature = "v1_8", feature = "v1_9", feature = "v1_10", feature = "v1_11", feature = "v1_12", feature = "v1_13", feature = "v1_15"))))]
pub mod v1_14 {
    wit_bindgen::generate!({
        path: "wit/1.14.0",
        world: "model-component-graph-producer",
        pub_export_macro: true,
        default_bindings_module: "magnetar_component_api::v1_14",
    });
}

/// Guest bindings of contract 1.15.0.
#[rustfmt::skip]
#[cfg(all(feature = "v1_15", not(any(feature = "v1_2", feature = "v1_3", feature = "v1_4", feature = "v1_5", feature = "v1_6", feature = "v1_7", feature = "v1_8", feature = "v1_9", feature = "v1_10", feature = "v1_11", feature = "v1_12", feature = "v1_13", feature = "v1_14"))))]
pub mod v1_15 {
    wit_bindgen::generate!({
        path: "wit/1.15.0",
        world: "model-component-graph-producer",
        pub_export_macro: true,
        default_bindings_module: "magnetar_component_api::v1_15",
    });
}
