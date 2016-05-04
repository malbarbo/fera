use graph::*;
use choose::Choose;
use fnprop::*;
use vecprop::*;

use fera::{IteratorExt, MapBind1};
use fera::optional::{BuildNone, Optioned, OptionalMax};

use std::ops::Range;

use rand::Rng;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CompleteGraph(u32);

impl CompleteGraph {
    pub fn new(n: u32) -> Self {
        CompleteGraph(n)
    }
}


// Edge

#[derive(Clone, Copy, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct CompleteEdge {
    u: u32,
    v: u32,
}

impl CompleteEdge {
    fn new(u: u32, v: u32) -> CompleteEdge {
        debug_assert!(u != v);

        CompleteEdge { u: u, v: v }
    }
}

#[derive(Clone, Copy)]
pub struct CompleteEdgeNone;

impl BuildNone<CompleteEdge> for CompleteEdgeNone {
    fn none() -> CompleteEdge {
        CompleteEdge { u: ::std::u32::MAX, v: ::std::u32::MAX }
    }
}

impl PartialEq for CompleteEdge {
    fn eq(&self, other: &CompleteEdge) -> bool {
        (self.u == other.u && self.v == other.v) || (self.u == other.v && self.v == other.u)
    }
}

#[derive(Clone, Debug)]
pub struct CompleteEdgeIndex(u32);

impl PropGet<CompleteEdge> for CompleteEdgeIndex {
    type Output = usize;

    fn get(&self, e: CompleteEdge) -> usize {
        // TODO: explain
        let n = self.0 as usize;
        let (u, v) = (e.u as usize, e.v as usize);
        if u < v {
            u * (n - 1) - (u * u - u) / 2 + v - u - 1
        } else if u > v {
            v * (n - 1) - (v * v - v) / 2 + u - v - 1
        } else {
            panic!("u == v")
        }
    }
}


// Iterators

pub struct EdgesIter {
    n: u32,
    rem: usize,
    u: u32,
    v: u32,
}

impl Iterator for EdgesIter {
    type Item = CompleteEdge;

    fn next(&mut self) -> Option<Self::Item> {
        if self.rem == 0 {
            None
        } else {
            let e = CompleteEdge::new(self.u, self.v);
            if self.v + 1 >= self.n {
                self.u += 1;
                self.v = self.u + 1;
            } else {
                self.v += 1
            }
            self.rem -= 1;
            Some(e)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.rem, Some(self.rem))
    }
}

impl ExactSizeIterator for EdgesIter {
    fn len(&self) -> usize {
        self.rem
    }
}

pub struct IncCompleteEdgeIter {
    rem: usize,
    u: u32,
    v: u32,
}

impl Iterator for IncCompleteEdgeIter {
    type Item = CompleteEdge;

    fn next(&mut self) -> Option<Self::Item> {
        if self.rem == 0 {
            None
        } else {
            if self.u == self.v {
                self.v += 1;
            }
            let e = Some(CompleteEdge::new(self.u, self.v));
            self.v += 1;
            self.rem -= 1;
            e
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.rem, Some(self.rem))
    }
}

impl ExactSizeIterator for IncCompleteEdgeIter {
    fn len(&self) -> usize {
        self.rem
    }
}

// Graph implementation

impl WithVertex for CompleteGraph {
    type Vertex = u32;
    type OptionVertex = OptionalMax<u32>;
    type VertexIndexProp = FnProp<fn (u32) -> usize>;
}

impl WithEdge for CompleteGraph {
    type Edge = CompleteEdge;
    type OptionEdge = Optioned<CompleteEdge, CompleteEdgeNone>;
    type EdgeIndexProp = CompleteEdgeIndex;
}

impl WithPair<CompleteEdge> for CompleteGraph {
    fn source(&self, e: Edge<Self>) -> Vertex<Self> {
        e.u
    }

    fn target(&self, e: Edge<Self>) -> Vertex<Self> {
        e.v
    }
}

impl<'a> VertexTypes<'a, CompleteGraph> for CompleteGraph {
    type VertexIter = Range<u32>;
    // TODO: write another iterator
    type NeighborIter = MapBind1<'a, IncEdgeIter<'a, Self>, Self, Vertex<Self>>;
}

impl<'a> EdgeTypes<'a, CompleteGraph> for CompleteGraph {
    type EdgeIter = EdgesIter;
    type IncEdgeIter = IncCompleteEdgeIter;
}

impl VertexList for CompleteGraph {
    fn num_vertices(&self) -> usize {
        self.0 as usize
    }

    fn vertices(&self) -> VertexIter<Self> {
        0..self.0
    }
}

impl EdgeList for CompleteGraph {
    fn num_edges(&self) -> usize {
        let n = self.num_vertices();
        (n * n - n) / 2
    }

    fn edges(&self) -> EdgeIter<Self> {
        EdgesIter {
            n: self.num_vertices() as u32,
            rem: self.num_edges(),
            u: 0,
            v: 1,
        }
    }

    fn reverse(&self, e: Edge<Self>) -> Edge<Self> {
        CompleteEdge::new(e.v, e.u)
    }
}

impl Adjacency for CompleteGraph {
    fn neighbors(&self, v: Vertex<Self>) -> NeighborIter<Self> {
        self.inc_edges(v).map_bind1(self, Self::target)
    }

    fn degree(&self, _: Vertex<Self>) -> usize {
        self.num_vertices() - 1
    }
}

impl Incidence for CompleteGraph {
    fn inc_edges(&self, v: Vertex<Self>) -> IncEdgeIter<Self> {
        IncCompleteEdgeIter {
            rem: self.degree(v),
            u: v,
            v: 0,
        }
    }
}

impl VertexIndex for CompleteGraph {
    fn vertex_index(&self) -> VertexIndexProp<Self> {
        #[inline(always)]
        fn u32_to_usize(x: u32) -> usize {
            x as usize
        }
        FnProp(u32_to_usize)
    }
}

impl EdgeIndex for CompleteGraph {
    fn edge_index(&self) -> EdgeIndexProp<Self> {
        CompleteEdgeIndex(self.num_vertices() as u32)
    }
}

impl<T: Clone> WithVertexProp<T> for CompleteGraph {
    type VertexProp = VecVertexProp<CompleteGraph, T>;
}

impl<T: Clone> WithEdgeProp<T> for CompleteGraph {
    type EdgeProp = VecEdgeProp<CompleteGraph, T>;
}

impl BasicVertexProps for CompleteGraph {}
impl BasicEdgeProps for CompleteGraph {}
impl BasicProps for CompleteGraph {}

impl Choose for CompleteGraph {
    fn choose_vertex<R: Rng>(&self, rng: &mut R) -> Vertex<Self> {
        rng.gen_range(0, self.num_vertices() as u32)
    }

    fn choose_edge<R: Rng>(&self, rng: &mut R) -> Edge<Self> {
        let u = self.choose_vertex(rng);
        let v = self.choose_vertex_if(rng, &mut |v| v != u);
        CompleteEdge::new(u, v)
    }

    fn choose_inc_edge<R: Rng>(&self, rng: &mut R, u: Vertex<Self>) -> Edge<Self> {
        let v = self.choose_vertex_if(rng, &mut |v| v != u);
        CompleteEdge::new(u, v)
    }
}


// Tests

#[cfg(test)]
mod tests {
    pub use super::*;
    pub use graph::*;
    pub use tests::*;

    pub fn e(u: u32, v: u32) -> CompleteEdge {
        CompleteEdge::new(u, v)
    }

    #[test]
    fn test_edges() {
        for n in 1..100 {
            let g = CompleteGraph(n);
            let mut r = vec![];
            for u in 0..n {
                for v in (u + 1)..n {
                    r.push((u, v));
                }
            }
            let ind = g.edge_index();
            for (i, e) in g.edges().enumerate() {
                assert_eq!(i, ind.get(e));
                assert_eq!(i, ind.get(g.reverse(e)));
                assert_eq!(r[i].0, g.source(e));
                assert_eq!(r[i].1, g.target(e));
            }
        }
    }

    mod k0 {
        use super::*;

        struct Test;

        impl GraphTests for Test {
            type G = CompleteGraph;

            fn new() -> (Self::G, VecVertex<Self::G>, VecEdge<Self::G>) {
                (CompleteGraph::new(0), vec![], vec![])
            }
        }

        graph_basic_tests!{Test}
        graph_prop_tests!{Test}
        graph_adj_tests!{Test}
    }

    mod k1 {
        use super::*;

        struct Test;

        impl GraphTests for Test {
            type G = CompleteGraph;

            fn new() -> (Self::G, VecVertex<Self::G>, VecEdge<Self::G>) {
                (CompleteGraph::new(1), vec![0], vec![])
            }
        }

        graph_basic_tests!{Test}
        graph_prop_tests!{Test}
        graph_adj_tests!{Test}
    }

    mod k2 {
        use super::*;

        struct Test;

        impl GraphTests for Test {
            type G = CompleteGraph;

            fn new() -> (Self::G, VecVertex<Self::G>, VecEdge<Self::G>) {
                (CompleteGraph::new(2), vec![0, 1], vec![e(0, 1)])
            }
        }

        graph_basic_tests!{Test}
        graph_prop_tests!{Test}
        graph_adj_tests!{Test}
    }

    mod k5 {
        use super::*;

        struct Test;

        impl GraphTests for Test {
            type G = CompleteGraph;

            fn new() -> (Self::G, VecVertex<Self::G>, VecEdge<Self::G>) {
                (CompleteGraph::new(5),
                 vec![0, 1, 2, 3, 4],
                 vec![e(0, 1), e(0, 2), e(0, 3), e(0, 4), e(1, 2), e(1, 3), e(1, 4), e(2, 3),
                      e(2, 4), e(3, 4)])
            }
        }

        graph_basic_tests!{Test}
        graph_prop_tests!{Test}
        graph_adj_tests!{Test}
    }
}
