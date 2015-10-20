use graph::*;
use iter::IteratorExt;
use builder::{Builder, WithBuilder};
use choose::Choose;
use std::iter::{Cloned, Map};
use std::ops::{Index, IndexMut, Range};
use std::slice::Iter;
use std::hash::{Hash, Hasher};
use std::cmp::Ordering;
use rand::Rng;

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

    fn to_index(self) -> usize {
        self.0 / 2
    }

    fn reverse(self) -> StaticEdge {
        StaticEdge(self.0 ^ 1)
    }
}

impl PartialEq<StaticEdge> for StaticEdge {
    fn eq(&self, other: &StaticEdge) -> bool {
        self.to_index() == other.to_index()
    }
}

impl Eq for StaticEdge { }

impl PartialOrd for StaticEdge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.to_index().partial_cmp(&other.to_index())
    }
}

impl Ord for StaticEdge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_index().cmp(&other.to_index())
    }
}

impl Hash for StaticEdge {
    fn hash<H>(&self, state: &mut H)
        where H: Hasher
    {
        self.to_index().hash(state)
    }
}

#[derive(Clone, Debug)]
pub struct PropStaticEdge<T>(pub Vec<T>);

impl<T> Index<StaticEdge> for PropStaticEdge<T> {
    type Output = T;
    fn index(&self, index: StaticEdge) -> &Self::Output {
        self.0.index(index.to_index())
    }
}

impl<T> IndexMut<StaticEdge> for PropStaticEdge<T> {
    fn index_mut(&mut self, index: StaticEdge) -> &mut Self::Output {
        self.0.index_mut(index.to_index())
    }
}


// StaticGraph

#[derive(Clone)]
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

    fn add_edge(&mut self, u: usize, v: usize) {
        self.endvertices.push(u);
        self.endvertices.push(v);
        let e = (self.endvertices.len() - 2) / 2;
        self.inc[u].push(StaticEdge::new(e));
        self.inc[v].push(StaticEdge::new_reverse(e));
    }
}

impl WithBuilder for StaticGraph {
    type Builder = StaticGraphBuilder;

    fn builder(num_vertices: usize, num_edges: usize) -> StaticGraphBuilder {
        StaticGraphBuilder {
            g: StaticGraph {
                num_vertices: num_vertices,
                endvertices: Vec::with_capacity(2 * num_edges),
                inc: vec![vec![]; num_vertices],
            },
        }
    }
}

pub struct StaticGraphBuilder {
    g: StaticGraph,
}

impl Builder for StaticGraphBuilder {
    type Graph = StaticGraph;

    fn add_edge(&mut self, u: usize, v: usize) {
        self.g.add_edge(u, v);
    }

    fn finalize(self) -> Self::Graph {
        self.g
    }

    fn finalize_(self) -> (Self::Graph, VecVertex<Self::Graph>, VecEdge<Self::Graph>) {
        let v = self.g.vertices().into_vec();
        let e = self.g.edges().into_vec();
        (self.g, v, e)
    }
}


impl<'a> IterTypes<StaticGraph> for &'a StaticGraph {
    type Vertex = Range<usize>;
    type Edge = Map<Range<usize>, fn(usize) -> StaticEdge>;
    type Inc = Cloned<Iter<'a, StaticEdge>>;
}


impl Basic for StaticGraph {
    type Vertex = usize;
    type Edge = StaticEdge;

    fn num_vertices(&self) -> usize {
        self.num_vertices
    }

    fn vertices<'a>(&'a self) -> IterVertex<Self>
        where &'a (): Sized
    {
        0..self.num_vertices
    }

    fn source(&self, e: Edge<Self>) -> Vertex<Self> {
        self.endvertices[e.0 ^ 1]
    }

    fn target(&self, e: Edge<Self>) -> Vertex<Self> {
        self.endvertices[e.0]
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
        self.inc[v].len()
    }

    fn inc_edges<'a>(&'a self, v: Vertex<Self>) -> IterInc<Self>
        where &'a (): Sized
    {
        self.inc[v].iter().cloned()
    }
}

impl<T: Clone> WithProps<T> for StaticGraph {
    type Vertex = Vec<T>;
    type Edge = PropStaticEdge<T>;

    fn vertex_prop(&self, value: T) -> PropVertex<Self, T> {
        vec![value; self.num_vertices()]
    }

    fn edge_prop(&self, value: T) -> PropEdge<Self, T> {
        PropStaticEdge(vec![value; self.num_edges()])
    }
}


impl Choose for StaticGraph  {
    fn choose_vertex<R: Rng>(&self, rng: &mut R) -> Vertex<Self> {
        rng.gen_range(0, self.num_vertices())
    }

    fn choose_edge<R: Rng>(&self, rng: &mut R) -> Edge<Self> {
        StaticEdge::new(rng.gen_range(0, self.num_edges()))
    }

    fn choose_inc_edge<R: Rng>(&self, rng: &mut R, v: Vertex<Self>) -> Edge<Self> {
        self.inc[v][rng.gen_range(0, self.degree(v))]
    }
}

// Tests

#[cfg(test)]
mod tests {
    use graph::*;
    use static_::*;
    use builder::*;
    use iter::*;
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
