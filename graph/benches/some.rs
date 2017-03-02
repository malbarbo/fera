#![feature(test)]

extern crate fera_graph;
extern crate rand;
extern crate test;

use fera_graph::prelude::*;
use fera_graph::cycles::Cycles;
use fera_graph::components::Components;

use test::Bencher;

fn bench_is_acyclic(b: &mut Bencher, n: usize) {
    let mut rng = rand::XorShiftRng::new_unseeded();
    let g = StaticGraph::new_random_tree(n, &mut rng);
    b.iter(|| {
        assert!(g.is_acyclic());
    })
}

#[bench]
fn bench_is_acyclic_10(b: &mut Bencher) {
    bench_is_acyclic(b, 10);
}

#[bench]
fn bench_is_acyclic_100(b: &mut Bencher) {
    bench_is_acyclic(b, 100);
}

#[bench]
fn bench_is_acyclic_1000(b: &mut Bencher) {
    bench_is_acyclic(b, 1000);
}

fn bench_is_connected(b: &mut Bencher, n: usize) {
    let mut rng = rand::XorShiftRng::new_unseeded();
    let g = StaticGraph::new_random_tree(n, &mut rng);
    b.iter(|| {
        assert!(g.is_connected());
    })
}

#[bench]
fn bench_is_connected_10(b: &mut Bencher) {
    bench_is_connected(b, 10);
}

#[bench]
fn bench_is_connected_100(b: &mut Bencher) {
    bench_is_connected(b, 100);
}

#[bench]
fn bench_is_connected_1000(b: &mut Bencher) {
    bench_is_connected(b, 1000);
}
