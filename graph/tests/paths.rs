// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate fera_graph;
extern crate rand;

use fera_graph::algs::Paths;
use fera_graph::choose::Choose;
use fera_graph::prelude::*;
use rand::prelude::*;

#[test]
fn is_walk() {
    let g = CompleteGraph::new(10);
    let mut rng = SmallRng::from_entropy();
    for _ in 0..100 {
        let x = rng.gen_range(0, 100);
        assert!(g.is_walk(g.random_walk(&mut rng).take(x)));
    }
}

#[test]
fn path() {
    let n = 10;
    let g = CompleteGraph::new(n);
    for u in 0..n {
        for v in 0..n {
            if let Some(path) = g.find_path(u, v) {
                assert!(g.is_path(&path));
                assert_eq!(u, g.source(*path.first().unwrap()));
                assert_eq!(v, g.target(*path.last().unwrap()));
            } else {
                assert_eq!(u, v);
            }
        }
    }
}
