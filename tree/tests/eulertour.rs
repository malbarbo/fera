#[macro_use]
extern crate quickcheck;
extern crate fera_tree;

use fera_tree::eulertour::{EulerTourTree, NestedSeq, Seq, Sequence};
use fera_tree::{check_dynamic_tree, check_dynamic_tree_incremental, DynamicTree};


#[test]
fn basic_seq() {
    basic::<Seq>();
}

#[test]
fn basic_nested_seq() {
    basic::<NestedSeq>();
}

fn basic<S: Sequence>() {
    let mut dc = EulerTourTree::<NestedSeq>::new(3);

    assert!(!dc.is_connected(0, 1));
    assert!(!dc.is_connected(0, 2));
    assert!(!dc.is_connected(1, 2));

    let e = dc.link(0, 1).unwrap();
    assert!(dc.is_connected(0, 1));
    assert!(!dc.is_connected(0, 2));
    assert!(!dc.is_connected(1, 2));

    let f = dc.link(0, 2).unwrap();
    assert!(dc.is_connected(0, 1));
    assert!(dc.is_connected(0, 2));
    assert!(dc.is_connected(1, 2));

    dc.cut(e);
    assert!(!dc.is_connected(0, 1));
    assert!(dc.is_connected(0, 2));
    assert!(!dc.is_connected(1, 2));

    dc.cut(f);
    assert!(!dc.is_connected(0, 1));
    assert!(!dc.is_connected(0, 2));
    assert!(!dc.is_connected(1, 2));

    incremental::<S>(vec![(0, 1), (0, 2)]);
    incremental::<S>(vec![(1, 2), (0, 3), (0, 1)]);
    check::<S>(vec![(2, 0), (1, 0), (0, 1)]);
    check::<S>(vec![(0, 2), (4, 3), (3, 2), (3, 2), (4, 3)]);
    check::<S>(vec![(2, 3), (2, 4), (0, 3), (4, 2), (2, 3)]);
    check::<S>(vec![(2, 4), (4, 0), (2, 3), (4, 2)]);
}

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
    let n = 5;
    check_dynamic_tree_incremental(edges, EulerTourTree::<S>::new(n), n);
}

fn check<S: Sequence>(edges: Vec<(u8, u8)>) {
    let n = 5;
    check_dynamic_tree(edges, EulerTourTree::<S>::new(n), n);
}
