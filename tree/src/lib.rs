#![feature(slice_rotate)]

use std::collections::HashMap;

pub mod eulertour;
pub use eulertour::EulerTourTree;

pub mod linkcut;
pub use linkcut::LinkCutTree;

pub mod parentpointer;
pub use parentpointer::ParentPointerTree;

pub trait DynamicTree {
    type Edge;

    fn is_connected(&self, u: usize, v: usize) -> bool;

    fn link(&mut self, u: usize, v: usize) -> Option<Self::Edge>;

    fn cut(&mut self, e: Self::Edge);
}

pub fn check_dynamic_tree_incremental<D: DynamicTree>(
    edges: Vec<(u8, u8)>,
    mut actual: D,
    n: usize,
) {
    let mut expected = ParentPointerTree::new(n);

    for (u, v) in edges {
        let u = u as usize % n;
        let v = v as usize % n;
        if expected.is_connected(u, v) {
            assert!(expected.is_connected(u, v));
            assert!(actual.is_connected(u, v));
        } else {
            expected.link(u, v);
            assert!(expected.is_connected(u, v));
            actual.link(u, v).unwrap();
            assert!(actual.is_connected(u, v));
        }

        _check_dynamic_tree(&expected, &actual, n);
    }
}

pub fn check_dynamic_tree<D: DynamicTree>(edges: Vec<(u8, u8)>, mut actual: D, n: usize) {
    let mut expected = ParentPointerTree::new(n);
    let mut map = HashMap::new();

    for (u, v) in edges {
        let u = u as usize % n;
        let v = v as usize % n;
        if expected.has_edge(u, v) {
            assert!(expected.is_connected(u, v));

            assert!(actual.is_connected(u, v));

            expected.cut((u, v));
            assert!(!expected.is_connected(u, v));

            let e = if let Some(e) = map.remove(&(u, v)) {
                e
            } else {
                map.remove(&(v, u)).unwrap()
            };

            actual.cut(e);
            assert!(!actual.is_connected(u, v));
        } else if !expected.is_connected(u, v) {
            expected.link(u, v);
            assert!(expected.is_connected(u, v));

            let e = actual.link(u, v).unwrap();
            map.insert((u, v), e);
            assert!(actual.is_connected(u, v));
        }

        _check_dynamic_tree(&expected, &actual, n);
    }
}

pub fn _check_dynamic_tree<D: DynamicTree>(expected: &ParentPointerTree, actual: &D, n: usize) {
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
