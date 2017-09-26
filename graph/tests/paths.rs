extern crate fera_graph;
extern crate rand;

use fera_graph::prelude::*;
use fera_graph::algs::Paths;
use fera_graph::choose::Choose;
use rand::Rng;

#[test]
fn is_walk() {
    let g = CompleteGraph::new(10);
    let mut rng = rand::weak_rng();
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