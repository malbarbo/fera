// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Create adapted graphs without modifying the wrapped graph.

mod subgraph;
mod spanning_subgraph;

pub use self::subgraph::{Subgraph, WithSubgraph};
pub use self::spanning_subgraph::{SpanningSubgraph};

// TODO: add Reversed
// TODO: add Filtered
