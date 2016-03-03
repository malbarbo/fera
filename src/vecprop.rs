use std::fmt::Debug;
use std::ops::{Deref, Index, IndexMut};

// TODO: Define a feature to disable bounds check.

pub trait ToIndex<I>: Clone + Debug {
    fn to_index(&self, x: I) -> usize;
}

impl<I: Clone + Debug> ToIndex<I> for fn (I) -> usize {
    fn to_index(&self, x: I) -> usize {
        self(x)
    }
}

#[derive(Clone, Debug)]
pub struct VecProp<I, T> {
    to_index: I,
    data: Vec<T>,
}

impl<I, T> VecProp<I, T> {
    pub fn new(to_index: I, data: Vec<T>) -> Self {
        VecProp {
            to_index: to_index,
            data: data,
        }
    }
}

impl<I, T> Deref for VecProp<I, T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &*self.data
    }
}

impl<K, I, T> Index<K> for VecProp<I, T>
    where I: ToIndex<K>
{
    type Output = T;

    #[inline(always)]
    fn index(&self, key: K) -> &Self::Output {
        self.data.index(self.to_index.to_index(key))
    }
}

impl<K, I, T> IndexMut<K> for VecProp<I, T>
    where I: ToIndex<K>
{
    #[inline(always)]
    fn index_mut(&mut self, key: K) -> &mut Self::Output {
        self.data.index_mut(self.to_index.to_index(key))
    }
}
