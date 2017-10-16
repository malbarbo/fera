use std::cell::{Cell, UnsafeCell};
use std::mem;
use std::ops::{Index, Range};
use std::ptr;

pub trait Sequence: 'static + Index<usize, Output = EdgeRef> {
    fn with_capacity(cap: usize) -> Self;
    fn first(&self) -> Option<EdgeRef>;
    fn push(&self, value: EdgeRef);
    fn rotate(&self, p: usize);
    fn extract(&self, range: Range<usize>, to: &Self);
    fn append(&self, from: &Self);
    fn len(&self) -> usize;
    fn tree_rank(e: &Edge) -> (&'static Self, usize);
}

pub struct Seq {
    // TODO: use a type parameter
    parent: Cell<*const ()>,
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
        Self {
            parent: ptr::null().into(),
            inner: Vec::with_capacity(cap).into(),
        }
    }

    fn first(&self) -> Option<EdgeRef> {
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

    fn parent(&self) -> *const () {
        self.parent.get()
    }

    fn set_parent(&self, parent: *const ()) {
        self.parent.set(parent);
    }

    // TODO: extract_to -> extract
    // TODO: extract -> extract_and_pop_ends
    fn extract_to(&self, range: Range<usize>, to: &Self) {
        {
            let d = self.inner_mut().drain(range.clone());
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

    fn pop(&self) -> Option<EdgeRef> {
        self.inner_mut().pop()
    }
}


pub struct NestedSeq {
    pref_seq_len: usize,
    inner: UnsafeCell<Vec<Box<Seq>>>,
}

impl Index<usize> for NestedSeq {
    type Output = EdgeRef;

    fn index(&self, index: usize) -> &EdgeRef {
        let (i, j) = self.find_seq(index);
        &self.inner()[i][j]
    }
}

impl Sequence for NestedSeq {
    fn with_capacity(cap: usize) -> Self {
        let cap_sqrt = (cap as f64).sqrt() as usize;
        Self {
            pref_seq_len: cap_sqrt,
            inner: Vec::with_capacity(cap_sqrt).into(),
        }
    }

    fn first(&self) -> Option<EdgeRef> {
        // self.inner().first().and_then(|x| x.first())
        self.inner().iter().find(|x| x.len() != 0).and_then(|x| x.first())
    }

    fn push(&self, edge: EdgeRef) {
        if let Some(seq) = self.inner().last() {
            if seq.len() < self.max_seq_len() {
                seq.push(edge);
                self.check();
                return;
            }
        }
        // FIXME: respect min_seq_len, add and split
        self.add_new_seq().push(edge);

        self.check();
    }

    fn rotate(&self, p: usize) {
        let (i, j) = self.find_seq(p);
        if j == 0 {
            self.inner_mut().rotate(i);
            return;
        }

        if i == 0 {
            if self.inner().len() == 1 {
                assert_eq!(p, j);
                self.inner()[0].rotate(j);
            } else {
                let first = self.inner().first().unwrap();
                let last = self.inner().last().unwrap();
                first.extract_to(0..j, last);
            }
        } else {
            let tree = &self.inner()[i];
            let prev = &self.inner()[i - 1];
            tree.extract_to(0..j, prev);
            self.inner_mut().rotate(i);
        }

        self.check();
    }

    fn extract(&self, range: Range<usize>, to: &Self) {
        let (start, si) = self.find_seq(range.start);
        let (end, ei) = self.find_seq(range.end - 1);

        if start == end {
            self.inner()[start].extract(si..ei + 1, to.add_new_seq());
            //if self.inner()[start].len() == 0 {
            //    self.inner_mut().remove(0);
            //}
            // TODO: to and start can be empty
            return;
        }

        // first tree
        let seq = to.add_new_seq();
        let tree = &self.inner()[start];
        tree.extract_to(si + 1..tree.len(), seq);
        tree.pop().unwrap();
        if seq.len() == 0 {
            to.inner_mut().pop().unwrap();
        }
        // TODO: tree can be empty

        // middle trees
        to.inner_mut().extend(self.inner_mut().drain(start + 1..end));

        // last tree
        let seq = to.add_new_seq();
        let tree = &self.inner()[start + 1];
        tree.extract_to(0..ei + 1, seq);
        seq.pop().unwrap();

        for seq in to.inner() {
            seq.set_parent(ptr(to))
        }

        self.check();
        to.check();
    }

    fn append(&self, from: &Self) {
        let last = self.inner().last().unwrap();
        let first = from.inner().first().unwrap();
        if last.len() + first.len() < self.max_seq_len() {
            last.append(first);
            self.inner_mut().extend(from.inner_mut().drain(1..));
            from.inner_mut().pop();
        } else {
            self.inner_mut().append(from.inner_mut());
        }
        for t in self.inner() {
            t.set_parent(ptr(self));
        }
        self.check();
        from.check();
    }

    fn len(&self) -> usize {
        self.inner().iter().map(|x| x.len()).sum()
    }

    fn tree_rank(e: &Edge) -> (&'static Self, usize) {
        let (tree, rank) = Seq::tree_rank(e);
        let seq: &'static Self = unsafe { mem::transmute(tree.parent()) };
        let mut count = 0;
        for t in seq.inner().iter() {
            if ptr::eq(&**t, tree) {
                return (seq, count + rank);
            }
            count += t.len();
        }
        unreachable!();
    }
}

impl NestedSeq {
    fn find_seq(&self, index: usize) -> (usize, usize) {
        let mut count = 0;
        for (i, seq) in self.inner().iter().enumerate() {
            if index < count + seq.len() {
                return (i, index - count);
            }
            count += seq.len();
        }
        panic!("index out of bounds: {}", index)
    }

    fn max_seq_len(&self) -> usize {
        2 * self.pref_seq_len
    }

    fn min_seq_len(&self) -> usize {
        self.pref_seq_len / 2
    }

    fn inner(&self) -> &Vec<Box<Seq>> {
        unsafe { &*self.inner.get() }
    }

    fn inner_mut(&self) -> &mut Vec<Box<Seq>> {
        unsafe { &mut *self.inner.get() }
    }

    fn add_new_seq(&self) -> &Seq {
        self.inner_mut().push(Seq::with_capacity(self.pref_seq_len).into());
        let seq = self.inner().last().unwrap();
        seq.set_parent(ptr(self));
        seq
    }

    fn check(&self) {
        let mut count = 0;
        for t in self.inner() {
            let t: &Seq = t;
            assert_eq!(ptr(self), t.parent());
            for (i, &e) in t.inner().iter().enumerate() {
                let (tt, r) = Seq::tree_rank(e);
                assert_eq!(i, r);
                assert_eq!(ptr(tt), ptr(t));
                assert_eq!(ptr(e), ptr(t[r]));
                assert_eq!(ptr(self[count + r]), ptr(e));

                let (x, rr) = NestedSeq::tree_rank(e);
                assert_eq!(count + r, rr);
                assert_eq!(ptr(x), ptr(self));
            }
            count += t.len();
        }
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

#[cfg(test)]
unsafe fn static_lifetime<T>(x: &T) -> &'static T {
    ::std::mem::transmute(x)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seq() {
        basic::<Seq>();
    }

    #[test]
    fn test_nested_seq() {
        basic::<NestedSeq>();
    }

    fn ids<S: Sequence>(seq: &S) -> Vec<usize> {
        (0..seq.len()).map(|i| seq[i].id).collect()
    }

    fn basic<S: Sequence>() {
        let n = 9;
        let edges: Vec<_> = (0..n).map(Edge::new).collect();
        let e = |i: usize| -> EdgeRef { unsafe { static_lifetime(&edges[i]) } };

        let seq = S::with_capacity(n);
        for i in 0..n {
            seq.push(e(i));
        }

        let mut expected = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];

        assert_eq!(expected, ids(&seq));

        for i in 0..n {
            expected.rotate(i);
            seq.rotate(i);
            assert_eq!(expected, ids(&seq), "rotate = {}", i);
            assert_eq!(expected.len(), seq.len(), "rotate = {}", i);
            assert_eq!(expected.first(),
                       seq.first().map(|e| &e.id),
                       "rotate = {}",
                       i);
        }

        for i in 0..n {
            for j in i + 2..(n + 1) {
                let (e1, e2, mut exp1) = {
                    let mut d = expected.drain(i..j);
                    let e1 = d.next().unwrap();
                    let e2 = d.next_back().unwrap();
                    let exp1: Vec<_> = d.collect();
                    (e1, e2, exp1)
                };
                println!("len = {}, range = {:?}", seq.len(), i..j);
                println!("seq = {:?}", ids(&seq));

                let seq1 = S::with_capacity(n);
                seq.extract(i..j, &seq1);
                assert_eq!(exp1, ids(&seq1));

                expected.append(&mut exp1);
                expected.push(e1);
                expected.push(e2);

                seq.append(&seq1);
                seq.push(e(e1));
                seq.push(e(e2));

                assert_eq!(expected, ids(&seq));
            }
        }
    }
}
