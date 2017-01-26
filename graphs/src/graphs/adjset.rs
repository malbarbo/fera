use prelude::*;
use props::HashMapProp;

use std::collections::hash_map;
use std::collections::{HashMap, HashSet};
use std::collections::hash_set;
use std::hash::BuildHasherDefault;
use std::hash::{Hash, Hasher};
use std::iter::Cloned;
use std::marker::PhantomData;
use fnv::FnvHasher;

pub type AdjSetGraph<V> = AdjSet<V, Undirected>;
pub type AdjSetDiGraph<V> = AdjSet<V, Directed>;

type HashMapFnv<K, V> = HashMap<K, V, BuildHasherDefault<FnvHasher>>;
type HashSetFnv<K> = HashSet<K, BuildHasherDefault<FnvHasher>>;

pub struct AdjSet<V: AdjSetVertex, K: AdjSetEdgeKind<V>> {
    adj: HashMapFnv<V, HashSetFnv<V>>,
    num_edges: usize,
    _marker: PhantomData<K>,
}

pub trait AdjSetEdgeKind<V: AdjSetVertex>: UniformEdgeKind {
    type Edge: AdjSetEdge<V>;
}

pub trait AdjSetVertex: 'static + GraphItem + PartialOrd {}

impl<V: 'static + GraphItem + PartialOrd> AdjSetVertex for V {}

pub trait AdjSetEdge<V>: 'static + GraphItem
    where V: AdjSetVertex
{
    fn new(u: V, v: V) -> Self;

    fn source(&self) -> V;

    fn target(&self) -> V;
}


// Undirected

#[derive(Copy, Clone, Eq, Debug, PartialOrd, Ord)]
pub struct UndirectedEdge<V>(V, V);

impl<V: PartialEq> PartialEq for UndirectedEdge<V> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 || self.1 == other.0 && self.0 == other.1
    }
}

impl<V: PartialEq> PartialEq<(V, V)> for UndirectedEdge<V> {
    fn eq(&self, other: &(V, V)) -> bool {
        self.0 == other.0 && self.1 == other.1 || self.1 == other.0 && self.0 == other.1
    }
}

impl<V: PartialOrd + Hash> Hash for UndirectedEdge<V> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if self.0 < self.1 {
            self.0.hash(state);
            self.1.hash(state);
        } else {
            self.1.hash(state);
            self.0.hash(state);
        }
    }
}

impl<V> AdjSetEdge<V> for UndirectedEdge<V>
    where V: AdjSetVertex + PartialOrd
{
    fn new(u: V, v: V) -> Self {
        UndirectedEdge(u, v)
    }

    fn source(&self) -> V {
        self.0
    }

    fn target(&self) -> V {
        self.1
    }
}

impl<V> AdjSetEdgeKind<V> for Undirected
    where V: AdjSetVertex + PartialOrd
{
    type Edge = UndirectedEdge<V>;
}


// Directed

pub type DirectedEdge<V> = (V, V);

impl<V> AdjSetEdge<V> for DirectedEdge<V>
    where V: AdjSetVertex
{
    fn new(u: V, v: V) -> Self {
        (u, v)
    }

    fn source(&self) -> V {
        self.0
    }

    fn target(&self) -> V {
        self.1
    }
}

impl<V> AdjSetEdgeKind<V> for Directed
    where V: AdjSetVertex
{
    type Edge = DirectedEdge<V>;
}


// Graph traits implementation

impl<'a, V, K> VertexTypes<'a, AdjSet<V, K>> for AdjSet<V, K>
    where V: AdjSetVertex,
          K: AdjSetEdgeKind<V>
{
    type VertexIter = Cloned<hash_map::Keys<'a, V, HashSetFnv<V>>>;
    type OutNeighborIter = Cloned<hash_set::Iter<'a, V>>;
}

impl<V, K> WithVertex for AdjSet<V, K>
    where V: AdjSetVertex,
          K: AdjSetEdgeKind<V>
{
    type Vertex = V;
    type OptionVertex = Option<V>;
}

impl<'a, V, K> EdgeTypes<'a, AdjSet<V, K>> for AdjSet<V, K>
    where V: AdjSetVertex,
          K: AdjSetEdgeKind<V>
{
    type EdgeIter = Edges<'a, V, K>;
    type OutEdgeIter = OutEdges<'a, V, K>;
}

impl<V, K> WithEdge for AdjSet<V, K>
    where V: AdjSetVertex,
          K: AdjSetEdgeKind<V>
{
    type Kind = K;
    type Edge = K::Edge;
    type OptionEdge = Option<K::Edge>;

    fn source(&self, e: Edge<Self>) -> Vertex<Self> {
        e.source()
    }

    fn target(&self, e: Edge<Self>) -> Vertex<Self> {
        e.target()
    }

    fn orientation(&self, _e: Edge<Self>) -> Orientation {
        K::orientation()
    }

    fn get_reverse(&self, e: Edge<Self>) -> Option<Edge<Self>> {
        Some(K::Edge::new(e.target(), e.source()))
    }
}

impl<V, K> VertexList for AdjSet<V, K>
    where V: AdjSetVertex,
          K: AdjSetEdgeKind<V>
{
    fn vertices(&self) -> VertexIter<Self> {
        self.adj.keys().cloned()
    }

    fn num_vertices(&self) -> usize {
        self.adj.len()
    }
}

impl<V, K> EdgeList for AdjSet<V, K>
    where V: AdjSetVertex,
          K: AdjSetEdgeKind<V>
{
    fn edges(&self) -> EdgeIter<Self> {
        Edges {
            iter: self.adj.iter(),
            inner: None,
            _marker: PhantomData,
        }
    }

    fn num_edges(&self) -> usize {
        self.num_edges
    }
}

impl<V, K> Adjacency for AdjSet<V, K>
    where V: AdjSetVertex,
          K: AdjSetEdgeKind<V>
{
    fn out_neighbors(&self, v: Vertex<Self>) -> OutNeighborIter<Self> {
        self.out_neighbors_(v).iter().cloned()
    }

    fn out_degree(&self, v: Vertex<Self>) -> usize {
        self.out_neighbors_(v).len()
    }
}

impl<V, K> Incidence for AdjSet<V, K>
    where V: AdjSetVertex,
          K: AdjSetEdgeKind<V>
{
    fn out_edges(&self, v: Vertex<Self>) -> OutEdgeIter<Self> {
        OutEdges {
            source: v,
            adj: self.out_neighbors_(v).iter(),
            _marker: PhantomData,
        }
    }
}

impl<V, K> AdjSet<V, K>
    where V: AdjSetVertex,
          K: AdjSetEdgeKind<V>
{
    pub fn new() -> Self {
        AdjSet {
            adj: Default::default(),
            num_edges: 0,
            _marker: PhantomData,
        }
    }

    pub fn add_vertex(&mut self, v: V) {
        self.adj.entry(v).or_insert_with(Default::default);
    }

    pub fn add_edge(&mut self, u: V, v: V) -> K::Edge {
        if self.edge_by_ends(u, v).is_some() {
            panic!("Multiedge not supported");
        }

        // insert u and (u, v)
        self.adj.entry(u).or_insert_with(Default::default).insert(v);

        // insert v
        let mut entry = self.adj.entry(v).or_insert_with(Default::default);

        if K::is_undirected() {
            // insert (v, u)
            entry.insert(u);
        }

        self.num_edges += 1;

        K::Edge::new(u, v)
    }

    fn out_neighbors_(&self, v: Vertex<Self>) -> &HashSetFnv<V> {
        self.adj
            .get(&v)
            .unwrap_or_else(|| panic!("{:?} is not a valid vertex", v))
    }
}

impl<V, K> EdgeByEnds for AdjSet<V, K>
    where V: AdjSetVertex,
          K: AdjSetEdgeKind<V>
{
    fn edge_by_ends(&self, u: Vertex<Self>, v: Vertex<Self>) -> Option<Edge<Self>> {
        self.adj
            .get(&u)
            .and_then(|adj| adj.get(&v))
            .and_then(|_| Some(K::Edge::new(u, v)))
    }
}


// Props

impl<V, K, T> WithVertexProp<T> for AdjSet<V, K>
    where V: AdjSetVertex,
          K: AdjSetEdgeKind<V>,
          T: Clone
{
    type VertexProp = HashMapProp<V, T>;
}

impl<V, K, T> WithEdgeProp<T> for AdjSet<V, K>
    where V: AdjSetVertex,
          K: AdjSetEdgeKind<V>,
          T: Clone
{
    type EdgeProp = HashMapProp<K::Edge, T>;
}


// Iterators

pub struct Edges<'a, V, K>
    where V: AdjSetVertex,
          K: AdjSetEdgeKind<V>
{
    iter: hash_map::Iter<'a, V, HashSetFnv<V>>,
    inner: Option<(V, hash_set::Iter<'a, V>)>,
    _marker: PhantomData<K>,
}

impl<'a, V, K> Iterator for Edges<'a, V, K>
    where V: AdjSetVertex,
          K: AdjSetEdgeKind<V>
{
    type Item = K::Edge;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some((source, ref mut inner)) = self.inner {
                if let Some(target) = inner.next() {
                    if K::is_undirected() && source > *target {
                        continue;
                    }
                    return Some(K::Edge::new(source, *target));
                }
            }

            if let Some((source, inner)) = self.iter.next() {
                self.inner = Some((*source, inner.iter()));
            } else {
                return None;
            }
        }
    }

    // TODO: implements size_hint and ExactSizeIterator
}


pub struct OutEdges<'a, V, K>
    where V: AdjSetVertex,
          K: AdjSetEdgeKind<V>
{
    source: V,
    adj: hash_set::Iter<'a, V>,
    _marker: PhantomData<K>,
}

impl<'a, V, K> Iterator for OutEdges<'a, V, K>
    where V: AdjSetVertex,
          K: AdjSetEdgeKind<V>
{
    type Item = K::Edge;

    fn next(&mut self) -> Option<Self::Item> {
        let source = self.source;
        self.adj.next().map(|target| K::Edge::new(source, *target))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.adj.size_hint()
    }
}

impl<'a, V, K> ExactSizeIterator for OutEdges<'a, V, K>
    where V: AdjSetVertex,
          K: AdjSetEdgeKind<V>
{
    fn len(&self) -> usize {
        self.adj.len()
    }
}


#[cfg(test)]
mod tests {
    pub use super::*;
    pub use prelude::*;
    pub use tests::GraphTests;
    pub use fera_fun::vec;

    pub fn sorted<T: Clone + Ord>(xs: &[T]) -> Vec<T> {
        let mut v = xs.to_vec();
        v.sort();
        v
    }

    mod undirected {
        use super::*;
        struct Test;

        impl GraphTests for Test {
            type G = AdjSetGraph<u32>;

            fn new() -> (Self::G, VecVertex<Self::G>, VecEdge<Self::G>) {
                let e = UndirectedEdge::new;
                let mut g = AdjSet::new();
                g.add_edge(1, 2);
                g.add_edge(3, 4);
                g.add_edge(4, 1);
                g.add_edge(7, 4);
                let v = vec(g.vertices());
                let ee = vec(g.edges());
                assert_eq!(sorted(&v), vec![1, 2, 3, 4, 7]);
                assert_eq!(sorted(&ee), vec![e(1, 2), e(1, 4), e(3, 4), e(4, 7)]);
                (g, v, ee)
            }
        }

        graph_tests!{Test}
    }

    mod directed {
        use super::*;
        struct Test;

        impl GraphTests for Test {
            type G = AdjSetDiGraph<u32>;

            fn new() -> (Self::G, VecVertex<Self::G>, VecEdge<Self::G>) {
                let mut g = AdjSet::new();
                g.add_edge(1, 2);
                g.add_edge(3, 4);
                g.add_edge(4, 1);
                g.add_edge(7, 4);
                let v = vec(g.vertices());
                let ee = vec(g.edges());
                assert_eq!(sorted(&v), vec![1, 2, 3, 4, 7]);
                assert_eq!(sorted(&ee), vec![(1, 2), (3, 4), (4, 1), (7, 4)]);
                (g, v, ee)
            }
        }

        graph_tests!{Test}
    }
}
