#![feature(test)]

extern crate fera_graphs as graphs;
extern crate rand;
extern crate test;

use graphs::prelude::*;
use graphs::paths::Paths;
use rand::XorShiftRng;
use test::Bencher;

fn find_path_n(b: &mut Bencher, n: usize) {
    let mut rng = XorShiftRng::new_unseeded();
    let g = StaticGraph::random_tree(n, &mut rng);
    b.iter(|| for e in g.edges() {
        let (u, v) = g.ends(e);
        assert!(g.find_path(v, u).is_some());
    })
}

#[bench]
fn find_path_10(b: &mut Bencher) {
    find_path_n(b, 10);
}

#[bench]
fn find_path_100(b: &mut Bencher) {
    find_path_n(b, 100);
}

#[bench]
fn find_path_1000(b: &mut Bencher) {
    find_path_n(b, 1000);
}
