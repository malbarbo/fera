use std::cell::{Cell, UnsafeCell};
use std::ops::{Index, Range};

pub trait Sequence: Index<usize, Output = EdgeRef> {
    fn with_capacity(index: usize, cap: usize) -> Self;
    fn first(&self) -> Option<EdgeRef>;
    fn last(&self) -> Option<EdgeRef>;
    fn push(&self, value: EdgeRef);
    fn rotate(&self, p: usize);
    fn extract(&self, range: Range<usize>, to: &Self);
    fn append(&self, from: &Self);
    fn len(&self) -> usize;
}

pub struct Seq {
    index: usize,
    values: UnsafeCell<Vec<EdgeRef>>,
}

impl Index<usize> for Seq {
    type Output = EdgeRef;

    fn index(&self, index: usize) -> &EdgeRef {
        &self.values()[index]
    }
}

impl Sequence for Seq {
    fn with_capacity(index: usize, cap: usize) -> Self {
        Self {
            index: index,
            values: Vec::with_capacity(cap).into(),
        }
    }

    fn first(&self) -> Option<EdgeRef> {
        self.values().first().cloned()
    }

    fn last(&self) -> Option<EdgeRef> {
        self.values().first().cloned()
    }

    fn push(&self, edge: EdgeRef) {
        edge.set_tree(self.index);
        edge.set_rank(self.len());
        self.values_mut().push(edge);
    }

    fn rotate(&self, p: usize) {
        self.values_mut().rotate(p);
        for i in 0..self.len() {
            self[i].set_rank(i);
        }
    }

    fn extract(&self, range: Range<usize>, to: &Self) {
        {
            let mut d = self.values_mut().drain(range.clone());
            d.next().unwrap();
            d.next_back().unwrap();
            to.values_mut().extend(d);
        }

        for i in 0..to.len() {
            to[i].set_tree(to.index);
            to[i].set_rank(i);
        }

        for i in range.start..self.len() {
            self[i].set_tree(self.index);
            self[i].set_rank(i);
        }
    }

    fn append(&self, from: &Self) {
        for i in 0..from.len() {
            from[i].set_tree(self.index);
            from[i].set_rank(self.len() + i);
        }
        self.values_mut().append(from.values_mut());
    }

    fn len(&self) -> usize {
        self.values().len()
    }
}

impl Seq {
    fn values(&self) -> &Vec<EdgeRef> {
        unsafe { &*self.values.get() }
    }

    fn values_mut(&self) -> &mut Vec<EdgeRef> {
        unsafe { &mut *self.values.get() }
    }
}

pub type EdgeRef = &'static Edge;

#[derive(Debug)]
pub struct Edge {
    pub id: usize,
    rank: Cell<usize>,
    tree: Cell<usize>,
}

impl Edge {
    pub fn new(id: usize) -> Self {
        Self {
            id: id,
            rank: 0.into(),
            tree: 0.into(),
        }
    }

    pub fn is_reversed(&self) -> bool {
        self.id & 1 == 1
    }

    pub fn index(&self) -> usize {
        self.id >> 1
    }

    pub fn tree(&self) -> usize {
        self.tree.get()
    }

    fn set_tree(&self, tree: usize) {
        self.tree.set(tree)
    }

    pub fn rank(&self) -> usize {
        self.rank.get()
    }

    fn set_rank(&self, rank: usize) {
        self.rank.set(rank)
    }
}
