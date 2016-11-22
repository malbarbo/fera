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

// graphs
pub mod adjset;
pub mod common;
pub mod complete;
pub mod graph_ref;
pub mod static_;
pub mod subgraph;

// props
pub mod arrayprop;
pub mod delegateprop;
pub mod fnprop;
pub mod hashmapprop;

// algorithms
pub mod kruskal;
pub mod path;
pub mod trees;
pub mod cycles;
pub mod components;

// others
pub mod choose;
pub mod unionfind;

pub mod prelude {
    pub use graph::*;
    pub use subgraph::{Subgraph, WithSubgraph};
    pub use complete::CompleteGraph;
    pub use builder::{Builder, WithBuilder};
    pub use static_::*;
}
