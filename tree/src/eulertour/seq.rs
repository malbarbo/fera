use std::cell::{Cell, UnsafeCell};
use std::mem;
use std::ops::{Index, Range};
use std::ptr;

pub trait Sequence: Index<usize, Output = EdgeRef> {
    fn with_capacity(cap: usize) -> Self;
    fn first(&self) -> Option<EdgeRef>;
    fn last(&self) -> Option<EdgeRef>;
    fn push(&self, value: EdgeRef);
    fn rotate(&self, p: usize);
    fn extract(&self, range: Range<usize>, to: &Self);
    fn append(&self, from: &Self);
    fn len(&self) -> usize;
    fn tree_rank(e: &Edge) -> (&'static Self, usize);
}

pub struct Seq {
    inner: UnsafeCell<Vec<EdgeRef>>,
}

impl Index<usize> for Seq {
    type Output = EdgeRef;

    fn index(&self, index: usize) -> &EdgeRef {
        &self.inner()[index]
    }
}

impl Sequence for Seq {
    fn with_capacity(cap: usize) -> Self {
        Self { inner: Vec::with_capacity(cap).into() }
    }

    fn first(&self) -> Option<EdgeRef> {
        self.inner().first().cloned()
    }

    fn last(&self) -> Option<EdgeRef> {
        self.inner().first().cloned()
    }

    fn push(&self, edge: EdgeRef) {
        edge.set_tree(ptr(self));
        edge.set_rank(self.len());
        self.inner_mut().push(edge);
    }

    fn rotate(&self, p: usize) {
        self.inner_mut().rotate(p);
        for i in 0..self.len() {
            self[i].set_rank(i);
        }
    }

    fn extract(&self, range: Range<usize>, to: &Self) {
        {
            let mut d = self.inner_mut().drain(range.clone());
            d.next().unwrap();
            d.next_back().unwrap();
            to.inner_mut().extend(d);
        }

        for i in 0..to.len() {
            to[i].set_tree(ptr(to));
            to[i].set_rank(i);
        }

        for i in range.start..self.len() {
            self[i].set_tree(ptr(self));
            self[i].set_rank(i);
        }
    }

    fn append(&self, from: &Self) {
        for i in 0..from.len() {
            from[i].set_tree(ptr(self));
            from[i].set_rank(self.len() + i);
        }
        self.inner_mut().append(from.inner_mut());
    }

    fn len(&self) -> usize {
        self.inner().len()
    }

    fn tree_rank(e: &Edge) -> (&'static Self, usize) {
        (unsafe { mem::transmute(e.tree()) }, e.rank())
    }
}

impl Seq {
    fn inner(&self) -> &Vec<EdgeRef> {
        unsafe { &*self.inner.get() }
    }

    fn inner_mut(&self) -> &mut Vec<EdgeRef> {
        unsafe { &mut *self.inner.get() }
    }
}

fn ptr<T>(x: &T) -> *const () {
    x as *const _ as _
}

pub type EdgeRef = &'static Edge;

#[derive(Debug)]
pub struct Edge {
    id: usize,
    rank: Cell<usize>,
    tree: Cell<*const ()>,
}

impl Edge {
    pub fn new(id: usize) -> Self {
        Self {
            id: id,
            rank: 0.into(),
            tree: ptr::null().into(),
        }
    }

    pub fn id_pair(&self) -> usize {
        self.id ^ 1
    }

    pub fn is_reversed(&self) -> bool {
        self.id & 1 == 1
    }

    pub fn index(&self) -> usize {
        self.id >> 1
    }

    fn tree(&self) -> *const () {
        self.tree.get()
    }

    fn set_tree(&self, tree: *const ()) {
        self.tree.set(tree)
    }

    fn rank(&self) -> usize {
        self.rank.get()
    }

    fn set_rank(&self, rank: usize) {
        self.rank.set(rank)
    }
}
