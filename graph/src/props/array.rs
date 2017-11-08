// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use prelude::*;

use std::ops::{Index, IndexMut};

// TODO: define a feature to disable bounds check (or a property type?).
/// A vertex property backed by a [`Vec`].
///
/// [`Vec`]: https://doc.rust-lang.org/stable/std/vec/struct.Vec.html
pub type VecVertexProp<G, T> = ArrayProp<VertexIndexProp<G>, Vec<T>>;

/// A edge property backed by a [`Vec`].
///
/// [`Vec`]: https://doc.rust-lang.org/stable/std/vec/struct.Vec.html
pub type VecEdgeProp<G, T> = ArrayProp<EdgeIndexProp<G>, Vec<T>>;

// TODO: Define SliceVertexProp and SliceEdgeProp

// TODO: Rename to SequenceProp
/// A property backed by an array.
#[derive(Clone, Debug)]
pub struct ArrayProp<P, D> {
    index: P,
    data: D,
}

impl<P, D> ArrayProp<P, D> {
    fn new(index: P, data: D) -> Self {
        Self { index, data }
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
    where G: VertexList + WithVertexIndexProp
{
    fn new_vertex_prop(g: &G, value: T) -> Self
        where T: Clone
    {
        ArrayProp::new(g.vertex_index(), vec![value; g.num_vertices()])
    }
}

impl<T, G> EdgePropMutNew<G, T> for ArrayProp<EdgeIndexProp<G>, Vec<T>>
    where G: EdgeList + WithEdgeIndexProp
{
    fn new_edge_prop(g: &G, value: T) -> Self
        where T: Clone
    {
        ArrayProp::new(g.edge_index(), vec![value; g.num_edges()])
    }
}
