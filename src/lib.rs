#![cfg_attr(all(feature = "nightly", test), feature(test))]
#![cfg_attr(feature="clippy", allow(inline_always))]

#[cfg(all(feature = "nightly", test))]
extern crate test;

#[cfg(test)]
extern crate itertools;

extern crate num;
extern crate rand;

#[macro_use]
extern crate fera;

#[cfg(test)]
#[macro_use]
pub mod tests;

#[macro_use]
pub mod graph;
pub mod common;
#[macro_use]
pub mod params;

#[macro_use]
pub mod builder;

pub mod adjset;
pub mod graph_ref;
pub mod subgraph;
pub mod complete;
pub mod static_;
pub mod choose;
pub mod kruskal;
pub mod path;
pub mod props;
pub mod traverse;
pub mod unionfind;

pub mod arrayprop;
pub mod delegateprop;
pub mod fnprop;
pub mod hashmapprop;

pub mod prelude {
    pub use graph::*;
    pub use subgraph::{Subgraph, WithSubgraph};
    pub use complete::CompleteGraph;
    pub use builder::{Builder, WithBuilder};
    pub use static_::*;
}
