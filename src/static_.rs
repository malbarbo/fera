use graph::*;
use ds::{IteratorExt, VecExt};
use builder::{Builder, WithBuilder};
use choose::Choose;

use std::iter::{Cloned, Map};
use std::ops::{Deref, Index, IndexMut, Range};
use std::slice::Iter;
use std::hash::{Hash, Hasher};
use std::cmp::Ordering;
use std::fmt::Debug;

use rand::Rng;

pub type StaticGraph = StaticGraphGeneric<u32, usize>;

pub trait Num: Eq + Copy + Clone + Debug + Hash {
    fn to_usize(self) -> usize;
    fn from_usize(v: usize) -> Self;
    fn max() -> u64;
}

macro_rules! impl_num {
    ($t: ident) => (
        impl Num for $t {
            fn to_usize(self) -> usize {
                self as usize
            }

            fn from_usize(v: usize) -> Self {
                v as Self
            }

            fn max() -> u64 {
                use std;
                std::$t::MAX as u64
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

// TODO: Document the representation of StaticEdge
impl<N: Num> StaticEdge<N> {
    fn new(e: usize) -> Self {
        StaticEdge(Num::from_usize(2 * e + 1))
    }

    fn new_reverse(e: usize) -> Self {
        StaticEdge(Num::from_usize(2 * e))
    }

    fn to_index(self) -> usize {
        Num::to_usize(self.0) / 2
    }

    fn reverse(self) -> Self {
        StaticEdge(Num::from_usize(Num::to_usize(self.0) ^ 1))
    }
}

impl<N: Num> traits::Item for StaticEdge<N> {
    type Option = Option<Self>;

    fn none() -> Option<Self> {
        None
    }

    fn to_some(&self) -> Option<Self> {
        Some(*self)
    }
}

impl<N: Num> PartialEq for StaticEdge<N> {
    fn eq(&self, other: &Self) -> bool {
        self.to_index() == other.to_index()
    }
}

impl<N: Num> PartialOrd for StaticEdge<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.to_index().partial_cmp(&other.to_index())
    }
}

impl<N: Num> Ord for StaticEdge<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_index().cmp(&other.to_index())
    }
}

impl<N: Num> Hash for StaticEdge<N> {
    fn hash<H>(&self, state: &mut H)
        where H: Hasher
    {
        self.to_index().hash(state)
    }
}

#[derive(Clone, Debug)]
pub struct PropStaticEdge<T>(Vec<T>);

impl<T> Deref for PropStaticEdge<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Vec<T> {
        &self.0
    }
}

impl<T, N: Num> Index<StaticEdge<N>> for PropStaticEdge<T> {
    type Output = T;
    fn index(&self, index: StaticEdge<N>) -> &Self::Output {
        self.0.index(index.to_index())
    }
}

impl<T, N: Num> IndexMut<StaticEdge<N>> for PropStaticEdge<T> {
    fn index_mut(&mut self, index: StaticEdge<N>) -> &mut Self::Output {
        self.0.index_mut(index.to_index())
    }
}


// Vertex

pub type StaticVertex<N> = N;

#[derive(Clone, Debug)]
pub struct PropStaticVertex<T>(Vec<T>);

impl<T> Deref for PropStaticVertex<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Vec<T> {
        &self.0
    }
}

impl<T, N: Num> Index<StaticVertex<N>> for PropStaticVertex<T> {
    type Output = T;
    fn index(&self, index: StaticVertex<N>) -> &Self::Output {
        self.0.index(Num::to_usize(index))
    }
}

impl<T, N: Num> IndexMut<StaticVertex<N>> for PropStaticVertex<T> {
    fn index_mut(&mut self, index: StaticVertex<N>) -> &mut Self::Output {
        self.0.index_mut(Num::to_usize(index))
    }
}

impl<N: Num> traits::Item for StaticVertex<N> {
    type Option = Option<Self>;

    fn none() -> Option<Self> {
        None
    }

    fn to_some(&self) -> Option<Self> {
        Some(*self)
    }
}

// TODO: Define StaticVertex struct
// TODO: Allow the num type of StaticEdge and StaticVertex to be specified.
// TODO: Define a feature to disable property bounds check for vertex and edge property.

// StaticGraphGeneric

#[derive(Clone)]
pub struct StaticGraphGeneric<V: Num, E: Num> {
    num_vertices: usize,
    endvertices: Vec<StaticVertex<V>>,
    inc: Vec<Vec<StaticEdge<E>>>,
}

impl<V: Num, E: Num> StaticGraphGeneric<V, E> {
    pub fn new_with_edges(num_vertices: usize, edges: &[(usize, usize)]) -> Self {
        let mut builder = StaticGraphGeneric::builder(num_vertices, edges.len());
        for &(u, v) in edges {
            builder.add_edge(u, v)
        }
        builder.finalize()
    }

    pub fn new_empty() -> Self {
        StaticGraphGeneric::new_with_edges(0, &[])
    }

    fn add_edge(&mut self, u: Vertex<Self>, v: Vertex<Self>) {
        self.endvertices.push(u);
        self.endvertices.push(v);
        let e = (self.endvertices.len() - 2) / 2;
        self.inc[Num::to_usize(u)].push(StaticEdge::new(e));
        self.inc[Num::to_usize(v)].push(StaticEdge::new_reverse(e));
    }

    fn inc(&self, v: Vertex<Self>) -> &Vec<StaticEdge<E>> {
        self.inc.index(Num::to_usize(v))
    }
}

impl<V: Num, E: Num> WithBuilder for StaticGraphGeneric<V, E> {
    type Builder = StaticGraphGenericBuilder<V, E>;

    fn builder(num_vertices: usize, num_edges: usize) -> Self::Builder {
        // TODO: test this assert
        assert!((num_vertices as u64) < V::max());
        StaticGraphGenericBuilder {
            g: StaticGraphGeneric {
                num_vertices: num_vertices,
                endvertices: Vec::with_capacity(2 * num_edges),
                inc: vec![vec![]; num_vertices],
            },
        }
    }
}

pub struct StaticGraphGenericBuilder<V: Num, E: Num> {
    g: StaticGraphGeneric<V, E>,
}

impl<V: Num, E: Num> Builder for StaticGraphGenericBuilder<V, E> {
    type Graph = StaticGraphGeneric<V, E>;

    fn add_edge(&mut self, u: usize, v: usize) {
        self.g.add_edge(Num::from_usize(u), Num::from_usize(v));
    }

    fn finalize(self) -> Self::Graph {
        assert!((self.g.endvertices.len() as u64) < E::max());
        self.g
    }

    fn finalize_(self) -> (Self::Graph, VecVertex<Self::Graph>, VecEdge<Self::Graph>) {
        // TODO: test this assert
        assert!((self.g.endvertices.len() as u64) < E::max());
        let v = self.g.vertices().into_vec();
        let e = self.g.edges().into_vec();
        (self.g, v, e)
    }
}


impl<'a, V: 'a +Num, E: 'a + Num> IterTypes<StaticGraphGeneric<V, E>> for &'a StaticGraphGeneric<V, E> {
    type Vertex = Map<Range<usize>, fn(usize) -> StaticVertex<V>>;
    type Edge = Map<Range<usize>, fn(usize) -> StaticEdge<E>>;
    type Inc = Cloned<Iter<'a, StaticEdge<E>>>;
}

impl<V: Num, E: Num> Basic for StaticGraphGeneric<V, E> {
    type Vertex = StaticVertex<V>;
    type Edge = StaticEdge<E>;

    fn num_vertices(&self) -> usize {
        self.num_vertices
    }

    fn vertices<'a>(&'a self) -> IterVertex<Self>
        where &'a (): Sized
    {
        (0..self.num_vertices).map(Num::from_usize)
    }

    fn source(&self, e: Edge<Self>) -> Vertex<Self> {
        self.endvertices[Num::to_usize(e.0) ^ 1]
    }

    fn target(&self, e: Edge<Self>) -> Vertex<Self> {
        self.endvertices[Num::to_usize(e.0)]
    }

    fn num_edges(&self) -> usize {
        self.endvertices.len() / 2
    }

    fn edges<'a>(&'a self) -> IterEdge<Self>
        where &'a (): Sized
    {
        (0..self.num_edges()).map(StaticEdge::new)
    }

    fn reverse(&self, e: Edge<Self>) -> Edge<Self> {
        e.reverse()
    }

    // Inc

    fn degree(&self, v: Vertex<Self>) -> usize {
        self.inc[Num::to_usize(v)].len()
    }

    fn inc_edges<'a>(&'a self, v: Vertex<Self>) -> IterInc<Self>
        where &'a (): Sized
    {
        self.inc(v).iter().cloned()
    }
}

impl<T: 'static + Clone, V: Num, E: Num> WithProps<T> for StaticGraphGeneric<V, E> {
    type Vertex = PropStaticVertex<T>;
    type Edge = PropStaticEdge<T>;

    fn vertex_prop(&self, value: T) -> PropVertex<Self, T> {
        PropStaticVertex(Vec::with_value(value, self.num_vertices()))
    }

    fn edge_prop(&self, value: T) -> PropEdge<Self, T> {
        PropStaticEdge(Vec::with_value(value, self.num_edges()))
    }
}


impl<V: Num, E: Num> Choose for StaticGraphGeneric<V, E>  {
    fn choose_vertex<R: Rng>(&self, rng: &mut R) -> Vertex<Self> {
        Num::from_usize(rng.gen_range(0, self.num_vertices()))
    }

    fn choose_edge<R: Rng>(&self, rng: &mut R) -> Edge<Self> {
        StaticEdge::new(rng.gen_range(0, self.num_edges()))
    }

    fn choose_inc_edge<R: Rng>(&self, rng: &mut R, v: Vertex<Self>) -> Edge<Self> {
        self.inc(v)[rng.gen_range(0, self.degree(v))]
    }
}

// Tests

#[cfg(test)]
mod tests {
    use graph::*;
    use static_::*;
    use builder::*;
    use tests::*;

    #[test]
    fn builder() {
        let mut builder = StaticGraph::builder(3, 1);

        builder.add_edge(0, 1);
        builder.add_edge(1, 2);

        let g = builder.finalize();
        assert_eq!(3, g.num_vertices);
        assert_eq!(vec![0, 1, 1, 2], g.endvertices);
        assert_eq!(vec![vec![StaticEdge::new(0)],
                        vec![StaticEdge::new_reverse(0), StaticEdge::new(1)],
                        vec![StaticEdge::new_reverse(1)]],
                   g.inc);
    }


    impl StaticGraph {
        fn new(num_vertices: usize,
               edges: &[(usize, usize)])
               -> (Self, VecVertex<Self>, VecEdge<Self>) {
            let g = StaticGraph::new_with_edges(num_vertices, edges);
            let vertices = g.vertices().into_vec();
            let edges = g.edges().into_vec();
            (g, vertices, edges)
        }
    }

    test_basic!{ StaticGraph }
    test_degree!{ StaticGraph }
    test_inc!{ StaticGraph }
    test_adj!{ StaticGraph }
    test_vertex_prop!{ StaticGraph }
    test_edge_prop!{ StaticGraph }
}
