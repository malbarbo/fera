// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[cfg(feature = "quickcheck")]
#[macro_use]
extern crate quickcheck;
extern crate fera_graph;

#[cfg(feature = "quickcheck")]
mod quickchecks {
    use fera_graph::arbitrary::Gn;
    use fera_graph::prelude::*;
    use fera_graph::traverse::{Dfs, OnTraverseEvent, RecursiveDfs};

    quickcheck! {
        fn dfs(x: Gn<StaticGraph>) -> bool {
            let Gn(g) = x;

            let mut v1 = vec![];
            g.recursive_dfs(OnTraverseEvent(|evt| v1.push(evt))).run();

            let mut v2 = vec![];
            g.dfs(OnTraverseEvent(|evt| v2.push(evt))).run();

            v1 == v2
        }
    }
}
