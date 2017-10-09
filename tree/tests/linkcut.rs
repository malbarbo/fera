#[macro_use]
extern crate quickcheck;
extern crate fera_tree;

use fera_tree::linkcut::{LinkCutTree, Node, UnsafeCellNode};
use fera_tree::{check_dynamic_tree, check_dynamic_tree_incremental};

#[test]
fn basic() {
    check(vec![(0, 0)]);
    check(vec![(0, 1), (0, 1)]);
    check(vec![(0, 1), (1, 0)]);
    check(vec![(0, 1), (0, 2)]);
    check(vec![(3, 2), (1, 2), (1, 0), (3, 4)]);
    check(vec![(0, 2), (1, 2), (1, 3)]);
    check(vec![(0, 2), (1, 2), (1, 3)]);
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
    check_dynamic_tree_incremental(edges, LinkCutTree::new(n), n);
}

fn check(edges: Vec<(u8, u8)>) {
    let n = 25;
    check_dynamic_tree(edges, LinkCutTree::new(n), n);
}

#[test]
fn rotate() {
    // left rotation
    //     pp               pp
    //      \                \
    //       p                s
    //      / \      ->      / \
    //     s  p.r         s.l   p
    //    / \                  / \
    // s.l  s.r              s.r  p.r
    let nodes: Vec<_> = (0..6).map(UnsafeCellNode::new).collect();
    let pp = &nodes[0];
    let p = &nodes[1];
    let pr = &nodes[2];
    let s = &nodes[3];
    let sl = &nodes[4];
    let sr = &nodes[5];

    pp.connect_right(Some(p));
    p.connect_left(Some(s));
    p.connect_right(Some(pr));
    s.connect_left(Some(sl));
    s.connect_right(Some(sr));

    s.rotate();

    assert_node(pp, None, None, Some(s));
    assert_node(s, Some(pp), Some(sl), Some(p));
    assert_node(sl, Some(s), None, None);
    assert_node(p, Some(s), Some(sr), Some(pr));
    assert_node(sr, Some(p), None, None);
    assert_node(pr, Some(p), None, None);
}

fn assert_node<N: Node>(x: &N, parent: Option<&N>, left: Option<&N>, right: Option<&N>) {
    assert_eq!(parent, x.parent());
    assert_eq!(left, x.left());
    assert_eq!(right, x.right());
}
