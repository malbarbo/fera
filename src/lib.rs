#![cfg_attr(test, feature(test))]

#[cfg(test)]
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

pub mod prelude {
    pub use graph::*;
    pub use subgraph::*;
    pub use builder::WithBuilder;
    pub use static_::*;
    pub use iter::*;
}
