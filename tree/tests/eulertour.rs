#[macro_use]
extern crate quickcheck;
extern crate fera_tree;

use fera_tree::eulertour::{EulerTourTree, NestedSeq, Seq, Sequence};
use fera_tree::{check_dynamic_tree, check_dynamic_tree_incremental, DynamicTree};

quickcheck! {
    fn quickcheck_incremental(edges: Vec<(u8, u8)>) -> bool {
        incremental::<Seq>(edges);
        true
    }

    fn quickcheck(edges: Vec<(u8, u8)>) -> bool {
        check::<Seq>(edges);
        true
    }

    fn quickcheck_incremental_nested(edges: Vec<(u8, u8)>) -> bool {
        incremental::<NestedSeq>(edges);
        true
    }

    fn quickcheck_nested(edges: Vec<(u8, u8)>) -> bool {
        check::<NestedSeq>(edges);
        true
    }
}

fn incremental<S: Sequence>(edges: Vec<(u8, u8)>) {
    let n = 25;
    check_dynamic_tree_incremental(edges, EulerTourTree::<S>::new(n), n);
}

fn check<S: Sequence>(edges: Vec<(u8, u8)>) {
    let n = 25;
    check_dynamic_tree(edges, EulerTourTree::<S>::new(n), n);
}
