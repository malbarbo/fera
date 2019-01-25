// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![feature(test)]

extern crate fera_graph;
extern crate rand;
extern crate test;

use fera_graph::algs::{Components, Cycles};
use fera_graph::prelude::*;
use rand::prelude::*;

use test::Bencher;

fn bench_is_acyclic(b: &mut Bencher, n: usize) {
    let mut rng = SmallRng::from_entropy();
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
    let mut rng = SmallRng::from_entropy();
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
