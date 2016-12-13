use prelude::*;
use choose::Choose;
use props::{VecEdgeProp, VecVertexProp};

use fera::optional::{BuildNone, Optioned, OptionalMax};

use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::iter::{Chain, Map};
use std::marker::PhantomData;
use std::ops::Range;

use rand::Rng;

pub type CompleteGraph = Complete<Undirected>;

pub type CompleteDiGraph = Complete<Directed>;

pub type CVertex = u32;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Complete<K: CompleteEdgeKind> {
    n: CVertex,
    _marker: PhantomData<K>,
}

impl<K: CompleteEdgeKind> Complete<K> {
    pub fn new(n: CVertex) -> Self {
        Complete {
            n: n,
            _marker: PhantomData,
        }
    }
}

pub trait CompleteEdgeKind: UniformEdgeKind {
    type Edge: 'static + GraphItem + EdgeImpl;
}

pub trait EdgeImpl {
    fn from_index(index: usize) -> Self;
    fn to_index(self) -> usize;
    fn new(n: CVertex, u: CVertex, v: CVertex) -> Self;
    fn ends(self, n: CVertex) -> (CVertex, CVertex);
    fn reverse(self, n: CVertex) -> Self;
}

impl<'a, K: CompleteEdgeKind> VertexTypes<'a, Complete<K>> for Complete<K> {
    type VertexIter = Range<Vertex<Self>>;
    type OutNeighborIter = Chain<Range<CVertex>, Range<CVertex>>;
}

impl<K: CompleteEdgeKind> WithVertex for Complete<K> {
    type Vertex = CVertex;
    type OptionVertex = OptionalMax<CVertex>;
}

impl<'a, K: CompleteEdgeKind> EdgeTypes<'a, Complete<K>> for Complete<K> {
    type EdgeIter = Map<Range<usize>, fn(usize) -> K::Edge>;
    type OutEdgeIter = COutEdgeIter<Edge<Self>>;
}

impl<K: CompleteEdgeKind> WithEdge for Complete<K> {
    type Kind = K;
    type Edge = K::Edge;
    type OptionEdge = Optioned<K::Edge, MaxNone<K::Edge>>;

    fn source(&self, e: Edge<Self>) -> Vertex<Self> {
        K::Edge::ends(e, self.n).0
    }

    fn target(&self, e: Edge<Self>) -> Vertex<Self> {
        K::Edge::ends(e, self.n).1
    }

    fn orientation(&self, _e: Edge<Self>) -> Orientation {
        K::orientation()
    }

    fn reverse(&self, e: Edge<Self>) -> Edge<Self> {
        K::Edge::reverse(e, self.n)
    }

    fn get_reverse(&self, e: Edge<Self>) -> Option<Edge<Self>> {
        Some(K::Edge::reverse(e, self.n))
    }
}

impl<K: CompleteEdgeKind> VertexList for Complete<K> {
    fn num_vertices(&self) -> usize {
        self.n as usize
    }

    fn vertices(&self) -> VertexIter<Self> {
        0..self.n
    }
}

impl<K: CompleteEdgeKind> EdgeList for Complete<K> {
    fn num_edges(&self) -> usize {
        let n = self.num_vertices();
        if K::is_directed() {
            n * n - n
        } else {
            (n * n - n) / 2
        }
    }

    fn edges(&self) -> EdgeIter<Self> {
        (0..self.num_edges()).map(K::Edge::from_index)
    }
}

impl<K: CompleteEdgeKind> Adjacency for Complete<K> {
    fn out_neighbors(&self, v: CVertex) -> OutNeighborIter<Self> {
        debug_assert!(v <= self.n);
        (0..v).chain((v + 1)..self.n)
    }

    fn out_degree(&self, v: Vertex<Self>) -> usize {
        debug_assert!(v <= self.n);
        self.num_vertices().checked_sub(1).unwrap_or(0)
    }
}

impl<K: CompleteEdgeKind> Incidence for Complete<K> {
    fn out_edges(&self, v: Vertex<Self>) -> OutEdgeIter<Self> {
        debug_assert!(v <= self.n);
        COutEdgeIter::new(self.n, v)
    }
}

impl<K: CompleteEdgeKind> EdgeByEnds for Complete<K> {
    fn edge_by_ends(&self, u: Vertex<Self>, v: Vertex<Self>) -> Option<Edge<Self>> {
        if u < self.n && v < self.n && u != v {
            Some(K::Edge::new(self.n, u, v))
        } else {
            None
        }
    }
}

impl<T, K: CompleteEdgeKind> WithVertexProp<T> for Complete<K> {
    type VertexProp = VecVertexProp<Complete<K>, T>;
}

impl<T, K: CompleteEdgeKind> WithEdgeProp<T> for Complete<K>
    where Complete<K>: EdgeIndex
{
    type EdgeProp = VecEdgeProp<Complete<K>, T>;
}

impl<K: CompleteEdgeKind> BasicVertexProps for Complete<K> {}
impl<K: CompleteEdgeKind> BasicEdgeProps for Complete<K> {}
impl<K: CompleteEdgeKind> BasicProps for Complete<K> {}

impl<K: CompleteEdgeKind> Choose for Complete<K> {
    fn choose_vertex<R: Rng>(&self, rng: &mut R) -> Vertex<Self> {
        rng.gen_range(0, self.n)
    }

    fn choose_edge<R: Rng>(&self, rng: &mut R) -> Edge<Self> {
        K::Edge::from_index(rng.gen_range(0, self.num_edges()))
    }

    fn choose_inc_edge<R: Rng>(&self, rng: &mut R, u: Vertex<Self>) -> Edge<Self> {
        let v = self.choose_vertex_if(rng, |v| v != u);
        K::Edge::new(self.n, u, v)
    }
}


#[derive(Clone, Debug)]
pub struct CVertexIndexProp;

impl PropGet<CVertex> for CVertexIndexProp {
    type Output = usize;

    fn get(&self, x: CVertex) -> usize {
        x as usize
    }
}

impl<K: CompleteEdgeKind> VertexIndex for Complete<K> {
    type VertexIndexProp = CVertexIndexProp;

    fn vertex_index(&self) -> CVertexIndexProp {
        CVertexIndexProp
    }
}


#[derive(Clone, Debug)]
pub struct CEdgeIndexProp<E>(PhantomData<E>);

impl<E: EdgeImpl> PropGet<E> for CEdgeIndexProp<E> {
    type Output = usize;

    fn get(&self, e: E) -> usize {
        E::to_index(e)
    }
}

impl<K: CompleteEdgeKind> EdgeIndex for Complete<K> {
    type EdgeIndexProp = CEdgeIndexProp<K::Edge>;

    fn edge_index(&self) -> CEdgeIndexProp<K::Edge> {
        CEdgeIndexProp(PhantomData)
    }
}


#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct MaxNone<E>(PhantomData<E>);

impl<E: EdgeImpl> BuildNone<E> for MaxNone<E> {
    fn none() -> E {
        E::from_index(usize::max_value())
    }

    fn desc() -> &'static str {
        "CompleteEdge"
    }
}


// Iterators

pub struct COutEdgeIter<E> {
    n: CVertex,
    rem: usize,
    u: CVertex,
    v: CVertex,
    _marker: PhantomData<E>,
}

impl<E> COutEdgeIter<E> {
    fn new(n: CVertex, u: CVertex) -> Self {
        COutEdgeIter {
            n: n,
            rem: (n - 1) as usize,
            u: u,
            v: 0,
            _marker: PhantomData,
        }
    }
}

impl<E: EdgeImpl> Iterator for COutEdgeIter<E> {
    type Item = E;

    fn next(&mut self) -> Option<Self::Item> {
        if self.rem == 0 {
            None
        } else {
            if self.u == self.v {
                self.v += 1;
            }
            let e = Some(E::new(self.n, self.u, self.v));
            self.v += 1;
            self.rem -= 1;
            e
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.rem, Some(self.rem))
    }
}

impl<E: EdgeImpl> ExactSizeIterator for COutEdgeIter<E> {
    fn len(&self) -> usize {
        self.rem
    }
}


// Undirected

impl CompleteEdgeKind for Undirected {
    type Edge = UndirectedEdge;
}

#[derive(Clone, Copy, Eq, Debug)]
pub struct UndirectedEdge(usize);

impl PartialEq for UndirectedEdge {
    fn eq(&self, other: &UndirectedEdge) -> bool {
        self.to_index() == other.to_index()
    }
}

impl PartialOrd for UndirectedEdge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for UndirectedEdge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_index().cmp(&other.to_index())
    }
}

impl Hash for UndirectedEdge {
    fn hash<H>(&self, state: &mut H)
        where H: Hasher
    {
        self.to_index().hash(state)
    }
}

impl EdgeImpl for UndirectedEdge {
    fn from_index(index: usize) -> Self {
        UndirectedEdge(index << 1)
    }

    fn to_index(self) -> usize {
        self.0 >> 1
    }

    fn new(n: CVertex, u: CVertex, v: CVertex) -> Self {
        let (n, u, v) = (n as usize, u as usize, v as usize);
        let id = |u, v| {
            if u < (n - 1) / 2 {
                u * n + v
            } else {
                (n - u - 1) * n - v - 1
            }
        };

        if u < v {
            UndirectedEdge(id(u, v) << 1)
        } else {
            UndirectedEdge(id(v, u) << 1 | 1)
        }
    }

    fn ends(self, n: CVertex) -> (CVertex, CVertex) {
        let (u, v) = {
            let e = (self.0 >> 1) as CVertex;
            let (u, v) = (e / n, e % n);
            if u < v {
                (u, v)
            } else {
                (n - u - 2, n - v - 1)
            }
        };

        if self.0 & 1 == 0 { (u, v) } else { (v, u) }
    }

    fn reverse(self, _n: CVertex) -> Self {
        UndirectedEdge(self.0 ^ 1)
    }
}


// Directed

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct DirectedEdge(usize);

impl CompleteEdgeKind for Directed {
    type Edge = DirectedEdge;
}

impl EdgeImpl for DirectedEdge {
    fn from_index(index: usize) -> Self {
        DirectedEdge(index)
    }

    fn to_index(self) -> usize {
        self.0
    }

    fn new(n: CVertex, u: CVertex, v: CVertex) -> Self {
        let (n, u, v) = (n as usize, u as usize, v as usize);
        DirectedEdge(n * u + v - u - (if v < u { 0 } else { 1 }))
    }

    fn ends(self, n: CVertex) -> (CVertex, CVertex) {
        let e = self.0 as CVertex;
        let u = e / (n - 1);
        let mut v = e % (n - 1);
        if v >= u {
            v += 1;
        }
        (u, v)
    }

    fn reverse(self, n: CVertex) -> Self {
        let (u, v) = Self::ends(self, n);
        Self::new(n, v, u)
    }
}


// Tests

#[cfg(test)]
mod tests {
    pub use super::{CVertex, EdgeImpl, UndirectedEdge, DirectedEdge};
    pub use prelude::*;
    pub use tests::GraphTests;
    pub use itertools::Itertools;
    pub use std::fmt::Debug;

    fn assert_edge<E: EdgeImpl + Debug + Copy>(n: CVertex, u: CVertex, v: CVertex) {
        let e = E::new(n, u, v);
        let r = E::reverse(e, n);
        assert_eq!((u, v), E::ends(e, n), "n = {}, e = {:?}", n, e);
        assert_eq!((v, u), E::ends(r, n), "n = {}, e = {:?}, r = {:?}", n, e, r);
    }

    #[test]
    fn edge_impl() {
        for n in 2..10 {
            for (u, v) in (0..n).tuple_combinations() {
                assert_edge::<UndirectedEdge>(n, u, v);
                assert_edge::<UndirectedEdge>(n, v, u);

                assert_edge::<DirectedEdge>(n, u, v);
                assert_edge::<DirectedEdge>(n, v, u);
            }
        }
    }

    macro_rules! t {
        ($m:ident, $n:expr, $G:ident, $v:expr, $e:expr) => (
            mod $m {
                use super::*;

                struct Test;

                impl GraphTests for Test {
                    type G = $G;

                    fn new() -> (Self::G, VecVertex<Self::G>, VecEdge<Self::G>) {
                        let e = $e.into_iter()
                                  .map(|(u, v)| EdgeImpl::new($n, u, v))
                                  .sorted();
                        ($G::new($n), $v, e)
                    }
                }

                graph_tests!{Test}
            }
        )
    }

    // Undirected

    t!{k0, 0, CompleteGraph, vec![], vec![]}
    t!{k1, 1, CompleteGraph, vec![0], vec![]}
    t!{k2, 2, CompleteGraph, vec![0, 1], vec![(0, 1)]}

    t!{k3, 3,
        CompleteGraph,
        vec![0, 1, 2],
        vec![(0, 1), (0, 2), (1, 2)]}

    t!{k4, 4,
        CompleteGraph,
        vec![0, 1, 2, 3],
        vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]}


    // Directed

    t!{directed_k0, 0, CompleteDiGraph, vec![], vec![]}
    t!{directed_k1, 1, CompleteDiGraph, vec![0], vec![]}
    t!{directed_k2, 2, CompleteDiGraph, vec![0, 1], vec![(0, 1), (1, 0)]}

    t!{directed_k3, 3,
        CompleteDiGraph,
        vec![0, 1, 2],
        vec![(0, 1), (0, 2),
             (1, 0), (1, 2),
             (2, 0), (2, 1)]}

    t!{directed_k4, 4,
        CompleteDiGraph,
        vec![0, 1, 2, 3],
        vec![(0, 1), (0, 2), (0, 3),
             (1, 0), (1, 2), (1, 3),
             (2, 0), (2, 1), (2, 3),
             (3, 0), (3, 1), (3, 2)]}
}
