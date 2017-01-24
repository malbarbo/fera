#![feature(test)]

extern crate fera_graphs as graphs;
extern crate rand;
extern crate test;

use graphs::prelude::*;
use graphs::traverse::*;
use rand::XorShiftRng;
use test::Bencher;

fn bfs(b: &mut Bencher, g: &StaticGraph) {
    b.iter(|| {
        g.bfs(OnDiscoverTreeEdge(|_| Control::Continue));
    });
}

#[bench]
fn bfs_complete_graph(b: &mut Bencher) {
    let g = StaticGraph::complete(100);
    bfs(b, &g);
}

#[bench]
fn bfs_tree(b: &mut Bencher) {
    let g = StaticGraph::random_tree(100, XorShiftRng::new_unseeded());
    bfs(b, &g);
}


fn dfs(b: &mut Bencher, g: &StaticGraph) {
    b.iter(|| {
        g.dfs(OnDiscoverTreeEdge(|_| Control::Continue));
    });
}

#[bench]
fn dfs_complete_graph(b: &mut Bencher) {
    let g = StaticGraph::complete(100);
    dfs(b, &g);
}

#[bench]
fn dfs_tree(b: &mut Bencher) {
    let g = StaticGraph::random_tree(100, XorShiftRng::new_unseeded());
    dfs(b, &g);
}


fn recursive_dfs(b: &mut Bencher, g: &StaticGraph) {
    b.iter(|| {
        g.recursive_dfs(OnDiscoverTreeEdge(|_| Control::Continue));
    });
}

#[bench]
fn recursive_dfs_complete_graph(b: &mut Bencher) {
    let g = StaticGraph::complete(100);
    recursive_dfs(b, &g);
}

#[bench]
fn recursive_dfs_tree(b: &mut Bencher) {
    let g = StaticGraph::random_tree(100, XorShiftRng::new_unseeded());
    recursive_dfs(b, &g);
}
