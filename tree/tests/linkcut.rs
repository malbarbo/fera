#[macro_use]
extern crate quickcheck;
extern crate fera_tree;

use fera_tree::linkcut::{LinkCutTree, Node, UnsafeCellNode};

quickcheck! {
    fn quickcheck(edges: Vec<(u8, u8)>) -> bool {
        check(edges);
        true
    }
}

fn check(edges: Vec<(u8, u8)>) {
    let n = 25;
    let mut expected = NaiveConnectivity::new(n);
    let mut actual = LinkCutTree::new(n);

    for (u, v) in edges {
        let u = u as usize % n;
        let v = v as usize % n;
        if expected.has_edge(u, v) {
            assert!(expected.is_connected(u, v));

            assert!(actual.is_connected(u, v));

            expected.cut(u, v);
            assert!(!expected.is_connected(u, v));

            actual.cut(u, v);
            assert!(!actual.is_connected(u, v));
        } else if !expected.is_connected(u, v) {
            expected.link(u, v);
            assert!(expected.is_connected(u, v));

            actual.link(u, v);
            assert!(actual.is_connected(u, v));
        }

        for i in 0..n {
            for j in (i + 1)..n {
                assert_eq!(
                    expected.is_connected(i, j),
                    actual.is_connected(i, j),
                    "{} {}",
                    i,
                    j
                );
            }
        }
    }
}

#[test]
fn basic() {
    check(vec![(0, 1), (0, 1)]);
    check(vec![(0, 1), (1, 0)]);
    check(vec![(0, 1), (0, 2)]);
    check(vec![(3, 2), (1, 2), (1, 0), (3, 4)]);
    check(vec![(0, 2), (1, 2), (1, 3)]);
    check(vec![(0, 2), (1, 2), (1, 3)]);
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

struct NaiveConnectivity {
    parent: Vec<Option<usize>>,
}

impl NaiveConnectivity {
    pub fn new(n: usize) -> Self {
        Self { parent: vec![None; n] }
    }

    pub fn link(&mut self, x: usize, y: usize) {
        assert!(!self.is_connected(x, y), "The edge ({}, {}) exist", x, y);
        self.make_root(y);
        self.parent[y] = Some(x);
    }

    pub fn cut(&mut self, x: usize, y: usize) {
        if self.parent[x] == Some(y) {
            self.parent[x] = None;
        } else if self.parent[y] == Some(x) {
            self.parent[y] = None;
        } else {
            panic!("The edge ({}, {}) does not exist", x, y);
        }
    }

    pub fn has_edge(&self, x: usize, y: usize) -> bool {
        self.parent[x] == Some(y) || self.parent[y] == Some(x)
    }

    pub fn is_connected(&self, x: usize, y: usize) -> bool {
        self.find_root(x) == self.find_root(y)
    }

    fn find_root(&self, mut x: usize) -> usize {
        while let Some(y) = self.parent[x] {
            x = y;
        }
        x
    }

    fn make_root(&mut self, x: usize) {
        if let Some(y) = self.parent[x] {
            self.make_root(y);
            self.parent[y] = Some(x);
            self.parent[x] = None;
        }
        assert_eq!(None, self.parent[x]);
    }
}
