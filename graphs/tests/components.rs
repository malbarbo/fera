extern crate fera_graphs;
#[macro_use]
extern crate quickcheck;

use fera_graphs::prelude::*;
use fera_graphs::components::{Components, cut_vertices_naive, cut_edges_naive};

use std::cmp::max;

fn sorted<T: Ord>(mut v: Vec<T>) -> Vec<T> {
    v.sort();
    v
}

// TODO: Implement Arbitrary for Graph
quickcheck! {
    fn quickcheck_cut_vertices(edges: Vec<(u8, u8)>) -> bool {
        let n = edges.iter().map(|x| max(x.0, x.1)).max().unwrap_or(0) as usize + 1;
        let mut builder = StaticGraph::builder(n, edges.len());
        for (u, v) in edges.clone() {
            if u == v {
                continue
            }
            builder.add_edge(u as usize, v as usize);
        }
        let g = builder.finalize();
        let expect = sorted(cut_vertices_naive(&g));
        let actual = sorted(g.cut_vertices());
        expect == actual
    }

    fn quickcheck_cut_edges(edges: Vec<(u8, u8)>) -> bool {
        let n = edges.iter().map(|x| max(x.0, x.1)).max().unwrap_or(0) as usize + 1;
        let mut builder = StaticGraph::builder(n, edges.len());
        for (u, v) in edges.clone() {
            if u == v {
                continue
            }
            builder.add_edge(u as usize, v as usize);
        }
        let g = builder.finalize();
        let expect = sorted(cut_edges_naive(&g));
        let actual = sorted(g.cut_edges());
        expect == actual
    }
}
