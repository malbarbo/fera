use graph::*;
use std::ops::{Deref, Index, IndexMut};

// TODO: Define a feature to disable bounds check.

pub type VecPropVertex<G, T> = VecProp<VertexIndex<G>, T>;
pub type VecPropEdge<G, T> = VecProp<EdgeIndex<G>, T>;

#[derive(Clone)]
pub struct VecProp<I, T> {
    to_index: I,
    data: Vec<T>,
}

impl<I, T> Deref for VecProp<I, T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &*self.data
    }
}

impl<I: ToIndex<K>, K, T> Index<K> for VecProp<I, T> {
    type Output = T;

    #[inline(always)]
    fn index(&self, key: K) -> &Self::Output {
        self.data.index(self.to_index.to_index(key))
    }
}

impl<I: ToIndex<K>, K, T> IndexMut<K> for VecProp<I, T> {
    #[inline(always)]
    fn index_mut(&mut self, key: K) -> &mut Self::Output {
        self.data.index_mut(self.to_index.to_index(key))
    }
}

impl<G: Indices, T: Clone> PropMutVertexNew<G, T> for VecPropVertex<G, T> {
    fn new_prop_vertex(g: &G, value: T) -> Self {
        VecPropVertex::<G, T> {
            to_index: g.prop_vertex_index(),
            data: vec![value; g.num_vertices()],
        }
    }
}

impl<G: Indices, T: Clone> PropMutEdgeNew<G, T> for VecPropEdge<G, T> {
    fn new_prop_edge(g: &G, value: T) -> Self {
        VecPropEdge::<G, T> {
            to_index: g.prop_edge_index(),
            data: vec![value; g.num_edges()],
        }
    }
}
