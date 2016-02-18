#![cfg_attr(all(feature = "unstable", test), feature(test))]

#[cfg(all(feature = "unstable", test))]
extern crate test;
extern crate rand;

#[macro_use]
extern crate ds;

#[cfg(test)]
#[macro_use]
pub mod tests;

// TODO: Create a CompleteGraph implementation of Graph
// TODO: Create a DiGraph trait and implementations
pub mod graph;
pub mod subgraph;
pub mod static_;
pub mod builder;
pub mod choose;
pub mod iter;
pub mod kruskal;
pub mod path;
pub mod props;
pub mod traverse;
pub mod unionfind;
pub mod vecprop;

pub mod prelude {
    pub use graph::*;
    pub use subgraph::{Subgraph, WithSubgraph};
    pub use builder::WithBuilder;
    pub use static_::*;
    pub use iter::*;
}
