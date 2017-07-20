//! Create adapted graphs without modifying the wrapped graph.

mod subgraph;
mod spanning_subgraph;

pub use self::subgraph::{Subgraph, WithSubgraph};
pub use self::spanning_subgraph::{SpanningSubgraph};

// TODO: add Reversed
// TODO: add Filtered
