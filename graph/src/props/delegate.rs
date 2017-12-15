// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use prelude::*;

use std::ops::{Index, IndexMut};

/// A vertex property that delegates all operations to a wrapped property.
pub struct DelegateVertexProp<G: WithVertexProp<T>, T>(DefaultVertexPropMut<G, T>);

impl<G: WithVertexProp<T>, T> Index<Vertex<G>> for DelegateVertexProp<G, T> {
    type Output = T;

    #[inline]
    fn index(&self, v: Vertex<G>) -> &Self::Output {
        self.0.index(v)
    }
}

impl<G: WithVertexProp<T>, T> IndexMut<Vertex<G>> for DelegateVertexProp<G, T> {
    #[inline]
    fn index_mut(&mut self, v: Vertex<G>) -> &mut Self::Output {
        self.0.index_mut(v)
    }
}

impl<G, D, T> VertexPropMutNew<G, T> for DelegateVertexProp<D, T>
where
    G: WithVertex<Vertex = Vertex<D>, OptionVertex = OptionVertex<D>> + AsRef<D>,
    D: WithVertexProp<T>,
{
    fn new_vertex_prop(g: &G, value: T) -> Self
    where
        T: Clone,
    {
        DelegateVertexProp(g.as_ref().vertex_prop(value))
    }
}

/// An edge property that delegates all operations to a wrapped property.
pub struct DelegateEdgeProp<G: WithEdgeProp<T>, T>(DefaultEdgePropMut<G, T>);

impl<G: WithEdgeProp<T>, T> Index<Edge<G>> for DelegateEdgeProp<G, T> {
    type Output = T;

    #[inline]
    fn index(&self, v: Edge<G>) -> &Self::Output {
        self.0.index(v)
    }
}

impl<G: WithEdgeProp<T>, T> IndexMut<Edge<G>> for DelegateEdgeProp<G, T> {
    #[inline]
    fn index_mut(&mut self, v: Edge<G>) -> &mut Self::Output {
        self.0.index_mut(v)
    }
}

impl<G, D, T> EdgePropMutNew<G, T> for DelegateEdgeProp<D, T>
where
    G: WithEdge<Edge = Edge<D>, OptionEdge = OptionEdge<D>> + AsRef<D>,
    D: WithEdgeProp<T>,
{
    fn new_edge_prop(g: &G, value: T) -> Self
    where
        T: Clone,
    {
        DelegateEdgeProp(g.as_ref().edge_prop(value))
    }
}
