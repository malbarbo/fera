#[macro_use]
extern crate quickcheck;
extern crate fera_tree;

use fera_tree::eulertour::{EulerTourTree, Seq};
use fera_tree::{check_dynamic_tree, check_dynamic_tree_incremental, DynamicTree};

#[test]
fn basic() {
    let mut dc = EulerTourTree::<Seq>::new(3);

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

    incremental(vec![(0, 1), (0, 2)]);
    incremental(vec![(1, 2), (0, 3), (0, 1)]);
    check(vec![(2, 0), (1, 0), (0, 1)]);
    check(vec![(0, 2), (4, 3), (3, 2), (3, 2), (4, 3)]);
    check(vec![(2, 3), (2, 4), (0, 3), (4, 2), (2, 3)]);
}

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
    check_dynamic_tree_incremental(edges, EulerTourTree::<Seq>::new(n), n);
}

fn check(edges: Vec<(u8, u8)>) {
    let n = 25;
    check_dynamic_tree(edges, EulerTourTree::<Seq>::new(n), n);
}
