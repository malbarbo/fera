#[macro_use]
extern crate quickcheck;
extern crate fera_tree;

use fera_tree::linkcut::LinkCutTree;
use fera_tree::{check_dynamic_tree, check_dynamic_tree_incremental, DynamicTree};

quickcheck! {
    fn quickcheck_incremental(edges: Vec<(u8, u8)>) -> bool {
        incremental(edges);
        true
    }

    fn quickcheck(edges: Vec<(u8, u8)>) -> bool {
        check(edges);
        true
    }
}

fn incremental(edges: Vec<(u8, u8)>) {
    let n = 25;
    check_dynamic_tree_incremental(edges, LinkCutTree::new(n), n);
}

fn check(edges: Vec<(u8, u8)>) {
    let n = 25;
    check_dynamic_tree(edges, LinkCutTree::new(n), n);
}
