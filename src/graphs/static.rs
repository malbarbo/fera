use graph::*;
use fera::{IteratorExt, MapBind};
use fera::optional::OptionalMax;
use builder::{Builder, WithBuilder};
use choose::Choose;
use arrayprop::*;
use fnprop::*;

use std::iter::{Cloned, Map};
use std::ops::{Index, Range};
use std::slice::Iter;
use std::hash::{Hash, Hasher};
use std::cmp::Ordering;
use std::fmt::Debug;

use rand::Rng;
use num::Bounded;

// TODO: Rename to FastGraph
pub type StaticGraph = StaticGraphGeneric<u32, usize>;

pub trait Num: 'static + Eq + Copy + Clone + Debug + Hash + Bounded {
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

            #[inline(always)]
            fn range(a: usize, b: usize) -> Self::Range {
                Self::from_usize(a) .. Self::from_usize(b)
            }

            #[inline(always)]
            fn to_usize(self) -> usize {
                self as usize
            }

            #[inline(always)]
            fn from_usize(v: usize) -> Self {
                v as Self
            }

            #[inline(always)]
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


// StaticEdge

#[derive(Copy, Clone, Debug, Eq)]
pub struct StaticEdge<N: Num>(N);

impl<N: Num> Bounded for StaticEdge<N> {
    fn max_value() -> Self {
        StaticEdge(N::max_value())
    }

    fn min_value() -> Self {
        StaticEdge(N::min_value())
    }
}

// TODO: Document the representation of StaticEdge
impl<N: Num> StaticEdge<N> {
    #[inline(always)]
    fn new(e: usize) -> Self {
        StaticEdge(Num::from_usize(2 * e + 1))
    }

    #[inline(always)]
    fn new_reverse(e: usize) -> Self {
        StaticEdge(Num::from_usize(2 * e))
    }

    #[inline(always)]
    fn reverse(self) -> Self {
        StaticEdge(Num::from_usize(Num::to_usize(self.0) ^ 1))
    }

    #[inline(always)]
    fn to_index(self) -> usize {
        Num::to_usize(self.0) / 2
    }
}

impl<N: Num> PartialEq for StaticEdge<N> {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.to_index() == other.to_index()
    }
}

impl<N: Num> PartialOrd for StaticEdge<N> {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.to_index().partial_cmp(&other.to_index())
    }
}

impl<N: Num> Ord for StaticEdge<N> {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_index().cmp(&other.to_index())
    }
}

impl<N: Num> Hash for StaticEdge<N> {
    #[inline(always)]
    fn hash<H>(&self, state: &mut H)
        where H: Hasher
    {
        self.to_index().hash(state)
    }
}


// Vertex

pub type StaticVertex<N> = N;

// StaticGraphGeneric

#[derive(Clone, Debug, PartialEq)]
pub struct StaticGraphGeneric<V: Num, E: Num> {
    num_vertices: usize,
    ends: Vec<StaticVertex<V>>,
    inc: Vec<Vec<StaticEdge<E>>>,
}

impl<V: Num, E: Num> StaticGraphGeneric<V, E> {
    fn add_edge(&mut self, u: Vertex<Self>, v: Vertex<Self>) {
        self.ends.push(u);
        self.ends.push(v);
        let e = (self.ends.len() - 2) / 2;
        self.inc[Num::to_usize(u)].push(StaticEdge::new(e));
        self.inc[Num::to_usize(v)].push(StaticEdge::new_reverse(e));
    }

    fn inc(&self, v: Vertex<Self>) -> &Vec<StaticEdge<E>> {
        self.inc.index(Num::to_usize(v))
    }
}

impl<V: Num, E: Num> WithBuilder for StaticGraphGeneric<V, E> {
    type Builder = StaticGraphGenericBuilder<V, E>;
}

pub struct StaticGraphGenericBuilder<V: Num, E: Num> {
    g: StaticGraphGeneric<V, E>,
}

impl<V: Num, E: Num> Builder for StaticGraphGenericBuilder<V, E> {
    type Graph = StaticGraphGeneric<V, E>;

    fn new(num_vertices: usize, num_edges: usize) -> Self {
        // TODO: test this assert
        assert!(V::is_valid(num_vertices));
        StaticGraphGenericBuilder {
            g: StaticGraphGeneric {
                num_vertices: num_vertices,
                ends: Vec::with_capacity(2 * num_edges),
                inc: vec![vec![]; num_vertices],
            },
        }
    }

    fn add_edge(&mut self, u: usize, v: usize) {
        self.g.add_edge(Num::from_usize(u), Num::from_usize(v));
    }

    fn finalize(self) -> Self::Graph {
        // TODO: test this assert
        assert!(E::is_valid(self.g.ends.len()));
        self.g
    }

    fn finalize_(self) -> (Self::Graph, VecVertex<Self::Graph>, VecEdge<Self::Graph>) {
        // TODO: test this assert
        assert!(E::is_valid(self.g.ends.len()));
        let v = self.g.vertices().into_vec();
        let e = self.g.edges().into_vec();
        (self.g, v, e)
    }
}


// Graph implementation

impl<V: Num, E: Num> WithVertex for StaticGraphGeneric<V, E> {
    type Vertex = StaticVertex<V>;
    type OptionVertex = OptionalMax<StaticVertex<V>>;
}

impl<V: Num, E: Num> WithEdge for StaticGraphGeneric<V, E> {
    type Kind = Undirected;
    type Edge = StaticEdge<E>;
    type OptionEdge = OptionalMax<StaticEdge<E>>;

    #[inline(always)]
    fn source(&self, e: Edge<Self>) -> Vertex<Self> {
        self.ends[Num::to_usize(e.0) ^ 1]
    }

    #[inline(always)]
    fn orientation(&self, _e: Edge<Self>) -> Orientation {
        Orientation::Undirected
    }

    #[inline(always)]
    fn target(&self, e: Edge<Self>) -> Vertex<Self> {
        self.ends[Num::to_usize(e.0)]
    }

    #[inline(always)]
    fn reverse(&self, e: Edge<Self>) -> Edge<Self> {
        e.reverse()
    }

    #[inline(always)]
    fn get_reverse(&self, e: Edge<Self>) -> Option<Edge<Self>> {
        Some(self.reverse(e))
    }
}

impl<'a, V: Num, E: Num> VertexTypes<'a, StaticGraphGeneric<V, E>> for StaticGraphGeneric<V, E> {
    type VertexIter = V::Range;
    // TODO: Use IncidenceOutNeighborIter
    type OutNeighborIter = MapBind<OutEdgeIter<'a, Self>, &'a Self, fn(&Self, StaticEdge<E>) -> V>;
}

impl<'a, V: Num, E: Num> EdgeTypes<'a, StaticGraphGeneric<V, E>> for StaticGraphGeneric<V, E> {
    type EdgeIter = Map<Range<usize>, fn(usize) -> StaticEdge<E>>;
    type OutEdgeIter = Cloned<Iter<'a, StaticEdge<E>>>;
}

impl<V: Num, E: Num> VertexList for StaticGraphGeneric<V, E> {
    fn num_vertices(&self) -> usize {
        self.num_vertices
    }

    fn vertices(&self) -> VertexIter<Self> {
        V::range(0, self.num_vertices)
    }
}

impl<V: Num, E: Num> EdgeList for StaticGraphGeneric<V, E> {
    fn num_edges(&self) -> usize {
        self.ends.len() / 2
    }

    fn edges(&self) -> EdgeIter<Self> {
        // TODO: iterate over 1, 3, 5, ...
        (0..self.num_edges()).map(StaticEdge::new)
    }
}

impl<V: Num, E: Num> Adjacency for StaticGraphGeneric<V, E> {
    #[inline(always)]
    fn out_neighbors(&self, v: Vertex<Self>) -> OutNeighborIter<Self> {
        self.out_edges(v).map_bind(self, Self::target)
    }

    #[inline(always)]
    fn out_degree(&self, v: Vertex<Self>) -> usize {
        self.inc[Num::to_usize(v)].len()
    }
}

impl<V: Num, E: Num> Incidence for StaticGraphGeneric<V, E> {
    fn out_edges(&self, v: Vertex<Self>) -> OutEdgeIter<Self> {
        self.inc(v).iter().cloned()
    }
}

impl<V: Num, E: Num> VertexIndex for StaticGraphGeneric<V, E> {
    type VertexIndexProp = FnProp<fn(Vertex<StaticGraphGeneric<V, E>>) -> usize>;

    fn vertex_index(&self) -> VertexIndexProp<Self> {
        // TODO: check if to_usize is being inlined
        FnProp(V::to_usize)
    }
}

impl<V: Num, E: Num> EdgeIndex for StaticGraphGeneric<V, E> {
    type EdgeIndexProp = FnProp<fn(Edge<StaticGraphGeneric<V, E>>) -> usize>;

    fn edge_index(&self) -> EdgeIndexProp<Self> {
        // TODO: check if to_index is being inlined
        FnProp(StaticEdge::<E>::to_index)
    }
}

impl<T, V: Num, E: Num> WithVertexProp<T> for StaticGraphGeneric<V, E> {
    type VertexProp = VecVertexProp<Self, T>;
}

impl<V: Num, E: Num> BasicVertexProps for StaticGraphGeneric<V, E> {}

impl<T, V: Num, E: Num> WithEdgeProp<T> for StaticGraphGeneric<V, E> {
    type EdgeProp = VecEdgeProp<Self, T>;
}

impl<V: Num, E: Num> BasicEdgeProps for StaticGraphGeneric<V, E> {}

impl<V: Num, E: Num> BasicProps for StaticGraphGeneric<V, E> {}

impl<V: Num, E: Num> Choose for StaticGraphGeneric<V, E> {
    fn choose_vertex<R: Rng>(&self, rng: &mut R) -> Vertex<Self> {
        Num::from_usize(rng.gen_range(0, self.num_vertices()))
    }

    fn choose_edge<R: Rng>(&self, rng: &mut R) -> Edge<Self> {
        StaticEdge::new(rng.gen_range(0, self.num_edges()))
    }

    fn choose_inc_edge<R: Rng>(&self, rng: &mut R, v: Vertex<Self>) -> Edge<Self> {
        self.inc(v)[rng.gen_range(0, self.out_degree(v))]
    }
}


// Tests

#[cfg(test)]
mod tests {
    use super::{StaticGraph, StaticEdge};
    use graph::*;
    use builder::*;
    use tests::*;

    #[test]
    fn builder() {
        let mut builder = StaticGraph::builder(3, 1);

        builder.add_edge(0, 1);
        builder.add_edge(1, 2);

        let g = builder.finalize();
        assert_eq!(3, g.num_vertices);
        assert_eq!(vec![0, 1, 1, 2], g.ends);
        assert_eq!(vec![vec![StaticEdge::new(0)],
                        vec![StaticEdge::new_reverse(0), StaticEdge::new(1)],
                        vec![StaticEdge::new_reverse(1)]],
                   g.inc);
    }

    struct Test;

    impl GraphTests for Test {
        type G = StaticGraph;

        fn new() -> (Self::G, VecVertex<Self::G>, VecEdge<Self::G>) {
            Self::new_with_builder()
        }
    }

    graph_tests!{Test}

    mod with_builder {
        use super::super::StaticGraph;
        use builder::*;

        struct Test;

        impl BuilderTests for Test {
            type G = StaticGraph;
        }

        graph_builder_tests!{Test}
    }
}
