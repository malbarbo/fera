use graph::*;
use std::iter::Cloned;
use std::slice::Iter;
use rand::Rng;

pub struct Subgraph<'a, G>
    where G: 'a + Basic<'a> + WithVertexProp<'a>,
{
    g: &'a G,
    vertices: VecVertex<G>,
    edges: VecEdge<G>,
    inc: VertexProp<'a, G, VecEdge<G>>,
}

impl<'a, G> Types for Subgraph<'a, G>
    where G: 'a + Basic<'a> + WithVertexProp<'a>,
{
    type Vertex = G::Vertex;
    type Edge = G::Edge;
}

impl<'a, G> Basic<'a> for Subgraph<'a, G>
    where G: 'a + Basic<'a> + WithVertexProp<'a>,
          G::Vertex: 'a,
          G::Edge: 'a,
{
    type VertexIter = Cloned<Iter<'a, Self::Vertex>>;
    type EdgeIter = Cloned<Iter<'a, Self::Edge>>;

    fn num_vertices(&self) -> usize {
        self.vertices.len()
    }

    fn vertices(&'a self) -> Self::VertexIter {
        self.vertices.iter().cloned()
    }

    fn choose_vertex<R: Rng>(&self, rng: &mut R) -> Self::Vertex {
        self.vertices[rng.gen_range(0, self.num_vertices())]
    }

    fn source(&self, e: Self::Edge) -> Self::Vertex {
        self.g.source(e)
    }

    fn target(&self, e: Self::Edge) -> Self::Vertex {
        self.g.target(e)
    }

    fn num_edges(&self) -> usize {
        self.edges.len()
    }

    fn edges(&'a self) -> Self::EdgeIter {
        self.edges.iter().cloned()
    }

    fn choose_edge<R: Rng>(&self, rng: &mut R) -> Self::Edge {
        self.edges[rng.gen_range(0, self.num_edges())]
    }

    fn reverse(&self, e: Self::Edge) -> Self::Edge {
        self.g.reverse(e)
    }
}

impl<'a, G> Degree<'a> for Subgraph<'a, G>
    where G: 'a + Basic<'a> + WithVertexProp<'a>,
          G::Vertex: 'a,
          G::Edge: 'a,
{
    fn degree(&self, v: Self::Vertex) -> usize {
        self.inc[v].len()
    }
}

impl<'a, G> Inc<'a> for Subgraph<'a, G>
    where G: 'a + Inc<'a> + WithVertexProp<'a>,
          G::Vertex: 'a,
          G::Edge: 'a,
{
    type Type = Cloned<Iter<'a, Self::Edge>>;

    fn inc_edges(&'a self, v: Self::Vertex) -> IncIter<Self> {
        self.inc[v].iter().cloned()
    }

    fn choose_inc_edge<R: Rng>(&self, rng: &mut R, v: Self::Vertex) -> Self::Edge {
        self.inc[v][rng.gen_range(0, self.inc[v].len())]
    }
}

impl<'a, G, T: Clone> VertexProperty<'a, T> for Subgraph<'a, G>
    where G: 'a + Inc<'a> + WithVertexProp<'a> + VertexProperty<'a, T>,
          G::Vertex: 'a,
          G::Edge: 'a,
{
    type Type = VertexProp<'a, G, T>;

    fn vertex_prop(&'a self, value: T) -> VertexProp<Self, T> {
        self.g.vertex_prop(value)
    }
}

impl<'a, G> WithVertexProp<'a> for Subgraph<'a, G>
    where G: 'a + Inc<'a> + WithVertexProp<'a>,
          G::Vertex: 'a,
          G::Edge: 'a,
{ }

impl<'a, G, T: Clone> EdgeProperty<'a, T> for Subgraph<'a, G>
    where G: 'a + Inc<'a> + WithVertexProp<'a> + EdgeProperty<'a, T>,
          G::Vertex: 'a,
          G::Edge: 'a,
{
    type Type = EdgeProp<'a, G, T>;

    fn edge_prop(&'a self, value: T) -> EdgeProp<Self, T> {
        self.g.edge_prop(value)
    }
}

impl<'a, G> WithEdgeProp<'a> for Subgraph<'a, G>
    where G: 'a + Inc<'a> + WithVertexProp<'a> + WithEdgeProp<'a>,
          G::Vertex: 'a,
          G::Edge: 'a,
{ }
