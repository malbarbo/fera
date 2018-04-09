// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Random selection of vertices and edges.
//!
//! # Examples
//!
//! ```
//! extern crate rand;
//! extern crate fera_graph;
//!
//! use fera_graph::prelude::*;
//! use fera_graph::choose::Choose;
//!
//! # fn main() {
//! let g = CompleteGraph::new(5);
//! let iter = g.choose_vertex_iter(rand::weak_rng()).take(100);
//!
//! let mut saw = g.default_vertex_prop(false);
//! for v in iter {
//!     saw[v] = true;
//! }
//! // or saw.set_values(iter, true);
//!
//! // The probability of this test failing is left as an exercise for the reader.
//! assert!(g.vertices().all(|v| saw[v]));
//! # }
//! ```
use prelude::*;

use rand::Rng;

// TODO: specialization of *_iter
// TODO: remove WithEdge bound and add bounds to methods
// TODO: ex: g.choose().vertex(), g.choose_with_rng(rng).vertices()
/// A graph from which vertices and edges can be randomly selected.
///
/// See the [module documentation] for examples.
///
/// [module documentation]: index.html
pub trait Choose: WithEdge {
    /// Returns a random vertex of this graph or `None` if the graph has no vertices.
    fn choose_vertex<R: Rng>(&self, rng: R) -> Option<Vertex<Self>>;

    /// Returns an iterator that repeatedly calls `choose_vertex`.
    fn choose_vertex_iter<R: Rng>(&self, rng: R) -> ChooseVertexIter<Self, R> {
        ChooseVertexIter { g: self, rng: rng }
    }

    /// Returns a random neighbor vertex of `v` or `None` if `v` has no neighbors.
    fn choose_out_neighbor<R: Rng>(&self, v: Vertex<Self>, rng: R) -> Option<Vertex<Self>>;

    /// Returns an iterator that repeatedly calls `choose_out_neighbor(v)`.
    fn choose_out_neighbor_iter<R: Rng>(
        &self,
        v: Vertex<Self>,
        rng: R,
    ) -> ChooseOutNeighborIter<Self, R> {
        ChooseOutNeighborIter {
            g: self,
            v: v,
            rng: rng,
        }
    }

    /// Returns a random edge of this graph or `None` if the graph has no edges.
    fn choose_edge<R: Rng>(&self, rng: R) -> Option<Edge<Self>>;

    /// Returns an iterator that repeatedly calls `choose_edge`.
    fn choose_edge_iter<R: Rng>(&self, rng: R) -> ChooseEdgeIter<Self, R> {
        ChooseEdgeIter { g: self, rng: rng }
    }

    /// Returns a random out edge of `v` or `None` if `v` has no out edges.
    fn choose_out_edge<R: Rng>(&self, v: Vertex<Self>, rng: R) -> Option<Edge<Self>>;

    /// Returns an iterator that repeatedly calls `choose_out_edge(v)`.
    fn choose_out_edge_iter<R: Rng>(&self, v: Vertex<Self>, rng: R) -> ChooseOutEdgeIter<Self, R> {
        ChooseOutEdgeIter {
            g: self,
            v: v,
            rng: rng,
        }
    }

    /// Returns a iterator that produces a sequence of random edges that forms a walk, that is, the
    /// target vertex of the previous edge is the source vertex of the next edge.
    fn random_walk<R: Rng>(&self, mut rng: R) -> RandomWalk<Self, R> {
        let cur = self.choose_vertex(&mut rng);
        RandomWalk {
            g: self,
            cur: cur,
            rng: rng,
        }
    }
}

/// An iterator that produces random selected vertices of a graph.
///
/// This `struct` is created by [`Choose::choose_vertex_iter`].
///
/// [`Choose::choose_vertex_iter`]: trait.Choose.html#method.choose_vertex_iter
pub struct ChooseVertexIter<'a, G: 'a, R> {
    g: &'a G,
    rng: R,
}

impl<'a, G, R> Iterator for ChooseVertexIter<'a, G, R>
where
    G: 'a + Choose,
    R: Rng,
{
    type Item = Vertex<G>;

    fn next(&mut self) -> Option<Vertex<G>> {
        G::choose_vertex(self.g, &mut self.rng)
    }
}

/// An iterator that produces random selected neighbors of a vertex.
///
/// This `struct` is created by [`Choose::choose_out_neighbor_iter`].
///
/// [`Choose::choose_out_neighbor_iter`]: trait.Choose.html#method.choose_out_neighbor_iter
pub struct ChooseOutNeighborIter<'a, G: 'a + WithVertex, R> {
    g: &'a G,
    v: Vertex<G>,
    rng: R,
}

impl<'a, G, R> Iterator for ChooseOutNeighborIter<'a, G, R>
where
    G: 'a + Choose,
    R: Rng,
{
    type Item = Vertex<G>;

    fn next(&mut self) -> Option<Vertex<G>> {
        G::choose_out_neighbor(self.g, self.v, &mut self.rng)
    }
}

/// An iterator that produces random selected edges of a graph.
///
/// This `struct` is created by [`Choose::choose_edge_iter`].
///
/// [`Choose::choose_edge_iter`]: trait.Choose.html#method.choose_edge_iter
pub struct ChooseEdgeIter<'a, G: 'a, R> {
    g: &'a G,
    rng: R,
}

impl<'a, G, R> Iterator for ChooseEdgeIter<'a, G, R>
where
    G: 'a + Choose,
    R: Rng,
{
    type Item = Edge<G>;

    fn next(&mut self) -> Option<Edge<G>> {
        G::choose_edge(self.g, &mut self.rng)
    }
}

/// An iterator that produces random selected out edges of a vertex.
///
/// This `struct` is created by [`Choose::choose_out_edge_iter`].
///
/// [`Choose::choose_out_edge_iter`]: trait.Choose.html#method.choose_out_edge_iter
pub struct ChooseOutEdgeIter<'a, G: 'a + WithVertex, R> {
    g: &'a G,
    v: Vertex<G>,
    rng: R,
}

impl<'a, G, R> Iterator for ChooseOutEdgeIter<'a, G, R>
where
    G: 'a + Choose,
    R: Rng,
{
    type Item = Edge<G>;

    fn next(&mut self) -> Option<Edge<G>> {
        G::choose_out_edge(self.g, self.v, &mut self.rng)
    }
}

/// An iterator that produces a sequence of edges that forms a walk.
///
/// This `struct` is created by [`Choose::random_walk`].
///
/// [`Choose::random_walk`]: trait.Choose.html#method.random_walk
pub struct RandomWalk<'a, G: 'a + WithVertex, R> {
    g: &'a G,
    cur: Option<Vertex<G>>,
    rng: R,
}

impl<'a, G: 'a + WithVertex, R> RandomWalk<'a, G, R> {
    pub fn start(mut self, v: Vertex<G>) -> Self {
        self.cur = Some(v);
        self
    }
}

impl<'a, G, R> Iterator for RandomWalk<'a, G, R>
where
    G: 'a + Choose,
    R: Rng,
{
    type Item = Edge<G>;

    fn next(&mut self) -> Option<Self::Item> {
        self.cur.and_then(|cur| {
            if let Some(e) = self.g.choose_out_edge(cur, &mut self.rng) {
                self.cur = Some(self.g.target(e));
                Some(e)
            } else {
                self.cur = None;
                None
            }
        })
    }
}

// TODO: write tests
