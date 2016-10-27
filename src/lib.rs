#![cfg_attr(all(feature = "nightly", test), feature(test))]
#![cfg_attr(feature="clippy", allow(inline_always))]

#[cfg(all(feature = "nightly", test))]
extern crate test;

extern crate num;
extern crate rand;
#[cfg(test)]
extern crate itertools;

#[macro_use]
extern crate fera;

#[cfg(test)]
#[macro_use]
pub mod tests;

// TODO: Create a DiGraph trait and implementations
pub mod graph;

#[macro_use]
pub mod builder;

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
pub mod fnprop;
pub mod hashmapprop;
pub mod arrayprop;

pub mod common;
pub mod delegateprop;

pub mod prelude {
    pub use graph::*;
    pub use subgraph::{Subgraph, WithSubgraph};
    pub use complete::CompleteGraph;
    pub use builder::{Builder, WithBuilder};
    pub use static_::*;
}
