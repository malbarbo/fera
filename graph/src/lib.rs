//! Graph data structures and algorithms.
#![cfg_attr(feature = "cargo-clippy", allow(inline_always))]

#[cfg(test)]
extern crate itertools;

extern crate quickcheck;

extern crate fera_fun;
extern crate fera_optional;
extern crate fera_unionfind;
extern crate fnv;
extern crate num_traits;
extern crate rand;

#[cfg(test)]
#[macro_use]
pub mod tests;

// basic
#[macro_use]
pub mod builder;

#[macro_use]
pub mod params;

pub mod algs;
pub mod graphs;
pub mod props;
pub mod traverse;

// others
pub mod arbitrary;
pub mod choose;
pub mod ext;
pub mod unionfind;

mod fun;
pub use fun::*;

/// The fera graph prelude.
pub mod prelude {
    pub use graphs::adjset::{AdjSetGraph, AdjSetDigraph};
    pub use graphs::complete::{CompleteGraph, CompleteDigraph};
    pub use graphs::static_::{StaticGraph, StaticDigraph};
    pub use graphs::subgraph::{Subgraph, WithSubgraph};
    pub use graphs::spanning_subgraph::SpanningSubgraph;
    pub use graphs::{
        Adjacency,
        AdjacencyDigraph,
        AdjacencyGraph,
        DefaultEdgePropMut,
        DefaultVertexPropMut,
        Digraph,
        Directed,
        Edge,
        EdgeIndexProp,
        EdgeIter,
        EdgeKind,
        EdgeList,
        EdgeTypes,
        Graph,
        GraphItem,
        Incidence,
        IncidenceDigraph,
        IncidenceGraph,
        Mixed,
        OptionEdge,
        OptionVertex,
        Orientation,
        OutEdgeIter,
        OutNeighborIter,
        Undirected,
        UniformEdgeKind,
        Vertex,
        VertexIndexProp,
        VertexIter,
        VertexList,
        VertexTypes,
        WithEdge,
        WithVertex,
    };
    pub use props::{
        BasicEdgeProps,
        BasicProps,
        BasicVertexProps,
        EdgeProp,
        EdgePropGet,
        EdgePropMut,
        EdgePropMutNew,
        PropGet,
        PropIndexMut,
        VertexProp,
        VertexPropGet,
        VertexPropMut,
        VertexPropMutNew,
        WithEdgeIndexProp,
        WithEdgeProp,
        WithVertexIndexProp,
        WithVertexProp,
    };
    pub use builder::{Builder, WithBuilder};
    pub use ext::{GraphsSliceExt, GraphsVecExt};
    pub use fera_optional::Optional;
}
