// not working
// use super::*;
use super::{
    Basic,
    Degree,
    Inc,
    IncIter,
    IncIterType,
    EdgeProp,
    EdgePropType,
    VertexProp,
    VertexPropType,
    WithEdgeProp,
    WithVertexProp,
};

use std::iter::{Cloned, Map};
use std::ops::{Index, IndexMut, Range};
use std::slice::Iter;
use std::hash::{Hash, Hasher};

// Edge

#[derive(Copy, Clone, Debug)]
pub struct Edge(usize);

impl Edge {
    fn new(e: usize) -> Edge {
        Edge(2*e + 1)
    }

    fn new_reverse(e: usize) -> Edge {
        Edge(2*e)
    }

    pub fn to_index(self) -> usize {
        self.0 / 2
    }
}

impl PartialEq<Edge> for Edge {
    fn eq(&self, other: &Edge) -> bool {
        self.to_index() == other.to_index()
    }
}

impl Eq for Edge { }

impl Hash for Edge {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        self.to_index().hash(state)
    }
}

pub struct EdgePropVec<T>(Vec<T>);

impl<T> Index<Edge> for EdgePropVec<T> {
    type Output = T;
    fn index<'a>(&'a self, index: Edge) -> &'a Self::Output {
        self.0.index(index.to_index())
    }
}

impl<T> IndexMut<Edge> for EdgePropVec<T> {
    fn index_mut<'a>(&'a mut self, index: Edge) -> &'a mut Self::Output {
        self.0.index_mut(index.to_index())
    }
}


// Graph

pub struct StaticGraph {
    num_vertices: usize,
    endvertices: Vec<usize>,
    inc: Vec<Vec<Edge>>,
}

impl StaticGraph {
    pub fn new_with_edges(num_vertices: usize, edges: &[(usize, usize)]) -> StaticGraph {
        let mut builder = StaticGraph::builder(num_vertices, edges.len());
        for &(u, v) in edges {
            builder.add_edge(u, v)
        }
        builder.finalize()
    }

    pub fn new_empty() -> StaticGraph {
        StaticGraph::new_with_edges(0, &[])
    }

    pub fn builder(num_vertices: usize, num_edges_hint: usize) -> StaticGraphBuilder {
        StaticGraphBuilder {
            g: StaticGraph {
                num_vertices: num_vertices,
                endvertices: Vec::with_capacity(num_edges_hint),
                inc: vec![vec![]; num_vertices],
            }
        }
    }

    fn add_edge(&mut self, u: usize, v: usize) {
        self.endvertices.push(u);
        self.endvertices.push(v);
        let e = (self.endvertices.len() - 2) / 2;
        self.inc[u].push(Edge::new(e));
        self.inc[v].push(Edge::new_reverse(e));
    }
}

pub struct StaticGraphBuilder {
    g: StaticGraph,
}

impl StaticGraphBuilder {
    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.g.add_edge(u, v);
    }

    pub fn num_edges(&self) -> usize {
        self.g.num_edges()
    }

    pub fn finalize(self) -> StaticGraph {
        self.g
    }
}


impl Basic for StaticGraph {
    type Vertex = usize;
    type Edge = Edge;
    type VertexIter = Range<Self::Vertex>;
    type EdgeIter = Map<Range<usize>, fn(usize) -> Self::Edge>;

    fn num_vertices(&self) ->  usize {
        self.num_vertices
    }

    fn vertices(&self) -> Self::VertexIter {
        0..self.num_vertices
    }

    fn source(&self, e: Self::Edge) -> Self::Vertex {
        self.endvertices[e.0 ^ 1]
    }

    fn target(&self, e: Self::Edge) -> Self::Vertex {
        self.endvertices[e.0]
    }

    fn num_edges(&self) -> usize {
        self.endvertices.len() / 2
    }

    fn edges(&self) -> Self::EdgeIter {
        (0..self.num_edges()).map(Edge::new)
    }
}

impl Degree for StaticGraph {
    fn degree(&self, v: Self::Vertex) -> usize {
        self.inc[v].len()
    }
}

impl<'a> IncIterType<'a> for StaticGraph {
    type Type = Cloned<Iter<'a, Self::Edge>>;
}

impl Inc for StaticGraph {
    fn inc_edges(&self, v: Self::Vertex) -> IncIter<Self> {
        self.inc[v].iter().cloned()
    }
}

impl<'a, T> VertexPropType<'a, T> for StaticGraph {
    type Type = Vec<T>;
}

impl WithVertexProp for StaticGraph {
    fn vertex_prop<T: Clone>(&self, value: T) -> VertexProp<Self, T> {
        vec![value; self.num_vertices()]
    }
}

impl<'a, T> EdgePropType<'a, T> for StaticGraph {
    type Type = EdgePropVec<T>;
}

impl WithEdgeProp for StaticGraph {
    fn edge_prop<T: Clone>(&self, value: T) -> EdgeProp<Self, T> {
        EdgePropVec(vec![value; self.num_edges()])
    }
}


// Tests

#[cfg(test)]
mod tests {
    use super::Edge;
    use super::super::*;
    use super::super::tests::*;

    #[test]
    fn builder() {
        let mut builder = StaticGraph::builder(3, 1);
        assert_eq!(0, builder.num_edges());

        builder.add_edge(0, 1);
        assert_eq!(1, builder.num_edges());

        builder.add_edge(1, 2);
        assert_eq!(2, builder.num_edges());

        let g = builder.finalize();
        assert_eq!(3, g.num_vertices);
        assert_eq!(vec![0, 1, 1, 2], g.endvertices);
        assert_eq!(vec![vec![Edge::new(0)],
                        vec![Edge::new_reverse(0), Edge::new(1)],
                        vec![Edge::new_reverse(1)]],
                   g.inc);
    }

    struct StaticBuilder;

    impl Builder for StaticBuilder {
        type G = StaticGraph;

        fn new(num_vertices: usize, edges: &[(usize, usize)])
            -> (G<Self>, Vec<V<Self>>, Vec<E<Self>>) {
            let g = StaticGraph::new_with_edges(num_vertices, edges);
            let vertices = g.vertices().as_vec();
            let edges = g.edges().as_vec();
            (g, vertices, edges)
        }
    }

    test_basic!{ StaticBuilder }
    test_degree!{ StaticBuilder }
    test_inc!{ StaticBuilder }
    test_adj!{ StaticBuilder }
    test_vertex_prop!{ StaticBuilder }
    test_edge_prop!{ StaticBuilder }
}
