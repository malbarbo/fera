// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use prelude::*;

use std::collections::HashMap;
use std::hash::BuildHasherDefault;
use std::ops::{Index, IndexMut};
use fnv::FnvHasher;

type HashMapFnv<K, V> = HashMap<K, V, BuildHasherDefault<FnvHasher>>;

/// A property backed by a [`HashMap`].
///
/// [`HashMap`]: https://doc.rust-lang.org/stable/std/collections/struct.HashMap.html
pub struct HashMapProp<I: GraphItem, T: Clone> {
    default: T,
    map: HashMapFnv<I, T>,
}

impl<I: GraphItem, T: Clone> HashMapProp<I, T> {
    /// Creates a new [`HashMapProp`] that maps each to key to a reference to `default` value until
    /// a value is associated with the key.
    ///
    /// As there can be many references to `default`, interior mutability should be used with
    /// caution.
    pub fn new(default: T) -> Self {
        HashMapProp {
            default: default,
            map: HashMapFnv::default(),
        }
    }
}

impl<I: GraphItem, T: Clone> Index<I> for HashMapProp<I, T> {
    type Output = T;

    #[inline]
    fn index(&self, v: I) -> &Self::Output {
        self.map.get(&v).unwrap_or(&self.default)
    }
}

impl<I: GraphItem, T: Clone> IndexMut<I> for HashMapProp<I, T> {
    #[inline]
    fn index_mut(&mut self, v: I) -> &mut Self::Output {
        let default = &self.default;
        self.map.entry(v).or_insert_with(|| default.clone())
    }
}

impl<G, T> VertexPropMutNew<G, T> for HashMapProp<Vertex<G>, T>
    where G: WithVertex,
          T: Clone
{
    fn new_vertex_prop(_: &G, value: T) -> Self
        where T: Clone
    {
        Self::new(value)
    }
}

impl<G, T> EdgePropMutNew<G, T> for HashMapProp<Edge<G>, T>
    where G: WithEdge,
          T: Clone
{
    fn new_edge_prop(_: &G, value: T) -> Self
        where T: Clone
    {
        Self::new(value)
    }
}
