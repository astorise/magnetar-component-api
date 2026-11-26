//! The smallest Model Component: a graph that looks a token up in an
//! embedding table and returns it.
//!
//! It is a template, not a model. It shows the whole surface a Component
//! implements: the four graph builders of the contract, each of which
//! describes a graph through the Runtime's `graph-builder` capability and
//! returns the id the Runtime gave the finished graph.

use magnetar_component_api::v1_14::{
    Guest, export,
    magnetar::model_component_graph::graph_builder::{self as gb, GraphPhase, TensorShape},
};

/// The width of one embedding row. A real Component reads it from the
/// model's configuration (`model-config`) instead.
const HIDDEN: u64 = 8;

struct Component;

fn embedding_graph(phase: GraphPhase, sequence_length: u64, position_offset: u64) -> String {
    gb::begin_graph(phase, sequence_length, position_offset);
    let token_ids = gb::declare_input(
        "token_ids",
        &TensorShape {
            dimensions: vec![sequence_length],
        },
    );
    let table = gb::weight_edge("token_embedding");
    let hidden = gb::add_node(
        "embedding",
        "embedding",
        "Tensor",
        &[],
        &[token_ids, table],
        &TensorShape {
            dimensions: vec![sequence_length, HIDDEN],
        },
        None,
    );
    gb::finish_graph(&hidden)
}

impl Guest for Component {
    fn build_prefill_graph(sequence_length: u64) -> String {
        embedding_graph(GraphPhase::Prefill, sequence_length, 0)
    }

    fn build_decode_graph(cached_token_count: u64) -> String {
        embedding_graph(GraphPhase::Decode, 1, cached_token_count)
    }

    fn build_prefill_graph_segment(_: u64, _: u32, _: u32) -> String {
        panic!("minimal-component has no decoder layers to segment")
    }

    fn build_decode_graph_segment(_: u64, _: u32, _: u32) -> String {
        panic!("minimal-component has no decoder layers to segment")
    }
}

export!(Component);
