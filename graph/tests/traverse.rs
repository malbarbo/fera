extern crate fera_graph;

#[macro_use]
extern crate quickcheck;

use fera_graph::prelude::*;
use fera_graph::traverse::{Dfs, RecursiveDfs, OnTraverseEvent};
use fera_graph::arbitrary::Gn;

quickcheck! {
    fn quickcheck_dfs(x: Gn<StaticGraph>) -> bool {
        let Gn(g) = x;

        let mut v1 = vec![];
        g.recursive_dfs(OnTraverseEvent(|evt| v1.push(evt))).run();

        let mut v2 = vec![];
        g.dfs(OnTraverseEvent(|evt| v2.push(evt))).run();

        v1 == v2
    }
}
