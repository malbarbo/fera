use graph::*;

use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::ops::{Index, IndexMut};

pub struct HashProp<I: Item, T: Clone> {
    default: T,
    map: UnsafeCell<HashMap<I, Box<T>>>,
}

impl<I: Item, T: Clone> HashProp<I, T> {
    pub fn new(default: T) -> Self {
        HashProp {
            default: default,
            map: UnsafeCell::new(HashMap::new()),
        }
    }

    fn get_map(&self) -> &mut HashMap<I, Box<T>> {
        // TODO: explain why this is safe
        unsafe { &mut *self.map.get() }
    }

    fn index_default(&self, v: I) -> &mut T {
        self.get_map().entry(v).or_insert_with(|| Box::new(self.default.clone()))
    }
}

impl<I: Item, T: Clone> Index<I> for HashProp<I, T> {
    type Output = T;

    #[inline(always)]
    fn index(&self, v: I) -> &Self::Output {
        self.index_default(v)
    }
}

impl<I: Item, T: Clone> IndexMut<I> for HashProp<I, T> {
    #[inline(always)]
    fn index_mut(&mut self, v: I) -> &mut Self::Output {
        self.index_default(v)
    }
}
