use std::cell::{Cell, UnsafeCell};
use std::fmt::Debug;
use std::mem;
use std::ptr;

use linkcut::{Node, link, cut, is_connected};
use DynamicTree;

pub struct LinkCutTree {
    nodes: Vec<UnsafeCellNode<'static>>,
}

impl LinkCutTree {
    pub fn new(n: usize) -> Self {
        Self { nodes: (0..n).map(UnsafeCellNode::new).collect() }
    }
}

impl DynamicTree for LinkCutTree {
    type Edge = (usize, usize);

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
