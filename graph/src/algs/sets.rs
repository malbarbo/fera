// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Iterators for edge and vertex set complements.

use params::IntoOwned;
use prelude::*;

pub trait Sets {
    fn vertices_complement<I>(&self, vertices: I) -> VerticesComplement<Self>
    where
        Self: VertexList + WithVertexProp<bool>,
        I: IntoIterator,
        I::Item: IntoOwned<Vertex<Self>>,
    {
        let mut set = self.default_vertex_prop(false);
        set.set_values(vertices, true);
        VerticesComplement {
            vertices: self.vertices(),
            set,
        }
    }

    fn edges_complement<I>(&self, edges: I) -> EdgesComplement<Self>
    where
        Self: EdgeList + WithEdgeProp<bool>,
        I: IntoIterator,
        I::Item: IntoOwned<Edge<Self>>,
    {
        let mut set = self.default_edge_prop(false);
        set.set_values(edges, true);
        EdgesComplement {
            edges: self.edges(),
            set,
        }
    }

    fn independent_vertex_set_from_iter<I>(
        &self,
        vertices: I,
    ) -> IndependentVertexSetFromIter<Self, I::IntoIter>
    where
        Self: Adjacency + WithVertexProp<bool>,
        I: IntoIterator,
        I::Item: IntoOwned<Vertex<Self>>,
    {
        IndependentVertexSetFromIter {
            g: self,
            vertices: vertices.into_iter(),
            marked: self.default_vertex_prop(false),
        }
    }

    fn is_independent_vertex_set<I>(&self, vertices: I) -> bool
    where
        Self: Adjacency + WithVertexProp<bool>,
        I: IntoIterator,
        I::Item: IntoOwned<Vertex<Self>>,
    {
        let mut marked = self.default_vertex_prop(false);
        for v in vertices {
            let v = v.into_owned();
            if marked[v] {
                return false;
            }
            marked.set_values(self.out_neighbors(v), true);
        }
        true
    }
}

impl<G> Sets for G {}

pub struct VerticesComplement<'a, G>
where
    G: 'a + WithVertex + WithVertexProp<bool>,
{
    vertices: VertexIter<'a, G>,
    set: DefaultVertexPropMut<G, bool>,
}

impl<'a, G> Iterator for VerticesComplement<'a, G>
where
    G: 'a + WithVertex + WithVertexProp<bool>,
{
    type Item = Vertex<G>;

    fn next(&mut self) -> Option<Self::Item> {
        for e in self.vertices.by_ref() {
            if !self.set[e] {
                return Some(e);
            }
        }
        None
    }
}

pub struct EdgesComplement<'a, G>
where
    G: 'a + WithEdge + WithEdgeProp<bool>,
{
    edges: EdgeIter<'a, G>,
    set: DefaultEdgePropMut<G, bool>,
}

impl<'a, G> Iterator for EdgesComplement<'a, G>
where
    G: 'a + WithEdge + WithEdgeProp<bool>,
{
    type Item = Edge<G>;

    fn next(&mut self) -> Option<Self::Item> {
        for e in self.edges.by_ref() {
            if !self.set[e] {
                return Some(e);
            }
        }
        None
    }
}

pub struct IndependentVertexSetFromIter<'a, G, I>
where
    G: 'a + Adjacency + WithVertexProp<bool>,
{
    g: &'a G,
    vertices: I,
    marked: DefaultVertexPropMut<G, bool>,
}

impl<'a, G, I> Iterator for IndependentVertexSetFromIter<'a, G, I>
where
    G: 'a + Adjacency + WithVertexProp<bool>,
    I: Iterator,
    I::Item: IntoOwned<Vertex<G>>,
{
    type Item = Vertex<G>;

    fn next(&mut self) -> Option<Self::Item> {
        for v in self.vertices.by_ref() {
            let v = v.into_owned();
            if self.marked[v] {
                continue;
            }
            self.marked[v] = true;
            self.marked.set_values(self.g.out_neighbors(v), true);
            return Some(v);
        }
        None
    }
}
