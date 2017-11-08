// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Edge and vertex sets.

use prelude::*;

use std::iter;
use std::ops::IndexMut;
use std::slice;
use std::vec;

use rand::Rng;

// TODO: use OptionalMax
const NONE: usize = ::std::usize::MAX;

pub struct FastVecSet<T, P>
where
    P: IndexMut<T, Output = usize>,
    T: Copy,
{
    index: P,
    values: Vec<T>,
}

impl<T, P> FastVecSet<T, P>
where
    P: IndexMut<T, Output = usize>,
    T: Copy,
{
    pub fn insert(&mut self, item: T) -> bool {
        if self.index(item) == None {
            let i = self.values.len();
            self.values.push(item);
            self.set_index(item, i);
            true
        } else {
            false
        }
    }

    pub fn remove(&mut self, item: T) -> bool {
        if let Some(i) = self.index(item) {
            let last = self.values.last().cloned();
            self.values.swap_remove(i);
            if let Some(last) = last {
                self.set_index(last, i);
            }
            self.set_index(item, NONE);
            true
        } else {
            false
        }
    }

    pub fn clear(&mut self) {
        for &v in &self.values {
            self.index[v] = NONE;
        }
        self.values.clear();
    }

    #[inline]
    pub fn contains(&self, item: T) -> bool {
        self.index(item).is_some()
    }

    #[inline]
    pub fn iter(&self) -> iter::Cloned<slice::Iter<T>> {
        self.values.iter().cloned()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    #[inline]
    pub fn choose<R: Rng>(&self, mut rng: R) -> Option<&T> {
        rng.choose(&self.values)
    }

    #[inline]
    fn index(&self, item: T) -> Option<usize> {
        let i = self.index[item];
        if i == NONE { None } else { Some(i) }
    }

    #[inline]
    fn set_index(&mut self, item: T, i: usize) {
        self.index[item] = i;
    }
}

impl<T, P> IntoIterator for FastVecSet<T, P>
where
    P: IndexMut<T, Output = usize>,
    T: Copy,
{
    type Item = T;
    type IntoIter = vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter()
    }
}

impl<'a, T, P> IntoIterator for &'a FastVecSet<T, P>
where
    P: IndexMut<T, Output = usize>,
    T: Copy,
{
    type Item = T;
    type IntoIter = iter::Cloned<slice::Iter<'a, T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.iter().cloned()
    }
}

impl<'a, T, P> Extend<T> for FastVecSet<T, P>
where
    P: IndexMut<T, Output = usize>,
    T: Copy,
{
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>,
    {
        for item in iter {
            self.insert(item);
        }
    }
}

// FIXME: how to rewrite this without using fake type parameters?
impl FastVecSet<usize, Vec<usize>> {
    pub fn new_vertex_set<G>(g: &G) -> FastVecSet<Vertex<G>, DefaultVertexPropMut<G, usize>>
    where
        G: WithVertexProp<usize>,
    {
        FastVecSet {
            index: g.vertex_prop(NONE),
            values: vec![],
        }
    }

    pub fn new_edge_set<G>(g: &G) -> FastVecSet<Edge<G>, DefaultEdgePropMut<G, usize>>
    where
        G: WithEdgeProp<usize>,
    {
        FastVecSet {
            index: g.edge_prop(NONE),
            values: vec![],
        }
    }
}
