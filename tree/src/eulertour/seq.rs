use std::cell::Cell;
use std::ops::{Index, Range};

pub trait Sequence<T>: Index<usize, Output = T> {
    fn with_capacity(cap: usize) -> Self;
    fn first(&self) -> Option<&T>;
    fn last(&self) -> Option<&T>;
    fn push(&mut self, value: T);
    fn rotate(&mut self, p: usize);
    fn extract(&mut self, range: Range<usize>, to: &mut Self);
    fn append(&mut self, from: &mut Self);
    fn len(&self) -> usize;
}

impl<T> Sequence<T> for Vec<T> {
    fn with_capacity(cap: usize) -> Self {
        Vec::with_capacity(cap)
    }

    fn first(&self) -> Option<&T> {
        <[_]>::first(self)
    }

    fn last(&self) -> Option<&T> {
        <[_]>::last(self)
    }

    fn push(&mut self, value: T) {
        Vec::push(self, value);
    }

    fn rotate(&mut self, p: usize) {
        <[_]>::rotate(self, p);
    }

    fn extract(&mut self, range: Range<usize>, to: &mut Self) {
        let mut d = self.drain(range);
        d.next().unwrap();
        d.next_back().unwrap();
        to.extend(d);
    }

    fn append(&mut self, from: &mut Self) {
        Vec::append(self, from);
    }

    fn len(&self) -> usize {
        <[_]>::len(self)
    }
}

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

    pub fn set_tree(&self, tree: usize) {
        self.tree.set(tree)
    }

    pub fn rank(&self) -> usize {
        self.rank.get()
    }

    pub fn set_rank(&self, rank: usize) {
        self.rank.set(rank)
    }
}
