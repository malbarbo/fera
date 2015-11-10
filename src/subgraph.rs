use graph::*;
use choose::Choose;
use ds::IteratorExt;
use std::iter::Cloned;
use std::slice::Iter;
use rand::Rng;

#[derive(Clone)]
pub struct Subgraph<'a, G>
    where G: 'a + Graph,
          &'a G: Types<G>
{
    g: &'a G,
    vertices: VecVertex<G>,
    edges: VecEdge<G>,
    inc: PropVertex<G, VecEdge<G>>,
}

impl<'a: 'b, 'b, G> IterTypes<Subgraph<'a, G>> for &'b Subgraph<'a, G>
    where G: 'a + Graph,
          &'a G: Types<G>,
{
    type Vertex = Cloned<Iter<'b, Vertex<G>>>;
    type Edge = Cloned<Iter<'b, Edge<G>>>;
    type Inc = Cloned<Iter<'b, Edge<G>>>;
}

impl<'a, G> Basic for Subgraph<'a, G>
    where G: 'a + Graph,
          &'a G: Types<G>,
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

    // Inc

    fn degree(&self, v: Vertex<Self>) -> usize {
        self.inc[v].len()
    }

    fn inc_edges<'b>(&'b self, v: Vertex<Self>) -> IterInc<Self>
        where &'b (): Sized
    {
        self.inc[v].iter().cloned()
    }
}


impl<'a, T: Clone, G> WithProps<T> for Subgraph<'a, G>
    where G: 'a + Graph + WithProps<T>,
          &'a G: Types<G>,
{
    type Vertex = PropVertex<G, T>;
    type Edge = PropEdge<G, T>;

    fn vertex_prop(&self, value: T) -> PropVertex<Self, T> {
        self.g.vertex_prop(value)
    }

    fn edge_prop(&self, value: T) -> PropEdge<Self, T> {
        self.g.edge_prop(value)
    }
}


// Choose

impl<'a, G> Choose for Subgraph<'a, G>
    where G: 'a + Graph,
          &'a G: Types<G>,
{
    fn choose_vertex<R: Rng>(&self, rng: &mut R) -> Vertex<Self> {
        self.vertices[rng.gen_range(0, self.num_vertices())]
    }

    fn choose_edge<R: Rng>(&self, rng: &mut R) -> Edge<Self> {
        self.edges[rng.gen_range(0, self.num_edges())]
    }

    fn choose_inc_edge<R: Rng>(&self, rng: &mut R, v: Vertex<Self>) -> Edge<Self> {
        self.inc[v][rng.gen_range(0, self.degree(v))]
    }
}


pub trait WithSubgraph: Graph
    where for<'a> &'a Self: Types<Self>
{
    fn spanning_subgraph<I>(&self, edges_iter: I) -> Subgraph<Self>
        where I: Iterator<Item = Edge<Self>>
    {
        let vertices = self.vertices().into_vec();
        let mut edges = vec![];
        // TODO: why x is necessary?
        let x: VecEdge<Self> = Vec::with_capacity(3);
        let mut inc = self.vertex_prop(x);
        for e in edges_iter {
            let (u, v) = self.endvertices(e);
            edges.push(e);
            inc[u].push(e);
            inc[v].push(self.reverse(e));
        }

        Subgraph {
            g: self,
            vertices: vertices,
            edges: edges,
            inc: inc,
        }
    }

    fn edge_induced_subgraph<I>(&self, edges_iter: I) -> Subgraph<Self>
        where I: Iterator<Item = Edge<Self>>
    {
        let mut vin = self.vertex_prop(false);
        let mut vertices = vec![];
        let mut edges = vec![];
        let x: VecEdge<Self> = Vec::with_capacity(3);
        let mut inc = self.vertex_prop(x);
        for e in edges_iter {
            let (u, v) = self.endvertices(e);
            if !vin[u] {
                vin[u] = true;
                vertices.push(u);
            }
            if !vin[v] {
                vin[v] = true;
                vertices.push(v);
            }

            edges.push(e);
            inc[u].push(e);
            inc[v].push(self.reverse(e));
        }

        Subgraph {
            g: self,
            vertices: vertices,
            edges: edges,
            inc: inc,
        }
    }

    fn induced_subgraph<I>(&self, _vertex_iter: I) -> Subgraph<Self>
        where I: Iterator<Item = Vertex<Self>>
    {
        unimplemented!()
    }

}

impl<G> WithSubgraph for G
    where G: Graph,
          for<'a> &'a G: Types<G> { }

// TODO: write tests
