#![cfg_attr(feature = "cargo-clippy", allow(inline_always))]

#[cfg(test)]
extern crate itertools;

extern crate fera_fun;
extern crate fera_optional;
extern crate fera_unionfind;
extern crate fnv;
extern crate num;
extern crate rand;

#[cfg(test)]
#[macro_use]
pub mod tests;

// basic
#[macro_use]
pub mod builder;
#[macro_use]
pub mod graph;
#[macro_use]
pub mod params;
pub mod traverse;

mod graphs;
pub use graphs::*;

pub mod props;

pub mod extensions;

// algorithms
pub mod components;
pub mod cycles;
pub mod kruskal;
pub mod paths;
pub mod trees;

// others
pub mod choose;
pub mod unionfind;

pub mod prelude {
    pub use fera_optional::Optional;
    pub use graph::*;
    pub use props::{PropGet, BasicProps, VertexPropGet, VertexProp, VertexPropMut, VertexPropMutNew,
                    BasicVertexProps, WithVertexProp, EdgePropGet, EdgeProp, EdgePropMut,
                    EdgePropMutNew, BasicEdgeProps, WithEdgeProp, PropIndexMut};
    pub use builder::{Builder, WithBuilder};
    pub use complete::{CompleteGraph, CompleteDiGraph};
    pub use static_::StaticGraph;
    pub use subgraph::{Subgraph, WithSubgraph};
    pub use extensions::{GraphsIteratorExt, GraphsSliceExt, GraphsVecExt};
}