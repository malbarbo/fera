use graph::*;
use fera::array::{Array, VecArray};

use std::ops::{Deref, Index, IndexMut};

// TODO: Define a feature to disable bounds check.
pub type ArrayVertexProp<G, A> = ArrayProp<VertexIndexProp<G>, A>;
pub type VecVertexProp<G, T> = ArrayVertexProp<G, VecArray<T>>;

pub type ArrayEdgeProp<G, A> = ArrayProp<EdgeIndexProp<G>, A>;
pub type VecEdgeProp<G, T> = ArrayEdgeProp<G, VecArray<T>>;

#[derive(Clone)]
pub struct ArrayProp<P, D> {
    index: P,
    data: D,
}

impl<P, D> Deref for ArrayProp<P, D> {
    type Target = D;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<I, P, D> PropGet<I> for ArrayProp<P, D>
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

impl<I, P: PropGet<I, Output = usize>, D: Index<usize>> Index<I> for ArrayProp<P, D> {
    type Output = D::Output;

    #[inline(always)]
    fn index(&self, item: I) -> &Self::Output {
        self.data.index(self.index.get(item))
    }
}

impl<I, P: PropGet<I, Output = usize>, D: IndexMut<usize>> IndexMut<I> for ArrayProp<P, D> {
    #[inline(always)]
    fn index_mut(&mut self, item: I) -> &mut Self::Output {
        self.data.index_mut(self.index.get(item))
    }
}

impl<A, T, G> VertexPropMutNew<G, T> for ArrayVertexProp<G, A>
    where A: Array<T>,
          G: VertexList + VertexIndex
{
    fn new_vertex_prop(g: &G, value: T) -> Self
        where T: Clone
    {
        ArrayProp {
            index: g.vertex_index(),
            data: A::with_value(value, g.num_vertices()),
        }
    }
}

impl<A, T, G> EdgePropMutNew<G, T> for ArrayEdgeProp<G, A>
    where A: Array<T>,
          G: EdgeList + EdgeIndex
{
    fn new_edge_prop(g: &G, value: T) -> Self
        where T: Clone
    {
        ArrayProp {
            index: g.edge_index(),
            data: A::with_value(value, g.num_edges()),
        }
    }
}
