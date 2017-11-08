// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Test if a graph is regular, find minimum and maximum degrees, etc.

use prelude::*;
use params::IntoOwned;

pub trait Degrees: Adjacency {
    fn degree_spanning_subgraph<I>(&self, edges: I) -> DefaultVertexPropMut<Self, u32>
    where
        Self: WithEdge + WithVertexProp<u32>,
        I: IntoIterator,
        I::Item: IntoOwned<Edge<Self>>,
    {
        let mut degree = self.default_vertex_prop(0);
        self.degree_add(&mut degree, edges);
        degree
    }

    fn degree_add<P, I>(&self, degree: &mut P, edges: I)
    where
        Self: WithEdge,
        P: VertexPropMut<Self, u32>,
        I: IntoIterator,
        I::Item: IntoOwned<Edge<Self>>,
    {
        for e in edges {
            let (u, v) = self.end_vertices(e.into_owned());
            degree[u] += 1;
            degree[v] += 1;
        }
    }

    fn is_k_regular(&self, k: usize) -> bool
    where
        Self: WithEdge<Kind = Undirected> + VertexList,
    {
        self.vertices().all(|v| self.out_degree(v) == k)
    }

    fn is_regular(&self) -> bool
    where
        Self: WithEdge<Kind = Undirected> + VertexList,
    {
        let mut vertices = self.vertices();

        if let Some(v) = vertices.next() {
            let deg = self.out_degree(v);
            vertices.all(|v| self.out_degree(v) == deg)
        } else {
            true
        }
    }

    fn maximum_out_degree(&self) -> Option<usize>
    where
        Self: VertexList,
    {
        self.vertices().map(|v| self.out_degree(v)).max()
    }

    fn minimum_out_degree(&self) -> Option<usize>
    where
        Self: VertexList,
    {
        self.vertices().map(|v| self.out_degree(v)).min()
    }

    fn is_isolated(&self, v: Vertex<Self>) -> bool
    where
        Self: WithEdge<Kind = Undirected>,
    {
        self.out_degree(v) == 0
    }

    fn is_pendant(&self, v: Vertex<Self>) -> bool
    where
        Self: WithEdge<Kind = Undirected>,
    {
        self.out_degree(v) == 1
    }
}

impl<G: Adjacency> Degrees for G {}
