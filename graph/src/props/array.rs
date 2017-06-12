use prelude::*;

use std::ops::{Deref, Index, IndexMut};

// TODO: Define a feature to disable bounds check.
pub type VecVertexProp<G, T> = ArrayProp<VertexIndexProp<G>, Vec<T>>;
pub type VecEdgeProp<G, T> = ArrayProp<EdgeIndexProp<G>, Vec<T>>;

#[derive(Clone, Debug)]
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

impl<I, P, D> Index<I> for ArrayProp<P, D>
    where P: PropGet<I, Output = usize>,
          D: Index<usize>
{
    type Output = D::Output;

    #[inline(always)]
    fn index(&self, item: I) -> &Self::Output {
        self.data.index(self.index.get(item))
    }
}

impl<I, P, D> IndexMut<I> for ArrayProp<P, D>
    where P: PropGet<I, Output = usize>,
          D: IndexMut<usize>
{
    #[inline(always)]
    fn index_mut(&mut self, item: I) -> &mut Self::Output {
        self.data.index_mut(self.index.get(item))
    }
}

impl<T, G> VertexPropMutNew<G, T> for ArrayProp<VertexIndexProp<G>, Vec<T>>
    where G: VertexList + VertexIndex
{
    fn new_vertex_prop(g: &G, value: T) -> Self
        where T: Clone
    {
        ArrayProp {
            index: g.vertex_index(),
            data: vec![value; g.num_vertices()],
        }
    }
}

impl<T, G> EdgePropMutNew<G, T> for ArrayProp<EdgeIndexProp<G>, Vec<T>>
    where G: EdgeList + EdgeIndex
{
    fn new_edge_prop(g: &G, value: T) -> Self
        where T: Clone
    {
        ArrayProp {
            index: g.edge_index(),
            data: vec![value; g.num_edges()],
        }
    }
}
