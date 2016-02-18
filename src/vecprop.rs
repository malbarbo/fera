use std::ops::{Deref, Index, IndexMut};

// TODO: Define a feature to disable bounds check.

pub trait ToIndex {
    fn to_index(&self) -> usize;
}

#[derive(Clone, Debug)]
pub struct VecProp<T>(Vec<T>);

impl<T> VecProp<T> {
    pub fn new(v: Vec<T>) -> Self {
        VecProp(v)
    }
}

impl<T> Deref for VecProp<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl<I: ToIndex, T> Index<I> for VecProp<T> {
    type Output = T;

    #[inline(always)]
    fn index(&self, index: I) -> &Self::Output {
        self.0.index(index.to_index())
    }
}

impl<I: ToIndex, T> IndexMut<I> for VecProp<T> {
    #[inline(always)]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        self.0.index_mut(index.to_index())
    }
}
