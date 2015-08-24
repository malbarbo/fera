use graph::*;
use std::iter::{Cloned, Map};
use std::ops::{Index, IndexMut, Range};
use std::slice::Iter;
use std::hash::{Hash, Hasher};

// StaticEdge

#[derive(Copy, Clone, Debug)]
pub struct StaticEdge(usize);

impl StaticEdge {
    fn new(e: usize) -> StaticEdge {
        StaticEdge(2 * e + 1)
    }

    fn new_reverse(e: usize) -> StaticEdge {
        StaticEdge(2 * e)
    }

    pub fn to_index(self) -> usize {
        self.0 / 2
    }
}

impl PartialEq<StaticEdge> for StaticEdge {
    fn eq(&self, other: &StaticEdge) -> bool {
        self.to_index() == other.to_index()
    }
}

impl Eq for StaticEdge { }

impl Hash for StaticEdge {
    fn hash<H>(&self, state: &mut H)
        where H: Hasher
    {
        self.to_index().hash(state)
    }
}

pub struct StaticEdgePropVec<T>(pub Vec<T>);

impl<T> Index<StaticEdge> for StaticEdgePropVec<T> {
    type Output = T;
    fn index<'a>(&'a self, index: StaticEdge) -> &'a Self::Output {
        self.0.index(index.to_index())
    }
}

impl<T> IndexMut<StaticEdge> for StaticEdgePropVec<T> {
    fn index_mut<'a>(&'a mut self, index: StaticEdge) -> &'a mut Self::Output {
        self.0.index_mut(index.to_index())
    }
}


// Graph

pub struct StaticGraph {
    num_vertices: usize,
    endvertices: Vec<usize>,
    inc: Vec<Vec<StaticEdge>>,
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
            },
        }
    }

    fn add_edge(&mut self, u: usize, v: usize) {
        self.endvertices.push(u);
        self.endvertices.push(v);
        let e = (self.endvertices.len() - 2) / 2;
        self.inc[u].push(StaticEdge::new(e));
        self.inc[v].push(StaticEdge::new_reverse(e));
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


impl Types for StaticGraph {
    type Vertex = usize;
    type Edge = StaticEdge;
}

impl<'a> Basic<'a> for StaticGraph {
    type VertexIter = Range<Self::Vertex>;
    type EdgeIter = Map<Range<usize>, fn(usize) -> Self::Edge>;

    fn num_vertices(&self) -> usize {
        self.num_vertices
    }

    fn vertices(&'a self) -> Self::VertexIter {
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

    fn edges(&'a self) -> Self::EdgeIter {
        (0..self.num_edges()).map(StaticEdge::new)
    }
}

impl<'a> Degree<'a> for StaticGraph {
    fn degree(&self, v: Self::Vertex) -> usize {
        self.inc[v].len()
    }
}

impl<'a> Inc<'a> for StaticGraph {
    type Type = Cloned<Iter<'a, Self::Edge>>;
    fn inc_edges(&self, v: Self::Vertex) -> IncIter<Self> {
        self.inc[v].iter().cloned()
    }
}

impl<'a, T: Clone> VertexProperty<'a, T> for StaticGraph {
    type Type = Vec<T>;
    fn vertex_prop(&'a self, value: T) -> VertexProp<Self, T> {
        vec![value; self.num_vertices()]
    }
}

impl<'a> WithVertexProp<'a> for StaticGraph { }

impl<'a, T: Clone> EdgeProperty<'a, T> for StaticGraph {
    type Type = StaticEdgePropVec<T>;
    fn edge_prop(&'a self, value: T) -> EdgeProp<Self, T> {
        StaticEdgePropVec(vec![value; self.num_edges()])
    }
}

impl<'a> WithEdgeProp<'a> for StaticGraph { }


// Tests

#[cfg(test)]
mod tests {
    use graph::*;
    use static_::*;
    use iter::*;
    use tests::*;

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
        assert_eq!(vec![vec![StaticEdge::new(0)],
                        vec![StaticEdge::new_reverse(0), StaticEdge::new(1)],
                        vec![StaticEdge::new_reverse(1)]],
                   g.inc);
    }


    impl StaticGraph {
        fn new(num_vertices: usize,
               edges: &[(usize, usize)])
               -> (Self, Vec<<StaticGraph as Types>::Vertex>, Vec<<StaticGraph as Types>::Edge>) {
            let g = StaticGraph::new_with_edges(num_vertices, edges);
            let vertices = g.vertices().as_vec();
            let edges = g.edges().as_vec();
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
