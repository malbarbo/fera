// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![feature(test)]

extern crate fera_graph;
extern crate rand;
extern crate test;

use fera_graph::algs::Paths;
use fera_graph::prelude::*;
use rand::{FromEntropy, XorShiftRng};
use test::Bencher;

fn find_path_n(b: &mut Bencher, n: usize) {
    let mut rng = XorShiftRng::from_entropy();
    let g = StaticGraph::new_random_tree(n, &mut rng);
    b.iter(|| {
        for (u, v) in g.edges_ends() {
            assert!(g.find_path(v, u).is_some());
        }
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
