use std::mem;
use std::ops::Range;
use std::ptr;

use DynamicTree;

mod seq;
pub use self::seq::*;

// invariants:
// if x is a isolated vertex,
//     active[x] == None
// else
//     active[x].source == u
pub struct EulerTourTree<A: Sequence<&'static Edge>> {
    trees: Box<[A]>,
    ends: Box<[(usize, usize)]>,
    edges: Box<[Edge]>,
    active: Box<[Option<&'static Edge>]>,
    free_trees: Vec<usize>,
    free_edges: Vec<usize>,
}

impl<A: Sequence<&'static Edge>> DynamicTree for EulerTourTree<A> {
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
                self.make_root(u);
                self.make_root(v);
                self.push(u_act.tree(), e);
                self.free_trees.push(v_act.tree());
                {
                    let (u_tree, v_tree) = self.get_trees(u_act.tree(), v_act.tree());
                    for i in 0..v_tree.len() {
                        v_tree[i].set_tree(u_act.tree());
                        v_tree[i].set_rank(u_tree.len() + i);
                    }
                    u_tree.append(v_tree);
                }
                self.push(u_act.tree(), f);
            }
            (Some(u_act), None) => {
                self.make_root(u);
                self.active[v] = Some(f);
                self.push(u_act.tree(), e);
                self.push(u_act.tree(), f);
            }
            (None, Some(v_act)) => {
                self.make_root(v);
                self.active[u] = Some(e);
                self.push(v_act.tree(), f);
                self.push(v_act.tree(), e);
            }
            (None, None) => {
                self.active[u] = Some(e);
                self.active[v] = Some(f);
                self.new_tree(e, f);
            }
        }
        debug_assert!(self.is_connected(u, v));
        debug_assert!(self.check());
        Some(i)
    }

    fn cut(&mut self, edge: Self::Edge) {
        // TODO: avoid make_root
        let (u, v) = self.ends(&self.edges[edge << 1]);
        self.make_root(u);
        let (i, range) = self.tree_range(edge);
        let j = self.free_trees.pop().unwrap();
        let (i_free, j_free) = {
            let (i_tree, j_tree) = self.get_trees(i, j);
            i_tree.extract(range, j_tree);
            for k in 0..j_tree.len() {
                j_tree[k].set_tree(j);
                j_tree[k].set_rank(k);
            }
            for k in 0..i_tree.len() {
                i_tree[k].set_tree(i);
                i_tree[k].set_rank(k);
            }
            (i_tree.len() == 0, j_tree.len() == 0)
        };
        let e = self.trees[i].last().cloned();
        self.set_active(u, e);
        let e = self.trees[j].first().cloned();
        self.set_active(v, e);
        if i_free {
            self.free_trees.push(i);
        }
        if j_free {
            self.free_trees.push(j);
        }
        self.free_edges.push(edge);
        debug_assert!(!self.is_connected(u, v));
        debug_assert!(self.check());
    }
}

impl<A: Sequence<&'static Edge>> EulerTourTree<A> {
    pub fn new(n: usize) -> Self {
        let max_edges = 2 * (n - 1);
        let max_trees = n / 2 + 1;

        Self {
            trees: (0..max_trees)
                .map(|_| A::with_capacity(max_edges))
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            ends: vec![(0, 0); n - 1].into_boxed_slice(),
            edges: (0..max_edges)
                .map(Edge::new)
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            active: vec![None; n].into_boxed_slice(),
            free_edges: (0..n - 1).rev().collect(),
            free_trees: (0..max_trees).rev().collect(),
        }
    }

    fn make_root(&mut self, x: usize) {
        if let Some(e) = self.active[x] {
            assert_eq!(x, self.source(e));
            if e.rank() == 0 {
                return;
            }

            let tree = &mut self.trees[e.tree()];
            tree.rotate(e.rank());
            for i in 0..tree.len() {
                tree[i].set_rank(i);
            }
        }
        debug_assert_eq!(x, self.find_root_node(x));
        debug_assert!(self.check());
    }

    fn find_root_node(&self, x: usize) -> usize {
        if let Some(e) = self.active[x] {
            self.source(self.trees[e.tree()].first().unwrap())
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

    fn tree_range(&self, index: usize) -> (usize, Range<usize>) {
        let e = &self.edges[index << 1];
        let f = &self.edges[(index << 1) + 1];
        if e.rank() < f.rank() {
            (e.tree(), e.rank()..f.rank() + 1)
        } else {
            (e.tree(), f.rank()..e.rank() + 1)
        }
    }

    fn set_active(&mut self, v: usize, e: Option<&'static Edge>) {
        self.active[v] = e.map(|e| {
            let (s, t) = self.ends(e);
            if s == v {
                e
            } else {
                assert_eq!(v, t);
                self.pair(e)
            }
        });
    }

    fn pair(&self, e: &Edge) -> &'static Edge {
        unsafe { mem::transmute(&self.edges[e.id ^ 1]) }
    }

    fn get_trees(&mut self, i: usize, j: usize) -> (&mut A, &mut A) {
        assert_ne!(i, j);
        unsafe {
            let a: *mut A = &mut self.trees[i];
            let b: *mut A = &mut self.trees[j];
            (mem::transmute(a), mem::transmute(b))
        }
    }

    fn push(&mut self, tree: usize, e: &'static Edge) {
        e.set_tree(tree);
        e.set_rank(self.trees[tree].len());
        self.trees[tree].push(e);
    }

    fn new_edge(&mut self, u: usize, v: usize) -> (usize, &'static Edge, &'static Edge) {
        assert_ne!(u, v);
        let i = self.free_edges.pop().unwrap();
        self.ends[i] = (u, v);
        let e = &self.edges[i << 1];
        let f = &self.edges[(i << 1) + 1];
        unsafe { (i, mem::transmute(e), mem::transmute(f)) }
    }

    fn new_tree(&mut self, e: &'static Edge, f: &'static Edge) -> usize {
        let i = self.free_trees.pop().unwrap();
        e.set_tree(i);
        e.set_rank(0);
        f.set_tree(i);
        f.set_rank(1);
        self.trees[i].push(e);
        self.trees[i].push(f);
        i
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
