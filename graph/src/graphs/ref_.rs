// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use prelude::*;
use choose::Choose;

use rand::Rng;
use std::ops::{Index, IndexMut};

impl<'a, 'b, G: WithVertex> VertexTypes<'a, &'b G> for &'b G {
    type VertexIter = VertexIter<'a, G>;
    type OutNeighborIter = OutNeighborIter<'a, G>;
}

impl<'a, G: WithVertex> WithVertex for &'a G {
    type Vertex = Vertex<G>;
    type OptionVertex = OptionVertex<G>;

    fn vertex_none() -> OptionVertex<Self> {
        G::vertex_none()
    }

    fn vertex_some(v: Vertex<Self>) -> OptionVertex<Self> {
        G::vertex_some(v)
    }
}

impl<'a, 'b, G: WithEdge> EdgeTypes<'a, &'b G> for &'b G {
    type EdgeIter = EdgeIter<'a, G>;
    type OutEdgeIter = OutEdgeIter<'a, G>;
}

impl<'a, G: WithEdge> WithEdge for &'a G {
    type Kind = G::Kind;
    type Edge = Edge<G>;
    type OptionEdge = OptionEdge<G>;

    fn source(&self, e: Edge<Self>) -> Vertex<Self> {
        G::source(self, e)
    }

    fn target(&self, e: Edge<Self>) -> Vertex<Self> {
        G::target(self, e)
    }

    fn orientation(&self, e: Edge<Self>) -> Orientation {
        G::orientation(self, e)
    }

    fn end_vertices(&self, e: Edge<Self>) -> (Vertex<Self>, Vertex<Self>) {
        G::end_vertices(self, e)
    }

    fn opposite(&self, u: Vertex<Self>, e: Edge<Self>) -> Vertex<Self> {
        G::opposite(self, u, e)
    }

    // The compiler is not smart enough to allow this, so we use the default reverse
    // implemenetation
    //
    // fn reverse(&self, e: Edge<Self>) -> Edge<Self> where Self: WithEdge<Kind = Undirected> {
    //     G::reverse(self, e)
    // }

    fn get_reverse(&self, e: Edge<Self>) -> Option<Edge<Self>> {
        G::get_reverse(self, e)
    }

    fn edge_none() -> OptionEdge<Self> {
        G::edge_none()
    }

    fn edge_some(e: Edge<Self>) -> OptionEdge<Self> {
        G::edge_some(e)
    }
}

impl<'a, G: VertexList> VertexList for &'a G {
    fn vertices(&self) -> VertexIter<Self> {
        G::vertices(self)
    }

    fn num_vertices(&self) -> usize {
        G::num_vertices(self)
    }
}

impl<'a, G: EdgeList> EdgeList for &'a G {
    fn edges(&self) -> EdgeIter<Self> {
        G::edges(self)
    }

    fn num_edges(&self) -> usize {
        G::num_edges(self)
    }

    fn get_edge_by_ends(&self, u: Vertex<Self>, v: Vertex<Self>) -> Option<Edge<Self>> {
        G::get_edge_by_ends(self, u, v)
    }
}

impl<'a, G: Adjacency> Adjacency for &'a G {
    fn out_neighbors(&self, v: Vertex<Self>) -> OutNeighborIter<Self> {
        G::out_neighbors(self, v)
    }

    fn out_degree(&self, v: Vertex<Self>) -> usize {
        G::out_degree(self, v)
    }
}

impl<'a, G: Incidence> Incidence for &'a G {
    fn out_edges(&self, v: Vertex<Self>) -> OutEdgeIter<Self> {
        G::out_edges(self, v)
    }
}

impl<'a, G: WithVertexIndexProp> WithVertexIndexProp for &'a G {
    type VertexIndexProp = VertexIndexProp<G>;

    fn vertex_index(&self) -> VertexIndexProp<Self> {
        G::vertex_index(self)
    }
}

impl<'a, G: WithEdgeIndexProp> WithEdgeIndexProp for &'a G {
    type EdgeIndexProp = EdgeIndexProp<G>;

    fn edge_index(&self) -> EdgeIndexProp<Self> {
        G::edge_index(self)
    }
}

// Properties

pub struct RefVertexProp<G: WithVertexProp<T>, T>(DefaultVertexPropMut<G, T>);

impl<G: WithVertexProp<T>, T> Index<Vertex<G>> for RefVertexProp<G, T> {
    type Output = T;

    fn index(&self, v: Vertex<G>) -> &Self::Output {
        self.0.index(v)
    }
}

impl<G: WithVertexProp<T>, T> IndexMut<Vertex<G>> for RefVertexProp<G, T> {
    fn index_mut(&mut self, v: Vertex<G>) -> &mut Self::Output {
        self.0.index_mut(v)
    }
}

impl<'a, G: WithVertexProp<T>, T> VertexPropMutNew<&'a G, T> for RefVertexProp<G, T> {
    fn new_vertex_prop(g: &&'a G, value: T) -> Self
    where
        T: Clone,
    {
        RefVertexProp(G::default_vertex_prop(*g, value))
    }
}

impl<'a, G: WithVertexProp<T>, T> WithVertexProp<T> for &'a G {
    type VertexProp = RefVertexProp<G, T>;

    fn default_vertex_prop(&self, value: T) -> DefaultVertexPropMut<Self, T>
    where
        T: Clone,
    {
        RefVertexProp(G::default_vertex_prop(self, value))
    }
}

pub struct RefEdgeProp<G: WithEdgeProp<T>, T>(DefaultEdgePropMut<G, T>);

impl<G: WithEdgeProp<T>, T> Index<Edge<G>> for RefEdgeProp<G, T> {
    type Output = T;

    fn index(&self, v: Edge<G>) -> &Self::Output {
        self.0.index(v)
    }
}

impl<G: WithEdgeProp<T>, T> IndexMut<Edge<G>> for RefEdgeProp<G, T> {
    fn index_mut(&mut self, v: Edge<G>) -> &mut Self::Output {
        self.0.index_mut(v)
    }
}

impl<'a, G: WithEdgeProp<T>, T> EdgePropMutNew<&'a G, T> for RefEdgeProp<G, T> {
    fn new_edge_prop(g: &&'a G, value: T) -> Self
    where
        T: Clone,
    {
        RefEdgeProp(G::default_edge_prop(*g, value))
    }
}

impl<'a, G: WithEdgeProp<T>, T> WithEdgeProp<T> for &'a G {
    type EdgeProp = RefEdgeProp<G, T>;

    fn default_edge_prop(&self, value: T) -> DefaultEdgePropMut<Self, T>
    where
        T: Clone,
    {
        RefEdgeProp(G::default_edge_prop(self, value))
    }
}

impl<'a, G: BasicVertexProps> BasicVertexProps for &'a G {}

impl<'a, G: BasicEdgeProps> BasicEdgeProps for &'a G {}

impl<'a, G: BasicProps> BasicProps for &'a G {}

// Choose

impl<'a, G> Choose for &'a G
where
    G: 'a + Choose,
{
    fn choose_vertex<R: Rng>(&self, rng: R) -> Option<Vertex<Self>> {
        G::choose_vertex(self, rng)
    }

    fn choose_out_neighbor<R: Rng>(&self, v: Vertex<Self>, rng: R) -> Option<Vertex<Self>> {
        G::choose_out_neighbor(self, v, rng)
    }

    fn choose_edge<R: Rng>(&self, rng: R) -> Option<Edge<Self>> {
        G::choose_edge(self, rng)
    }

    fn choose_out_edge<R: Rng>(&self, v: Vertex<Self>, rng: R) -> Option<Edge<Self>> {
        G::choose_out_edge(self, v, rng)
    }
}
