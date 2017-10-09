use std::mem;
use std::ops::{Deref, Range};
use std::ptr;

use DynamicTree;

mod seq;
pub use self::seq::*;

// TODO: explain unsafe uses
// invariants:
// if x is a isolated vertex,
//     active[x] == None
// else
//     active[x].source == u
pub struct EulerTourTree<A: 'static + Sequence> {
    trees: Vec<Box<A>>,
    ends: Box<[(usize, usize)]>,
    edges: Box<[Edge]>,
    active: Box<[Option<EdgeRef>]>,
    free_trees: Vec<&'static A>,
    free_edges: Vec<usize>,
}

impl<A: 'static + Sequence> DynamicTree for EulerTourTree<A> {
    // TODO: use an opaque type
    type Edge = usize;

    fn is_connected(&self, u: usize, v: usize) -> bool {
        self.find_root_node(u) == self.find_root_node(v)
    }

    fn link(&mut self, u: usize, v: usize) -> Option<Self::Edge> {
        if self.is_connected(u, v) {
            return None;
        }
        let (i, e, f) = self.new_edge(u, v);
        match (self.active[u], self.active[v]) {
            (Some(u_act), Some(v_act)) => {
                // TODO: avoid one call to make_root
                let t = self.tree(v_act);
                self.make_root(u);
                self.make_root(v);
                self.tree(u_act).push(e);
                self.tree(u_act).append(self.tree(v_act));
                self.tree(u_act).push(f);
                self.dispose_tree(t);
            }
            (Some(u_act), None) => {
                self.make_root(u);
                self.set_active(v, Some(f));
                self.tree(u_act).push(e);
                self.tree(u_act).push(f);
            }
            (None, Some(v_act)) => {
                self.make_root(v);
                self.set_active(u, Some(e));
                self.tree(v_act).push(f);
                self.tree(v_act).push(e);
            }
            (None, None) => {
                self.set_active(u, Some(e));
                self.set_active(v, Some(f));
                self.new_tree_with_edges(e, f);
            }
        }
        debug_assert!(self.is_connected(u, v));
        debug_assert!(self.check());
        Some(i)
    }

    fn cut(&mut self, edge: Self::Edge) {
        // TODO: avoid make_root
        let (u, v) = self.ends(self.edges(edge).0);
        self.make_root(u);

        let (i_tree, range) = self.tree_range(edge);
        let j_tree = self.new_tree();
        i_tree.extract(range, j_tree);

        self.set_active(u, i_tree.first());
        self.set_active(v, j_tree.first());

        if i_tree.len() == 0 {
            self.dispose_tree(i_tree);
        }
        if j_tree.len() == 0 {
            self.dispose_tree(j_tree);
        }
        self.dispose_edge(edge);

        debug_assert!(!self.is_connected(u, v));
        debug_assert!(self.check());
    }
}

impl<A: 'static + Sequence> EulerTourTree<A> {
    pub fn new(n: usize) -> Self {
        let max_edges = 2 * (n - 1);
        let max_trees = n / 2 + 1;

        Self {
            trees: Vec::with_capacity(max_trees),
            ends: vec![(0, 0); n - 1].into_boxed_slice(),
            edges: (0..max_edges)
                .map(Edge::new)
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            active: vec![None; n].into_boxed_slice(),
            free_edges: (0..n - 1).rev().collect(),
            free_trees: Vec::with_capacity(max_trees),
        }
    }

    fn make_root(&mut self, x: usize) {
        if let Some(e) = self.active[x] {
            assert_eq!(x, self.source(e));
            if e.rank() == 0 {
                return;
            }
            self.tree(e).rotate(e.rank());
        }
        debug_assert_eq!(x, self.find_root_node(x));
        debug_assert!(self.check());
    }

    fn find_root_node(&self, x: usize) -> usize {
        if let Some(e) = self.active[x] {
            self.source(self.tree(e).first().unwrap())
        } else {
            x
        }
    }

    fn source(&self, e: &Edge) -> usize {
        self.ends(e).0
    }

    fn ends(&self, e: &Edge) -> (usize, usize) {
        let (u, v) = self.ends[e.index()];
        if e.is_reversed() { (v, u) } else { (u, v) }
    }

    fn tree(&self, e: &Edge) -> &'static A {
        unsafe { static_lifetime(&self.trees[e.tree()]) }
    }

    fn tree_range(&self, index: usize) -> (&'static A, Range<usize>) {
        let (e, f) = self.edges(index);
        if e.rank() < f.rank() {
            (self.tree(e), e.rank()..f.rank() + 1)
        } else {
            (self.tree(e), f.rank()..e.rank() + 1)
        }
    }

    fn set_active(&mut self, v: usize, e: Option<EdgeRef>) {
        self.active[v] = e.map(|e| {
            let (s, t) = self.ends(e);
            if s == v {
                e
            } else {
                assert_eq!(v, t);
                // pair edge
                unsafe { static_lifetime(&self.edges[e.id ^ 1]) }
            }
        });
    }

    fn edges(&self, i: usize) -> (EdgeRef, EdgeRef) {
        let e = &self.edges[i << 1];
        let f = &self.edges[(i << 1) + 1];
        unsafe { (static_lifetime(e), static_lifetime(f)) }
    }

    fn new_edge(&mut self, u: usize, v: usize) -> (usize, EdgeRef, EdgeRef) {
        assert_ne!(u, v);
        let i = self.free_edges.pop().unwrap();
        let (e, f) = self.edges(i);
        self.ends[i] = (u, v);
        (i, e, f)
    }

    fn dispose_edge(&mut self, e: usize) {
        self.free_edges.push(e);
    }

    fn new_tree(&mut self) -> &'static A {
        if let Some(tree) = self.free_trees.pop() {
            tree
        } else {
            let tree = Box::new(A::with_capacity(self.trees.len(), self.edges.len()));
            self.trees.push(tree);
            unsafe { static_lifetime(self.trees.last().unwrap()) }
        }
    }

    fn new_tree_with_edges(&mut self, e: EdgeRef, f: EdgeRef) -> &'static A {
        let tree = self.new_tree();
        tree.push(e);
        tree.push(f);
        tree
    }

    fn dispose_tree(&mut self, tree: &'static A) {
        self.free_trees.push(tree);
    }

    fn check(&self) -> bool {
        for i in 0..self.active.len() {
            if let Some(e) = self.active[i] {
                assert!(ptr::eq(e, self.trees[e.tree()][e.rank()]));
            }
        }
        for (i, tree) in self.trees.iter().enumerate() {
            for j in 0..tree.len() {
                let (u, v) = self.ends(tree[j]);
                assert_eq!(
                    self.find_root_node(u),
                    self.find_root_node(v),
                    "\nactive {} = {:?}\nactive {} = {:?}",
                    u,
                    self.active[u],
                    v,
                    self.active[v]
                );
                assert_eq!(
                    tree[j].tree(),
                    i,
                    "edge {} = {:?}",
                    tree[j].index(),
                    self.ends(tree[j])
                );
                assert_eq!(tree[j].rank(), j);
            }
        }
        true
    }
}

unsafe fn static_lifetime<T>(x: &T) -> &'static T {
    mem::transmute(x)
}
