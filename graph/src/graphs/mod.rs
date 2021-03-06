// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Graph traits and implementations.

pub mod adaptors;
pub mod adjset;
pub mod complete;
pub mod static_;

mod common;
mod ref_;

pub use self::common::*;

use params::IntoOwned;
use prelude::*;

use std::fmt::Debug;
use std::hash::Hash;

pub type Vertex<G> = <G as WithVertex>::Vertex;
pub type OptionVertex<G> = <G as WithVertex>::OptionVertex;
pub type VertexIndexProp<G> = <G as WithVertexIndexProp>::VertexIndexProp;
pub type VertexIter<'a, G> = <G as VertexTypes<'a, G>>::VertexIter;
pub type OutNeighborIter<'a, G> = <G as VertexTypes<'a, G>>::OutNeighborIter;
pub type DefaultVertexPropMut<G, T> = <G as WithVertexProp<T>>::VertexProp;

pub type Edge<G> = <G as WithEdge>::Edge;
pub type OptionEdge<G> = <G as WithEdge>::OptionEdge;
pub type EdgeIndexProp<G> = <G as WithEdgeIndexProp>::EdgeIndexProp;
pub type EdgeIter<'a, G> = <G as EdgeTypes<'a, G>>::EdgeIter;
pub type OutEdgeIter<'a, G> = <G as EdgeTypes<'a, G>>::OutEdgeIter;
pub type DefaultEdgePropMut<G, T> = <G as WithEdgeProp<T>>::EdgeProp;

macro_rules! items {
    ($($item:item)*) => ($($item)*);
}

macro_rules! trait_alias {
    ($name:ident = $($base:tt)+) => {
        items! {
            pub trait $name: $($base)+ { }
            impl<T: $($base)+> $name for T { }
        }
    };
}

trait_alias!(Graph = VertexList + EdgeList<Kind = Undirected> + BasicProps);
trait_alias!(AdjacencyGraph = Graph + Adjacency);
trait_alias!(IncidenceGraph = AdjacencyGraph + Incidence);

trait_alias!(Digraph = VertexList + EdgeList<Kind = Directed> + BasicProps);
trait_alias!(AdjacencyDigraph = Digraph + Adjacency);
trait_alias!(IncidenceDigraph = AdjacencyDigraph + Incidence);

trait_alias!(GraphItem = Copy + Eq + Hash + Debug);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Orientation {
    Directed,
    Undirected,
}

impl Orientation {
    #[inline]
    pub fn is_directed(&self) -> bool {
        *self == Orientation::Directed
    }

    #[inline]
    pub fn is_undirected(&self) -> bool {
        *self == Orientation::Undirected
    }
}

pub trait EdgeKind {}

pub trait UniformEdgeKind: EdgeKind {
    fn orientation() -> Orientation;

    #[inline]
    fn is_directed() -> bool {
        Self::orientation().is_directed()
    }

    #[inline]
    fn is_undirected() -> bool {
        Self::orientation().is_undirected()
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Directed {}

impl EdgeKind for Directed {}

impl UniformEdgeKind for Directed {
    #[inline]
    fn orientation() -> Orientation {
        Orientation::Directed
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Undirected {}

impl EdgeKind for Undirected {}

impl UniformEdgeKind for Undirected {
    #[inline]
    fn orientation() -> Orientation {
        Orientation::Undirected
    }
}

// TODO: write a graph with mixed edges and test it
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Mixed {}

impl EdgeKind for Mixed {}

pub trait VertexTypes<'a, G: WithVertex> {
    type VertexIter: Iterator<Item = Vertex<G>>;
    type OutNeighborIter: Iterator<Item = Vertex<G>>;
}

pub trait WithVertex: Sized + for<'a> VertexTypes<'a, Self> {
    type Vertex: 'static + GraphItem;
    type OptionVertex: 'static + GraphItem + Optional<Vertex<Self>> + From<Option<Vertex<Self>>>;

    // TODO: is this necessary?
    fn vertex_none() -> OptionVertex<Self> {
        Default::default()
    }

    // TODO: is this necessary?
    fn vertex_some(v: Vertex<Self>) -> OptionVertex<Self> {
        From::from(v)
    }

    fn vertex_prop<P, T>(&self, value: T) -> P
    where
        P: VertexPropMutNew<Self, T>,
        T: Clone,
    {
        P::new_vertex_prop(self, value)
    }

    fn vertex_prop_from_fn<P, T, F>(&self, mut fun: F) -> P
    where
        Self: VertexList,
        P: VertexPropMutNew<Self, T>,
        F: FnMut(Vertex<Self>) -> T,
        T: Default + Clone,
    {
        // FIXME: Can we remove T: Default + Clone?
        let mut p: P = self.vertex_prop(T::default());
        for v in self.vertices() {
            p[v] = fun(v);
        }
        p
    }
}

pub trait EdgeTypes<'a, G: WithEdge> {
    type EdgeIter: Iterator<Item = Edge<G>>;
    type OutEdgeIter: Iterator<Item = Edge<G>>;
}

pub trait WithEdge: Sized + WithVertex + for<'a> EdgeTypes<'a, Self> {
    type Kind: EdgeKind;
    type Edge: 'static + GraphItem;
    type OptionEdge: 'static + GraphItem + Optional<Edge<Self>> + From<Option<Edge<Self>>>;

    fn orientation(&self, _e: Edge<Self>) -> Orientation;

    fn source(&self, e: Edge<Self>) -> Vertex<Self>;

    fn target(&self, e: Edge<Self>) -> Vertex<Self>;

    fn ends<'a, I, O>(&'a self, item: I) -> O
    where
        I: Ends<'a, Self, O>,
    {
        item._ends(self)
    }

    fn end_vertices(&self, e: Edge<Self>) -> (Vertex<Self>, Vertex<Self>) {
        self.ends(e)
    }

    fn with_ends<I>(&self, iter: I) -> EdgesWithEnds<Self, I::IntoIter>
    where
        I: IntoIterator,
        I::Item: IntoOwned<Edge<Self>>,
    {
        EdgesWithEnds {
            g: self,
            iter: iter.into_iter(),
        }
    }

    fn opposite(&self, u: Vertex<Self>, e: Edge<Self>) -> Vertex<Self> {
        let (s, t) = self.ends(e);
        if u == s {
            t
        } else if u == t {
            s
        } else {
            panic!("u is not an end of e");
        }
    }

    fn is_incident(&self, v: Vertex<Self>, e: Edge<Self>) -> bool {
        let (s, t) = self.ends(e);
        v == s || v == t
    }

    fn reverse(&self, e: Edge<Self>) -> Edge<Self>
    where
        Self: WithEdge<Kind = Undirected>,
    {
        self.get_reverse(e)
            .expect("the reverse of an edge (all undirected graphs must implement reverse)")
    }

    fn get_reverse(&self, _e: Edge<Self>) -> Option<Edge<Self>> {
        // TODO: remove default impl
        None
    }

    // TODO: is this necessary?
    fn edge_none() -> OptionEdge<Self> {
        Default::default()
    }

    // TODO: is this necessary?
    fn edge_some(e: Edge<Self>) -> OptionEdge<Self> {
        From::from(e)
    }

    fn edge_prop<P, T>(&self, value: T) -> P
    where
        P: EdgePropMutNew<Self, T>,
        T: Clone,
    {
        P::new_edge_prop(self, value)
    }

    fn edge_prop_from_fn<P, F, T>(&self, mut fun: F) -> P
    where
        Self: EdgeList,
        P: EdgePropMutNew<Self, T>,
        F: FnMut(Edge<Self>) -> T,
        T: Default + Clone,
    {
        // FIXME: Can we remove T: Default + Clone?
        let mut p: P = self.edge_prop(T::default());
        for e in self.edges() {
            p[e] = fun(e);
        }
        p
    }
}

pub trait VertexList: Sized + WithVertex {
    fn vertices(&self) -> VertexIter<Self>;

    fn num_vertices(&self) -> usize {
        self.vertices().count()
    }
}

pub trait EdgeList: Sized + WithEdge {
    fn edges(&self) -> EdgeIter<Self>;

    fn edges_ends(&self) -> EdgesEnds<Self, EdgeIter<Self>> {
        // TODO: specialize
        self.ends(self.edges())
    }

    fn edges_with_ends(&self) -> EdgesWithEnds<Self, EdgeIter<Self>> {
        // TODO: specialize
        self.with_ends(self.edges())
    }

    fn num_edges(&self) -> usize {
        self.edges().count()
    }

    fn get_edge_by_ends(&self, u: Vertex<Self>, v: Vertex<Self>) -> Option<Edge<Self>> {
        // TODO: specialize for Incidence
        for (e, a, b) in self.edges_with_ends() {
            if (u, v) == (a, b) {
                return Some(e);
            }
            if self.orientation(e).is_directed() {
                continue;
            }
            if let Some(e) = self.get_reverse(e) {
                if (u, v) == (b, a) {
                    return Some(e);
                }
            }
        }
        None
    }

    fn edge_by_ends(&self, u: Vertex<Self>, v: Vertex<Self>) -> Edge<Self> {
        // TODO: fix expect message
        self.get_edge_by_ends(u, v).expect("an edge (u, v)")
    }
}

pub trait Adjacency: WithVertex {
    fn out_neighbors(&self, v: Vertex<Self>) -> OutNeighborIter<Self>;

    fn out_degree(&self, v: Vertex<Self>) -> usize {
        self.out_neighbors(v).count()
    }
}

pub trait Incidence: WithEdge + Adjacency {
    fn out_edges(&self, v: Vertex<Self>) -> OutEdgeIter<Self>;

    fn out_edges_ends(&self, v: Vertex<Self>) -> EdgesEnds<Self, OutEdgeIter<Self>> {
        // TODO: specialize
        self.ends(self.out_edges(v))
    }

    fn out_edges_with_ends(&self, v: Vertex<Self>) -> EdgesWithEnds<Self, OutEdgeIter<Self>> {
        // TODO: specialize
        self.with_ends(self.out_edges(v))
    }
}

// Ends

pub trait Ends<'a, G, O> {
    fn _ends(self, g: &'a G) -> O;
}

impl<'a, G> Ends<'a, G, (Vertex<G>, Vertex<G>)> for Edge<G>
where
    G: WithEdge,
{
    fn _ends(self, g: &'a G) -> (Vertex<G>, Vertex<G>) {
        (g.source(self), g.target(self))
    }
}

impl<'a, G> Ends<'a, G, (Edge<G>, Vertex<G>, Vertex<G>)> for Edge<G>
where
    G: WithEdge,
{
    fn _ends(self, g: &'a G) -> (Edge<G>, Vertex<G>, Vertex<G>) {
        let (u, v) = g.ends(self);
        (self, u, v)
    }
}

impl<'a, G, I> Ends<'a, G, EdgesEnds<'a, G, I::IntoIter>> for I
where
    G: WithEdge,
    I: IntoIterator,
    I::Item: IntoOwned<Edge<G>>,
{
    fn _ends(self, g: &'a G) -> EdgesEnds<'a, G, I::IntoIter> {
        EdgesEnds {
            g: g,
            iter: self.into_iter(),
        }
    }
}

// Iterators

pub struct EdgesEnds<'a, G: 'a, I> {
    g: &'a G,
    iter: I,
}

impl<'a, G, I> Iterator for EdgesEnds<'a, G, I>
where
    G: WithEdge,
    I: Iterator,
    I::Item: IntoOwned<Edge<G>>,
{
    type Item = (Vertex<G>, Vertex<G>);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|e| self.g.ends(e.into_owned()))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, G, I> ExactSizeIterator for EdgesEnds<'a, G, I>
where
    G: WithEdge,
    I: Iterator,
    I::Item: IntoOwned<Edge<G>>,
{
}

pub struct EdgesWithEnds<'a, G: 'a, I> {
    g: &'a G,
    iter: I,
}

impl<'a, G, I> Iterator for EdgesWithEnds<'a, G, I>
where
    G: WithEdge,
    I: Iterator,
    I::Item: IntoOwned<Edge<G>>,
{
    type Item = (Edge<G>, Vertex<G>, Vertex<G>);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|e| self.g.ends(e.into_owned()))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, G, I> ExactSizeIterator for EdgesWithEnds<'a, G, I>
where
    G: WithEdge,
    I: Iterator,
    I::Item: IntoOwned<Edge<G>>,
{
}
