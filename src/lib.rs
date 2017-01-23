#![cfg_attr(all(feature = "nightly", test), feature(test))]
#![cfg_attr(feature="clippy", allow(inline_always))]

#[cfg(all(feature = "nightly", test))]
extern crate test;

#[cfg(test)]
#[macro_use]
extern crate quickcheck;

#[cfg(test)]
extern crate itertools;

extern crate num;
extern crate rand;

#[macro_use]
extern crate fera;

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
    pub use fera::optional::Optional;
    pub use graph::*;
    pub use props::{PropGet, BasicProps, VertexPropGet, VertexProp, VertexPropMut,
                    VertexPropMutNew, BasicVertexProps, WithVertexProp, EdgePropGet, EdgeProp,
                    EdgePropMut, EdgePropMutNew, BasicEdgeProps, WithEdgeProp, PropIndexMut};
    pub use builder::{Builder, WithBuilder};
    pub use complete::{CompleteGraph, CompleteDiGraph};
    pub use static_::StaticGraph;
    pub use subgraph::{Subgraph, WithSubgraph};
    pub use extensions::{GraphsIteratorExt, GraphsSliceExt, GraphsVecExt};
}
