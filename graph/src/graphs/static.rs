use choose::Choose;
use graphs::common::OutNeighborFromOutEdge;
use prelude::*;
use props::{VecEdgeProp, VecVertexProp, FnProp};

use fera_fun::vec;
use fera_optional::OptionalMax;

use std::cmp::Ordering;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::iter::{Cloned, Map};
use std::ops::Range;
use std::slice::Iter;

use rand::Rng;
use num_traits::Bounded;

pub type StaticDigraph = Static<u32, (Directed, usize)>;

pub type StaticGraph = Static<u32, (Undirected, usize)>;


// Edge

pub trait StaticEdgeKind: 'static {
    type Kind: UniformEdgeKind; // TODO: change to EdgeKind
    type Edge: 'static + GraphItem + Bounded + EdgeImpl;
}

#[doc(hidden)]
pub trait EdgeImpl: Sized {
    fn new(e: usize) -> Self;
    fn new_checked(e: usize) -> Option<Self>;
    fn source<T>(self, ends: &[T]) -> &T;
    fn target<T>(self, ends: &[T]) -> &T;
    fn to_index(self) -> usize;
    fn reverse(self) -> Self;
}


// StaticDirectedEdge

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StaticDirectedEdge<N: Num>(N);

impl<E: Num> StaticEdgeKind for (Directed, E) {
    type Kind = Directed;
    type Edge = StaticDirectedEdge<E>;
}

impl<N: Num> Bounded for StaticDirectedEdge<N> {
    fn max_value() -> Self {
        StaticDirectedEdge(N::max_value())
    }

    fn min_value() -> Self {
        StaticDirectedEdge(N::min_value())
    }
}

impl<N: Num> EdgeImpl for StaticDirectedEdge<N> {
    fn new(e: usize) -> Self {
        StaticDirectedEdge(N::from_usize(e))
    }

    fn new_checked(e: usize) -> Option<Self> {
        if N::is_valid(e) {
            Some(Self::new(e))
        } else {
            None
        }
    }

    fn source<T>(self, ends: &[T]) -> &T {
        &ends[2 * N::to_usize(self.0)]
    }

    fn target<T>(self, ends: &[T]) -> &T {
        &ends[2 * N::to_usize(self.0) + 1]
    }

    fn to_index(self) -> usize {
        N::to_usize(self.0)
    }

    fn reverse(self) -> Self {
        panic!("StaticDirectedEdge::reverse should not be called")
    }
}


// StaticUndirectedEdge

#[derive(Copy, Clone, Debug, Eq)]
pub struct StaticUndirectedEdge<N: Num>(N);

impl<E: Num> StaticEdgeKind for (Undirected, E) {
    type Kind = Undirected;
    type Edge = StaticUndirectedEdge<E>;
}

impl<N: Num> Bounded for StaticUndirectedEdge<N> {
    fn max_value() -> Self {
        StaticUndirectedEdge(N::max_value())
    }

    fn min_value() -> Self {
        StaticUndirectedEdge(N::min_value())
    }
}

// TODO: Document the representation of StaticUndirectedEdge
impl<N: Num> EdgeImpl for StaticUndirectedEdge<N> {
    fn new(e: usize) -> Self {
        StaticUndirectedEdge(N::from_usize(2 * e + 1))
    }

    fn new_checked(e: usize) -> Option<Self> {
        e.checked_mul(2)
            .and_then(|x| x.checked_add(1))
            .and_then(|x| if N::is_valid(x) {
                Some(Self::new(e))
            } else {
                None
            })
    }

    fn source<T>(self, ends: &[T]) -> &T {
        &ends[N::to_usize(self.0) ^ 1]
    }

    fn target<T>(self, ends: &[T]) -> &T {
        &ends[N::to_usize(self.0)]
    }

    fn to_index(self) -> usize {
        N::to_usize(self.0) / 2
    }

    fn reverse(self) -> Self {
        StaticUndirectedEdge(N::from_usize(N::to_usize(self.0) ^ 1))
    }
}

impl<N: Num> PartialEq for StaticUndirectedEdge<N> {
    fn eq(&self, other: &Self) -> bool {
        self.to_index() == other.to_index()
    }
}

impl<N: Num> PartialOrd for StaticUndirectedEdge<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.to_index().partial_cmp(&other.to_index())
    }
}

impl<N: Num> Ord for StaticUndirectedEdge<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_index().cmp(&other.to_index())
    }
}

impl<N: Num> Hash for StaticUndirectedEdge<N> {
    fn hash<H>(&self, state: &mut H)
        where H: Hasher
    {
        self.to_index().hash(state)
    }
}


// Vertex

pub type StaticVertex<N> = N;


// Graph


#[derive(Clone, Debug, PartialEq)]
pub struct Static<V: Num, K: StaticEdgeKind> {
    num_vertices: usize,
    ends: Vec<StaticVertex<V>>,
    edges: Vec<K::Edge>,
    edges_start: Vec<usize>,
}

impl<V: Num, K: StaticEdgeKind> Static<V, K> {
    fn inc(&self, v: Vertex<Self>) -> &[Edge<Self>] {
        let i = V::to_usize(v);
        &self.edges[self.edges_start[i]..self.edges_start[i + 1]]
    }
}

impl<V: Num, K: StaticEdgeKind> WithBuilder for Static<V, K> {
    type Builder = StaticBuilder<V, K>;
}

pub struct StaticBuilder<V: Num, K: StaticEdgeKind> {
    num_vertices: usize,
    ends: Vec<StaticVertex<V>>,
    edges: Vec<K::Edge>,
}

impl<V: Num, K: StaticEdgeKind> Builder for StaticBuilder<V, K> {
    type Graph = Static<V, K>;

    fn new(num_vertices: usize, num_edges: usize) -> Self {
        assert!(V::is_valid(num_vertices));
        StaticBuilder {
            num_vertices: num_vertices,
            ends: Vec::with_capacity(2 * num_edges),
            edges: vec![],
        }
    }

    fn add_edge(&mut self, u: usize, v: usize) {
        self.ends.push(V::from_usize(u));
        self.ends.push(V::from_usize(v));
        let e = K::Edge::new_checked((self.ends.len() - 2) / 2).expect("too many edges");
        self.edges.push(e);
        if K::Kind::is_undirected() {
            self.edges.push(e.reverse());
        }
    }

    fn finalize(mut self) -> Self::Graph {
        // TODO: improve test
        let ends = self.ends;
        self.edges.sort_by_key(|e| (e.source(&ends), e.target(&ends)));

        let mut starts = Vec::with_capacity(self.num_vertices.checked_add(1).unwrap());
        let mut last = V::from_usize(self.num_vertices);
        for (i, e) in self.edges.iter().enumerate() {
            let s = *e.source(&ends);
            if s != last {
                while starts.len() != V::to_usize(s) {
                    starts.push(i)
                }
                assert_eq!(starts.len(), V::to_usize(s));
                starts.push(i);
                last = s;
            }
        }
        while starts.len() <= self.num_vertices {
            starts.push(self.edges.len());
        }

        Static {
            num_vertices: self.num_vertices,
            ends: ends,
            edges: self.edges,
            edges_start: starts,
        }
    }

    fn finalize_(self) -> (Self::Graph, VecVertex<Self::Graph>, VecEdge<Self::Graph>) {
        let g = self.finalize();
        let v = vec(g.vertices());
        let e = vec(g.edges());
        (g, v, e)
    }
}


// Graph implementation

impl<V: Num, K: StaticEdgeKind> WithVertex for Static<V, K> {
    type Vertex = StaticVertex<V>;
    type OptionVertex = OptionalMax<StaticVertex<V>>;
}

impl<V: Num, K: StaticEdgeKind> WithEdge for Static<V, K> {
    type Kind = K::Kind;
    type Edge = K::Edge;
    type OptionEdge = OptionalMax<Self::Edge>;

    fn orientation(&self, _e: Edge<Self>) -> Orientation {
        K::Kind::orientation()
    }

    fn source(&self, e: Edge<Self>) -> Vertex<Self> {
        *e.source(&self.ends)
    }

    fn target(&self, e: Edge<Self>) -> Vertex<Self> {
        *e.target(&self.ends)
    }

    fn get_reverse(&self, e: Edge<Self>) -> Option<Edge<Self>> {
        if K::Kind::is_undirected() {
            Some(e.reverse())
        } else {
            let (u, v) = self.ends(e);
            self.out_edges(v).find(|e| self.target(*e) == u)
        }
    }
}

impl<'a, V: Num, K: StaticEdgeKind> VertexTypes<'a, Static<V, K>> for Static<V, K> {
    type VertexIter = V::Range;
    type OutNeighborIter = OutNeighborFromOutEdge<'a, Self, OutEdgeIter<'a, Self>>;
}

impl<'a, V: Num, K: StaticEdgeKind> EdgeTypes<'a, Static<V, K>> for Static<V, K> {
    type EdgeIter = Map<Range<usize>, fn(usize) -> Edge<Self>>;
    type OutEdgeIter = Cloned<Iter<'a, Edge<Self>>>;
}

impl<V: Num, K: StaticEdgeKind> VertexList for Static<V, K> {
    fn num_vertices(&self) -> usize {
        self.num_vertices
    }

    fn vertices(&self) -> VertexIter<Self> {
        V::range(0, self.num_vertices)
    }
}

impl<V: Num, K: StaticEdgeKind> EdgeList for Static<V, K> {
    fn num_edges(&self) -> usize {
        self.ends.len() / 2
    }

    fn edges(&self) -> EdgeIter<Self> {
        // TODO: specialization
        (0..self.num_edges()).map(K::Edge::new)
    }
}

impl<V: Num, K: StaticEdgeKind> Adjacency for Static<V, K> {
    fn out_neighbors(&self, v: Vertex<Self>) -> OutNeighborIter<Self> {
        OutNeighborFromOutEdge::new(self, self.out_edges(v))
    }

    fn out_degree(&self, v: Vertex<Self>) -> usize {
        self.inc(v).len()
    }
}

impl<V: Num, K: StaticEdgeKind> Incidence for Static<V, K> {
    fn out_edges(&self, v: Vertex<Self>) -> OutEdgeIter<Self> {
        self.inc(v).iter().cloned()
    }
}

impl<V: Num, K: StaticEdgeKind> VertexIndex for Static<V, K> {
    type VertexIndexProp = FnProp<fn(Vertex<Self>) -> usize>;

    fn vertex_index(&self) -> VertexIndexProp<Self> {
        // TODO: check if to_usize is being inlined
        FnProp(V::to_usize)
    }
}

impl<V: Num, K: StaticEdgeKind> EdgeIndex for Static<V, K> {
    type EdgeIndexProp = FnProp<fn(Edge<Self>) -> usize>;

    fn edge_index(&self) -> EdgeIndexProp<Self> {
        // TODO: check if to_index is being inlined
        FnProp(K::Edge::to_index)
    }
}

impl<T, V: Num, K: StaticEdgeKind> WithVertexProp<T> for Static<V, K> {
    type VertexProp = VecVertexProp<Self, T>;
}

impl<V: Num, K: StaticEdgeKind> BasicVertexProps for Static<V, K> {}

impl<T, V: Num, K: StaticEdgeKind> WithEdgeProp<T> for Static<V, K> {
    type EdgeProp = VecEdgeProp<Self, T>;
}

impl<V: Num, K: StaticEdgeKind> BasicEdgeProps for Static<V, K> {}

impl<V: Num, K: StaticEdgeKind> BasicProps for Static<V, K> {}

impl<V: Num, K: StaticEdgeKind> Choose for Static<V, K> {
    fn choose_vertex<R: Rng>(&self, mut rng: R) -> Option<Vertex<Self>> {
        if self.num_vertices() == 0 {
            None
        } else {
            Some(V::from_usize(rng.gen_range(0, self.num_vertices())))
        }
    }

    fn choose_out_neighbor<R: Rng>(&self, v: Vertex<Self>, rng: R) -> Option<Vertex<Self>> {
        self.choose_out_edge(v, rng).map(|e| self.target(e))
    }

    fn choose_edge<R: Rng>(&self, mut rng: R) -> Option<Edge<Self>> {
        if self.num_edges() == 0 {
            None
        } else if K::Kind::is_undirected() {
            let i = rng.gen_range(0, 2 * self.num_edges());
            if i % 2 == 0 {
                Some(K::Edge::new(i / 2))
            } else {
                Some(K::Edge::new(i / 2).reverse())
            }
        } else {
            Some(K::Edge::new(rng.gen_range(0, self.num_edges())))
        }
    }

    fn choose_out_edge<R: Rng>(&self, v: Vertex<Self>, mut rng: R) -> Option<Edge<Self>> {
        if self.out_degree(v) == 0 {
            None
        } else {
            self.inc(v).get(rng.gen_range(0, self.out_degree(v))).cloned()
        }
    }
}


// Num

pub trait Num: 'static + Eq + Copy + Clone + Debug + Hash + Bounded + Ord {
    type Range: Iterator<Item = Self>;
    fn range(a: usize, b: usize) -> Self::Range;
    fn to_usize(self) -> usize;
    fn from_usize(v: usize) -> Self;
    fn is_valid(v: usize) -> bool;
}

macro_rules! impl_num {
    ($t: ident) => (
        impl Num for $t {
            type Range = Range<$t>;

            #[inline]
            fn range(a: usize, b: usize) -> Self::Range {
                Self::from_usize(a) .. Self::from_usize(b)
            }

            #[inline]
            fn to_usize(self) -> usize {
                self as usize
            }

            #[inline]
            fn from_usize(v: usize) -> Self {
                v as Self
            }

            #[inline]
            fn is_valid(v: usize) -> bool {
                (v as u64) < (Self::max_value() as u64)
            }
        }
    )
}

impl_num!(u8);
impl_num!(u16);
impl_num!(u32);
impl_num!(u64);
impl_num!(usize);


// Tests

#[cfg(test)]
mod tests {
    pub use super::{EdgeImpl, StaticUndirectedEdge, StaticGraph, StaticDigraph};
    pub use prelude::*;
    use tests::GraphTests;

    macro_rules! test {
        ($m: ident, $g: ident) => (
            mod $m {
                pub use super::*;

                struct Test;

                impl GraphTests for Test {
                    type G = $g;

                    fn new() -> (Self::G, VecVertex<Self::G>, VecEdge<Self::G>) {
                        Self::new_with_builder()
                    }
                }

                graph_tests!{Test}

                mod with_builder {
                    use super::*;
                    use builder::BuilderTests;

                    struct Test;

                    impl BuilderTests for Test {
                        type G = StaticGraph;
                    }

                    graph_builder_tests!{Test}
                }
            }
        )
    }

    test!(directed, StaticDigraph);
    test!(undirected, StaticGraph);
}
