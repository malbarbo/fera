// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use prelude::*;
use choose::Choose;
use graphs::OutNeighborFromOutEdge;
use params::IntoOwned;
use props::{DelegateEdgeProp, DelegateVertexProp};

use fera_fun::vec;

use std::iter::Cloned;
use std::slice;

use rand::Rng;

// TODO: Allow a subgraph be reused
// TODO: delegate all (possible) methods to g
// TODO: remove Graph bound to allow directed graphs
pub struct Subgraph<'a, G>
    where G: 'a + Graph
{
    g: &'a G,
    vertices: Vec<Vertex<G>>,
    edges: Vec<Edge<G>>,
    inc: DefaultVertexPropMut<G, Vec<Edge<G>>>,
}


// Traits implementations

impl<'a, G> AsRef<G> for Subgraph<'a, G>
    where G: 'a + Graph
{
    #[inline]
    fn as_ref(&self) -> &G {
        self.g
    }
}

impl<'a, 'b, G> VertexTypes<'a, Subgraph<'b, G>> for Subgraph<'b, G>
    where G: 'b + Graph
{
    type VertexIter = Cloned<slice::Iter<'a, Vertex<G>>>;
    type OutNeighborIter = OutNeighborFromOutEdge<'b, G, OutEdgeIter<'a, Self>>;
}

impl<'a, G> WithVertex for Subgraph<'a, G>
    where G: 'a + Graph
{
    type Vertex = Vertex<G>;
    type OptionVertex = OptionVertex<G>;
}

impl<'a, 'b, G> EdgeTypes<'a, Subgraph<'b, G>> for Subgraph<'b, G>
    where G: 'b + Graph
{
    type EdgeIter = Cloned<slice::Iter<'a, Edge<G>>>;
    type OutEdgeIter = Cloned<slice::Iter<'a, Edge<G>>>;
}

impl<'a, G> WithEdge for Subgraph<'a, G>
    where G: 'a + Graph
{
    type Kind = G::Kind;
    type Edge = Edge<G>;
    type OptionEdge = OptionEdge<G>;

    fn source(&self, e: Edge<Self>) -> Vertex<Self> {
        self.g.source(e)
    }

    fn target(&self, e: Edge<Self>) -> Vertex<Self> {
        self.g.target(e)
    }

    fn orientation(&self, e: Edge<Self>) -> Orientation {
        self.g.orientation(e)
    }

    fn end_vertices(&self, e: Edge<Self>) -> (Vertex<Self>, Vertex<Self>) {
        self.g.end_vertices(e)
    }

    fn opposite(&self, u: Vertex<Self>, e: Edge<Self>) -> Vertex<Self> {
        self.g.opposite(u, e)
    }

    fn reverse(&self, e: Edge<Self>) -> Edge<Self> {
        self.g.reverse(e)
    }

    fn get_reverse(&self, e: Edge<Self>) -> Option<Edge<Self>> {
        self.g.get_reverse(e)
    }
}

impl<'a, G> VertexList for Subgraph<'a, G>
    where G: 'a + Graph
{
    fn num_vertices(&self) -> usize {
        self.vertices.len()
    }

    fn vertices(&self) -> VertexIter<Self> {
        self.vertices.iter().cloned()
    }
}

impl<'a, G> EdgeList for Subgraph<'a, G>
    where G: 'a + Graph
{
    fn num_edges(&self) -> usize {
        self.edges.len()
    }

    fn edges(&self) -> EdgeIter<Self> {
        self.edges.iter().cloned()
    }

    fn get_edge_by_ends(&self, u: Vertex<Self>, v: Vertex<Self>) -> Option<Edge<Self>> {
        self.out_edges(u).find(|e| (u, v) == self.ends(*e))
    }
}

impl<'a, G> Adjacency for Subgraph<'a, G>
    where G: 'a + Graph
{
    fn out_neighbors(&self, v: Vertex<Self>) -> OutNeighborIter<Self> {
        OutNeighborFromOutEdge::new(self.g, self.out_edges(v))
    }

    fn out_degree(&self, v: Vertex<Self>) -> usize {
        self.inc[v].len()
    }
}

impl<'a, G> Incidence for Subgraph<'a, G>
    where G: 'a + Graph
{
    fn out_edges(&self, v: Vertex<Self>) -> OutEdgeIter<Self> {
        self.inc[v].iter().cloned()
    }
}

impl<'a, G, T> WithVertexProp<T> for Subgraph<'a, G>
    where G: 'a + Graph + WithVertexProp<T>
{
    type VertexProp = DelegateVertexProp<G, T>;
}

impl<'a, G, T> WithEdgeProp<T> for Subgraph<'a, G>
    where G: 'a + Graph + WithEdgeProp<T>
{
    type EdgeProp = DelegateEdgeProp<G, T>;
}

impl<'a, G> BasicVertexProps for Subgraph<'a, G> where G: 'a + Graph {}

impl<'a, G> BasicEdgeProps for Subgraph<'a, G> where G: 'a + Graph {}

impl<'a, G> BasicProps for Subgraph<'a, G> where G: 'a + Graph {}


// Choose

impl<'a, G> Choose for Subgraph<'a, G>
    where G: 'a + IncidenceGraph
{
    fn choose_vertex<R: Rng>(&self, mut rng: R) -> Option<Vertex<Self>> {
        self.vertices.get(rng.gen_range(0, self.num_vertices())).cloned()
    }

    fn choose_out_neighbor<R: Rng>(&self, v: Vertex<Self>, mut rng: R) -> Option<Vertex<Self>> {
        self.inc[v].get(rng.gen_range(0, self.out_degree(v))).map(|e| self.target(*e))
    }

    fn choose_edge<R: Rng>(&self, mut rng: R) -> Option<Edge<Self>> {
        if self.num_edges() == 0 {
            None
        } else {
            // TODO: choose to reverse undirected edges?
            self.edges.get(rng.gen_range(0, self.num_edges())).cloned()
        }
    }

    fn choose_out_edge<R: Rng>(&self, v: Vertex<Self>, mut rng: R) -> Option<Edge<Self>> {
        if self.out_degree(v) == 0 {
            None
        } else {
            self.inc[v].get(rng.gen_range(0, self.out_degree(v))).cloned()
        }
    }
}


// Extensions Traits

pub trait WithSubgraph<G: Graph> {
    fn empty_spanning_subgraph(&self) -> SpanningSubgraph<G>;

    fn spanning_subgraph<I>(&self, vertices: I) -> SpanningSubgraph<G>
        where I: IntoIterator,
              I::Item: IntoOwned<Edge<G>>;

    fn induced_subgraph<I>(&self, vertices: I) -> Subgraph<G>
        where G: Incidence,
              I: IntoIterator,
              I::Item: IntoOwned<Vertex<G>>;

    fn edge_induced_subgraph<I>(&self, edges: I) -> Subgraph<G>
        where I: IntoIterator,
              I::Item: IntoOwned<Edge<G>>;
}


impl<G: Graph> WithSubgraph<G> for G {
    fn spanning_subgraph<I>(&self, iter: I) -> SpanningSubgraph<G>
        where I: IntoIterator,
              I::Item: IntoOwned<Edge<G>>
    {
        let mut sub = SpanningSubgraph::new(self);
        sub.add_edges(iter);
        sub
    }

    fn empty_spanning_subgraph(&self) -> SpanningSubgraph<G> {
        SpanningSubgraph::new(self)
    }

    fn edge_induced_subgraph<I>(&self, edges: I) -> Subgraph<G>
        where I: IntoIterator,
              I::Item: IntoOwned<Edge<G>>
    {
        // FIXME: should be O(edges), but is O(V) + O(edges)
        let edges = vec(edges.into_iter().map(IntoOwned::into_owned));
        let mut vin = self.default_vertex_prop(false);
        let mut vertices = vec![];
        let mut inc = self.default_vertex_prop(Vec::<Edge<G>>::new());
        for (e, u, v) in self.with_ends(&edges) {
            if !vin[u] {
                vin[u] = true;
                vertices.push(u);
            }
            if !vin[v] {
                vin[v] = true;
                vertices.push(v);
            }
            inc[u].push(e);
            inc[v].push(self.reverse(e));
        }

        Subgraph {
            g: self,
            vertices: vertices,
            edges: edges,
            inc: inc,
        }
    }

    fn induced_subgraph<I>(&self, vertices: I) -> Subgraph<G>
        where G: Incidence,
              I: IntoIterator,
              I::Item: IntoOwned<Vertex<G>>
    {
        let vertices = vec(vertices.into_iter().map(IntoOwned::into_owned));
        let mut vs = self.default_vertex_prop(false);
        let mut edges = vec![];
        let mut inc = self.default_vertex_prop(Vec::<Edge<G>>::new());
        for &v in &vertices {
            vs[v] = true;
        }
        for (e, u, v) in self.edges_with_ends() {
            if vs[u] && vs[v] {
                edges.push(e);
                inc[u].push(e);
                inc[v].push(self.reverse(e));
            }
        }

        Subgraph {
            g: self,
            vertices: vertices,
            edges: edges,
            inc: inc,
        }
    }
}


// TODO: write benchs and optimize

#[cfg(test)]
mod tests {
    use prelude::*;
    use fera_fun::{set, vec};

    fn new_graph
        ()
        -> (StaticGraph, Edge<StaticGraph>, Edge<StaticGraph>, Edge<StaticGraph>, Edge<StaticGraph>)
    {
        let g: StaticGraph = graph!(5, (0, 1), (0, 2), (1, 2), (3, 4));
        let e = vec(g.edges());
        (g, e[0], e[1], e[2], e[3])
    }

    #[test]
    fn test_spanning_subgraph() {
        let (g, _, e02, e12, _) = new_graph();
        let s = g.spanning_subgraph(vec![e02, e12]);
        assert_eq!(vec![0, 1, 2, 3, 4], vec(s.vertices()));
        assert_eq!(set(vec![e02, e12]), set(s.edges()));
        assert_eq!(set(vec![e02]), set(s.out_edges(0)));
        assert_eq!(set(vec![e12]), set(s.out_edges(1)));
        assert_eq!(set(vec![e02, e12]), set(s.out_edges(2)));
        assert!(set(s.out_edges(3)).is_empty());
        assert!(set(s.out_edges(4)).is_empty());
    }

    #[test]
    fn test_edge_induced_subgraph() {
        let (g, e01, e02, _, _) = new_graph();
        let s = g.edge_induced_subgraph(vec![e01, e02]);
        assert_eq!(set(vec![0, 1, 2]), set(s.vertices()));
        assert_eq!(set(vec![e01, e02]), set(s.edges()));
        assert_eq!(set(vec![e01, e02]), set(s.out_edges(0)));
        assert_eq!(set(vec![1, 2]), set(s.out_neighbors(0)));
        assert_eq!(set(vec![e01]), set(s.out_edges(1)));
        assert_eq!(set(vec![0]), set(s.out_neighbors(1)));
        assert_eq!(set(vec![e02]), set(s.out_edges(2)));
        assert_eq!(set(vec![0]), set(s.out_neighbors(2)));
    }

    #[test]
    fn test_induced_subgraph() {
        let (g, e01, e02, e12, _) = new_graph();
        let s = g.induced_subgraph(vec![0, 1, 2]);
        assert_eq!(set(vec![0, 1, 2]), set(s.vertices()));
        assert_eq!(set(vec![e01, e02, e12]), set(s.edges()));
        assert_eq!(set(vec![e01, e02]), set(s.out_edges(0)));
        assert_eq!(set(vec![e01, e12]), set(s.out_edges(1)));
        assert_eq!(set(vec![e02, e12]), set(s.out_edges(2)));
    }
}
