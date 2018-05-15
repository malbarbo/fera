// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![feature(test)]

extern crate fera_graph;
extern crate rand;
extern crate test;

use fera_graph::prelude::*;
use fera_graph::traverse::*;
use rand::XorShiftRng;
use test::Bencher;

fn bfs(b: &mut Bencher, g: &StaticGraph) {
    b.iter(|| {
        g.bfs(OnDiscoverTreeEdge(|_| Control::Continue)).run();
    });
}

#[bench]
fn bfs_complete_graph(b: &mut Bencher) {
    let g = StaticGraph::new_complete(100);
    bfs(b, &g);
}

#[bench]
fn bfs_tree(b: &mut Bencher) {
    let g = StaticGraph::new_random_tree(100, XorShiftRng::new_unseeded());
    bfs(b, &g);
}

fn dfs(b: &mut Bencher, g: &StaticGraph) {
    b.iter(|| {
        g.dfs(OnDiscoverTreeEdge(|_| Control::Continue)).run();
    });
}

#[bench]
fn dfs_complete_graph(b: &mut Bencher) {
    let g = StaticGraph::new_complete(100);
    dfs(b, &g);
}

#[bench]
fn dfs_tree(b: &mut Bencher) {
    let g = StaticGraph::new_random_tree(100, XorShiftRng::new_unseeded());
    dfs(b, &g);
}

fn recursive_dfs(b: &mut Bencher, g: &StaticGraph) {
    b.iter(|| {
        g.recursive_dfs(OnDiscoverTreeEdge(|_| Control::Continue))
            .run();
    });
}

#[bench]
fn recursive_dfs_complete_graph(b: &mut Bencher) {
    let g = StaticGraph::new_complete(100);
    recursive_dfs(b, &g);
}

#[bench]
fn recursive_dfs_tree(b: &mut Bencher) {
    let g = StaticGraph::new_random_tree(100, XorShiftRng::new_unseeded());
    recursive_dfs(b, &g);
}
