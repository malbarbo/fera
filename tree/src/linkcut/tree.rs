use std::cell::{Cell, UnsafeCell};
use std::fmt::Debug;
use std::mem;
use std::ptr;

use linkcut::{cut, is_connected, link, Node};
use DynamicTree;

pub struct LinkCutTree {
    nodes: Vec<UnsafeCellNode<'static>>,
}

impl DynamicTree for LinkCutTree {
    // TODO: use an opaque type?
    type Edge = (usize, usize);

    fn new(n: usize) -> Self {
        Self {
            nodes: (0..n).map(UnsafeCellNode::new).collect(),
        }
    }

    fn num_vertices(&self) -> usize {
        self.nodes.len()
    }

    fn is_connected(&self, x: usize, y: usize) -> bool {
        x == y || is_connected(&self.nodes[x], &self.nodes[y])
    }

    fn link(&mut self, x: usize, y: usize) -> Option<Self::Edge> {
        link(&self.nodes[x], &self.nodes[y]);
        Some((x, y))
    }

    fn cut(&mut self, (x, y): Self::Edge) {
        cut(&self.nodes[x], &self.nodes[y]);
    }

    fn ends(&self, e: &Self::Edge) -> (usize, usize) {
        *e
    }

    fn clear(&mut self) {
        for node in &mut self.nodes {
            node.revert.set(false);
            node.inner = UnsafeCell::default();
        }
    }
}

#[derive(Default, Debug)]
struct CellNode<'a> {
    parent: Option<&'a UnsafeCellNode<'a>>,
    left: Option<&'a UnsafeCellNode<'a>>,
    right: Option<&'a UnsafeCellNode<'a>>,
}

#[derive(Default)]
pub struct UnsafeCellNode<'a> {
    id: usize,
    revert: Cell<bool>,
    inner: UnsafeCell<CellNode<'a>>,
}

impl<'a> UnsafeCellNode<'a> {
    pub fn new(id: usize) -> Self {
        let mut new = Self::default();
        new.id = id;
        new
    }

    pub fn id(&self) -> usize {
        self.id
    }

    fn inner(&self) -> &CellNode<'a> {
        unsafe { &*self.inner.get() }
    }

    unsafe fn inner_mut(&self) -> &mut CellNode<'a> {
        &mut *self.inner.get()
    }
}

impl<'a> Node for UnsafeCellNode<'a> {
    fn revert(&self) -> bool {
        self.revert.get()
    }

    fn flip_revert(&self) {
        self.revert.set(!self.revert.get());
    }

    fn left(&self) -> Option<&Self> {
        self.inner().left
    }

    fn set_left(&self, left: Option<&Self>) {
        unsafe {
            // TODO: explain why this is safe
            self.inner_mut().left = mem::transmute(left);
        }
    }

    fn right(&self) -> Option<&Self> {
        self.inner().right
    }

    fn set_right(&self, right: Option<&Self>) {
        unsafe {
            self.inner_mut().right = mem::transmute(right);
        }
    }

    fn parent(&self) -> Option<&Self> {
        self.inner().parent
    }

    fn set_parent(&self, parent: Option<&Self>) {
        unsafe {
            self.inner_mut().parent = mem::transmute(parent);
        }
    }
}

impl<'a> Debug for UnsafeCellNode<'a> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        f.debug_struct("LinkCutTreeNode")
            .field("id", &self.id())
            .field("p", &self.inner().parent.map(|x| x.id()))
            .field("l", &self.inner().left.map(|x| x.id()))
            .field("r", &self.inner().right.map(|x| x.id()))
            .field("f", &self.revert.get())
            .finish()
    }
}

impl<'a> PartialEq for UnsafeCellNode<'a> {
    fn eq(&self, other: &Self) -> bool {
        ptr::eq(&self.inner, &other.inner)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use check_dynamic_tree;

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
}
