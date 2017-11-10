// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use prelude::*;

use std::collections::HashMap;
use std::collections::hash_map::RandomState;
use std::hash::BuildHasher;
use std::ops::{Index, IndexMut};

/// A property backed by a [`HashMap`].
///
/// [`HashMap`]: https://doc.rust-lang.org/stable/std/collections/struct.HashMap.html
pub struct HashMapProp<I: GraphItem, T: Clone, S = RandomState> {
    default: T,
    map: HashMap<I, T, S>,
}

impl<I, T> HashMapProp<I, T>
    where I: GraphItem,
          T: Clone,
{
    /// Creates a new [`HashMapProp`] that maps each to key to a reference to `default` value until
    /// a value is associated with the key.
    ///
    /// As there can be many references to `default`, interior mutability should be used with
    /// caution.
    pub fn new(default: T) -> Self {
        HashMapProp {
            default: default,
            map: HashMap::default(),
        }
    }
}

impl<I, T, S> HashMapProp<I, T, S>
    where I: GraphItem,
          T: Clone,
          S: BuildHasher,
{
    pub fn with_hasher(default: T, hasher: S) -> Self {
        HashMapProp {
            default: default,
            map: HashMap::with_hasher(hasher),
        }
    }
}

impl<I, T, S> Index<I> for HashMapProp<I, T, S>
    where I: GraphItem,
          T: Clone,
          S: BuildHasher,
{
    type Output = T;

    #[inline]
    fn index(&self, v: I) -> &Self::Output {
        self.map.get(&v).unwrap_or(&self.default)
    }
}

impl<I, T, S> IndexMut<I> for HashMapProp<I, T, S>
    where I: GraphItem,
          T: Clone,
          S: BuildHasher,
{
    #[inline]
    fn index_mut(&mut self, v: I) -> &mut Self::Output {
        let default = &self.default;
        self.map.entry(v).or_insert_with(|| default.clone())
    }
}

impl<G, T, S> VertexPropMutNew<G, T> for HashMapProp<Vertex<G>, T, S>
    where G: WithVertex,
          T: Clone,
          S: BuildHasher + Default,
{
    fn new_vertex_prop(_: &G, value: T) -> Self
        where T: Clone
    {
        Self::with_hasher(value, S::default())
    }
}

impl<G, T, S> EdgePropMutNew<G, T> for HashMapProp<Edge<G>, T, S>
    where G: WithEdge,
          T: Clone,
          S: BuildHasher + Default,
{
    fn new_edge_prop(_: &G, value: T) -> Self
        where T: Clone
    {
        Self::with_hasher(value, S::default())
    }
}
