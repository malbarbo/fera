use graph::*;
use std::iter::Cloned;
use std::slice::Iter;
use rand::Rng;

pub struct Subgraph<'a, G>
    where G: 'a + Graph,
          &'a G: Types<G>,
          Vertex<G>: 'a,
          Edge<G>: 'a,
{
    g: &'a G,
    vertices: VecVertex<G>,
    edges: VecEdge<G>,
    inc: PropVertex<'a, G, VecEdge<G>>,
}

impl<'a: 'b, 'b, G> IterTypes<Subgraph<'a, G>> for &'b Subgraph<'a, G>
    where G: 'a + Graph,
          &'a G: Types<G>,
          Vertex<G>: 'a,
          Edge<G>: 'a,
{
    type Vertex = Cloned<Iter<'b, Vertex<G>>>;
    type Edge = Cloned<Iter<'b, Edge<G>>>;
    type Inc = Cloned<Iter<'b, Edge<G>>>;
}

impl<'a: 'b, 'b, T, G> PropTypes<T, Subgraph<'a, G>> for &'b Subgraph<'a, G>
    where G: 'a + Graph,
          &'a G: Types<G> + PropTypes<T, G>,
          Vertex<G>: 'a,
          Edge<G>: 'a,
{
    type Vertex = PropVertex<'a, G, T>;
    type Edge = PropEdge<'a, G, T>;
}


impl<'a, G> Basic for Subgraph<'a, G>
    where G: 'a + Graph,
          &'a G: Types<G>,
          Vertex<G>: 'a,
          Edge<G>: 'a,
{

    type Vertex = Vertex<G>;
    type Edge = Edge<G>;

    fn num_vertices(&self) -> usize {
        self.vertices.len()
    }

    fn vertices<'b>(&'b self) -> IterVertex<Self>
        where &'b (): Sized
    {
        self.vertices.iter().cloned()
    }

    fn choose_vertex<R: Rng>(&self, rng: &mut R) -> Vertex<Self> {
        self.vertices[rng.gen_range(0, self.num_vertices())]
    }

    fn source(&self, e: Edge<Self>) -> Vertex<Self> {
        self.g.source(e)
    }

    fn target(&self, e: Edge<Self>) -> Vertex<Self> {
        self.g.target(e)
    }

    fn num_edges(&self) -> usize {
        self.edges.len()
    }

    fn edges<'b>(&'b self) -> IterEdge<Self>
        where &'b (): Sized
    {
        self.edges.iter().cloned()
    }

    fn reverse(&self, e: Edge<Self>) -> Edge<Self> {
        self.g.reverse(e)
    }

    fn choose_edge<R: Rng>(&self, rng: &mut R) -> Edge<Self> {
        self.edges[rng.gen_range(0, self.num_edges())]
    }

    // Inc

    fn degree(&self, v: Vertex<Self>) -> usize {
        self.inc[v].len()
    }

    fn inc_edges<'b>(&'b self, v: Vertex<Self>) -> IterInc<Self>
        where &'b (): Sized
    {
        self.inc[v].iter().cloned()
    }

    fn choose_inc_edge<R: Rng>(&self, rng: &mut R, v: Vertex<Self>) -> Edge<Self> {
        self.inc[v][rng.gen_range(0, self.degree(v))]
    }
}

impl<'a, T: Clone, G> WithProps<T> for Subgraph<'a, G>
    where G: 'a + Graph + WithProps<T>,
          &'a G: PropTypes<T, G> + Types<G>,
          Vertex<G>: 'a,
          Edge<G>: 'a,
{
    fn vertex_prop<'c>(&'c self, value: T) -> PropVertex<'c, Self, T>
        where &'c (): Sized
    {
        self.g.vertex_prop(value)
    }

    fn edge_prop<'c>(&'c self, value: T) -> PropEdge<'c, Self, T>
        where &'c (): Sized
    {
        self.g.edge_prop(value)
    }
}
