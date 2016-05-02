use graph::*;

use fera::array::Array;

use std::ops::{Deref, Index, IndexMut};

// TODO: Define a feature to disable bounds check.
pub type VecVertexProp<G, T> = VecProp<VertexIndexProp<G>, Vec<T>>;

pub type VecEdgeProp<G, T> = VecProp<EdgeIndexProp<G>, Vec<T>>;

#[derive(Clone)]
pub struct VecProp<P, D> {
    index: P,
    data: D,
}

impl<P, D> Deref for VecProp<P, D> {
    type Target = D;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<I, P, D> PropGet<I> for VecProp<P, D>
    where P: PropGet<I, Output = usize>,
          D: Index<usize>,
          D::Output: Clone + Sized
{
    type Output = D::Output;

    #[inline(always)]
    fn get(&self, item: I) -> D::Output {
        self.data.index(self.index.get(item)).clone()
    }
}

impl<I, P: PropGet<I, Output = usize>, D: Index<usize>> Index<I> for VecProp<P, D> {
    type Output = D::Output;

    #[inline(always)]
    fn index(&self, item: I) -> &Self::Output {
        self.data.index(self.index.get(item))
    }
}

impl<I, P: PropGet<I, Output = usize>, D: IndexMut<usize>> IndexMut<I> for VecProp<P, D> {
    #[inline(always)]
    fn index_mut(&mut self, item: I) -> &mut Self::Output {
        self.data.index_mut(self.index.get(item))
    }
}

impl<G: VertexList + VertexIndex, T> VertexPropMutNew<G, T> for VecVertexProp<G, T> {
    fn new_vertex_prop(g: &G, value: T) -> Self
        where T: Clone
    {
        VecVertexProp::<G, T> {
            index: g.vertex_index(),
            data: vec![value; g.num_vertices()],
        }
    }
}

impl<G: EdgeList + EdgeIndex, T> EdgePropMutNew<G, T> for VecEdgeProp<G, T> {
    fn new_edge_prop(g: &G, value: T) -> Self
        where T: Clone
    {
        VecEdgeProp::<G, T> {
            index: g.edge_index(),
            data: vec![value; g.num_edges()],
        }
    }
}
