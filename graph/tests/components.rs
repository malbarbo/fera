// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate fera_graph;
#[macro_use]
extern crate quickcheck;

use fera_graph::prelude::*;
use fera_graph::algs::Components;
use fera_graph::algs::components::{cut_vertices_naive, cut_edges_naive};
use fera_graph::arbitrary::Gn;

fn sorted<T: Ord>(mut v: Vec<T>) -> Vec<T> {
    v.sort();
    v
}

quickcheck! {
    fn quickcheck_cut_vertices(g: Gn<StaticGraph>) -> bool {
        let g = g.0;
        if g.num_vertices() > 20 {
            return true
        }
        let expect = sorted(cut_vertices_naive(&g));
        let actual = sorted(g.cut_vertices());
        expect == actual
    }

    fn quickcheck_cut_edges(g: Gn<StaticGraph>) -> bool {
        let g = g.0;
        if g.num_vertices() > 20 {
            return true
        }
        let expect = sorted(cut_edges_naive(&g));
        let actual = sorted(g.cut_edges());
        expect == actual
    }
}
