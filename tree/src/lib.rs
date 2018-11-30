// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![doc(html_root_url = "https://docs.rs/fera-tree/0.1.0/")]

//! Tree data structures.

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

    #[must_use]
    fn link(&mut self, u: usize, v: usize) -> Option<Self::Edge>;

    fn cut(&mut self, e: Self::Edge);

    fn ends(&self, e: &Self::Edge) -> (usize, usize);

    fn clear(&mut self);
}

struct CheckedDynamicTree<D: DynamicTree>(D, usize);

impl<D: DynamicTree> DynamicTree for CheckedDynamicTree<D> {
    type Edge = D::Edge;

    fn is_connected(&self, u: usize, v: usize) -> bool {
        self.0.is_connected(u, v)
    }

    fn link(&mut self, u: usize, v: usize) -> Option<Self::Edge> {
        let tree = &mut self.0;
        let e = tree.link(u, v).unwrap();
        assert_eq!((u, v), tree.ends(&e));
        assert!(tree.is_connected(u, v));
        Some(e)
    }

    fn cut(&mut self, e: Self::Edge) {
        let (u, v) = self.0.ends(&e);
        assert!(self.0.is_connected(u, v));
        self.0.cut(e);
        assert!(!self.0.is_connected(u, v));
    }

    fn ends(&self, _e: &Self::Edge) -> (usize, usize) {
        unimplemented!()
    }

    fn clear(&mut self) {
        let tree = &mut self.0;
        let n = self.1;
        tree.clear();
        for i in 0..n {
            for j in 0..n {
                assert_eq!(i == j, tree.is_connected(i, j), "{} {}", i, j);
            }
        }
    }
}

pub fn check_dynamic_tree_incremental<D: DynamicTree>(edges: Vec<(u8, u8)>, actual: D, n: usize) {
    let mut expected = CheckedDynamicTree(ParentPointerTree::new(n), n);
    let mut actual = CheckedDynamicTree(actual, n);

    // run twice to test the tree after clear
    for _ in 0..2 {
        for &(u, v) in &edges {
            let u = u as usize % n;
            let v = v as usize % n;
            assert_eq!(expected.is_connected(u, v), actual.is_connected(u, v));

            if !expected.is_connected(u, v) {
                let _ = expected.link(u, v);
                let _ = actual.link(u, v);
            }

            check_all_pairs(&expected, &actual, n);
        }

        expected.clear();
        actual.clear();
    }
}

pub fn check_dynamic_tree<D: DynamicTree>(edges: Vec<(u8, u8)>, actual: D, n: usize) {
    let mut expected = CheckedDynamicTree(ParentPointerTree::new(n), n);
    let mut actual = CheckedDynamicTree(actual, n);
    let mut map = HashMap::new();

    // run twice to test the tree after clear
    for _ in 0..1 {
        for &(u, v) in &edges {
            let u = u as usize % n;
            let v = v as usize % n;
            let e = if let Some(e) = map.remove(&(u, v)) {
                Some(e)
            } else {
                map.remove(&(v, u))
            };

            if let Some(e) = e {
                assert!(expected.is_connected(u, v));
                expected.cut((u, v));

                assert!(actual.is_connected(u, v));
                actual.cut(e);
            } else if !expected.is_connected(u, v) {
                let _ = expected.link(u, v);
                map.insert((u, v), actual.link(u, v).unwrap());
            }

            check_all_pairs(&expected, &actual, n);
        }
        expected.clear();
        actual.clear();
        map.clear();
    }
}

fn check_all_pairs<D1, D2>(expected: &D1, actual: &D2, n: usize)
where
    D1: DynamicTree,
    D2: DynamicTree,
{
    for i in 0..n {
        for j in 0..n {
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
