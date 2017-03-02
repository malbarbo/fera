use prelude::*;

use std::collections::HashMap;
use std::hash::BuildHasherDefault;
use std::ops::{Index, IndexMut};
use fnv::FnvHasher;

type HashMapFnv<K, V> = HashMap<K, V, BuildHasherDefault<FnvHasher>>;

// TODO: explain what happens when default has interior mutability
pub struct HashMapProp<I: GraphItem, T: Clone> {
    default: T,
    map: HashMapFnv<I, T>,
}

impl<I: GraphItem, T: Clone> HashMapProp<I, T> {
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
