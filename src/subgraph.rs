use graph::*;
use choose::Choose;

use fera::{IteratorExt, MapBind1};

use std::iter::Cloned;
use std::slice::Iter;
use std::borrow::Borrow;

use rand::Rng;

// TODO: Allow a subgraph be reused

pub struct Subgraph<G, B>
    where G: Undirected + BasicProps,
          B: Borrow<G>
{
    g: B,
    vertices: VecVertex<G>,
    edges: VecEdge<G>,
    inc: DefaultPropMutVertex<G, VecEdge<G>>,
}

impl<G, B> Subgraph<G, B>
    where G: Undirected + BasicProps,
          B: Borrow<G>
{
    fn g(&self) -> &G {
        self.g.borrow()
    }
}

impl<G, B> WithVertex for Subgraph<G, B>
    where G: 'static + Undirected + BasicProps,
          B: Borrow<G>
{
    type Vertex = Vertex<G>;
    type OptionVertex = OptionVertex<G>;
}

impl<G, B> WithEdge for Subgraph<G, B>
    where G: 'static + Undirected + BasicProps,
          B: Borrow<G>
{
    type Edge = Edge<G>;
    type OptionEdge = OptionEdge<G>;
}

impl<G, B> WithPair<Edge<Subgraph<G, B>>> for Subgraph<G, B>
    where G: 'static + Undirected + BasicProps,
          B: Borrow<G>
{
    fn source(&self, e: Edge<Self>) -> Vertex<Self> {
        self.g().source(e)
    }

    fn target(&self, e: Edge<Self>) -> Vertex<Self> {
        self.g().target(e)
    }

    fn ends(&self, e: Edge<Self>) -> (Vertex<Self>, Vertex<Self>) {
        self.g().ends(e)
    }

    fn opposite(&self, u: Vertex<Self>, e: Edge<Self>) -> Vertex<Self> {
        self.g().opposite(u, e)
    }
}

impl<'a, G, B> VertexIterators<'a, Subgraph<G, B>> for Subgraph<G, B>
    where G: 'static + Undirected + BasicProps,
          B: Borrow<G>
{
    type Vertex = Cloned<Iter<'a, Vertex<G>>>;
    type Neighbor = MapBind1<'a, IterIncEdge<'a, Self>, G, Vertex<Self>>;
}

impl<G, B> VertexList for Subgraph<G, B>
    where G: 'static + Undirected + BasicProps,
          B: Borrow<G>
{
    fn num_vertices(&self) -> usize {
        self.vertices.len()
    }

    fn vertices(&self) -> IterVertex<Self> {
        self.vertices.iter().cloned()
    }
}

impl<'a, G, B> EdgeIterators<'a, Subgraph<G, B>> for Subgraph<G, B>
    where G: 'static + Undirected + BasicProps,
          B: Borrow<G>
{
    type Edge = Cloned<Iter<'a, Edge<G>>>;
    type IncEdge = Cloned<Iter<'a, Edge<G>>>;
}

impl<G, B> EdgeList for Subgraph<G, B>
    where G: 'static + Undirected + BasicProps,
          B: Borrow<G>
{
    fn num_edges(&self) -> usize {
        self.edges.len()
    }

    fn edges(&self) -> IterEdge<Self> {
        self.edges.iter().cloned()
    }

    fn reverse(&self, e: Edge<Self>) -> Edge<Self> {
        self.g().reverse(e)
    }
}
impl<G, B> Undirected for Subgraph<G, B>
    where G: 'static + Undirected + BasicProps,
          B: Borrow<G>
{
}

impl<G, B> Neighbors for Subgraph<G, B>
    where G: 'static + Undirected + BasicProps,
          B: Borrow<G>
{
    fn neighbors(&self, v: Vertex<Self>) -> IterNeighbor<Self> {
        self.inc_edges(v).map_bind1(self.g(), G::target)
    }

    fn degree(&self, v: Vertex<Self>) -> usize {
        self.inc[v].len()
    }
}

impl<G, B> IncEdges for Subgraph<G, B>
    where G: 'static + Undirected + BasicProps,
          B: Borrow<G>
{
    fn inc_edges(&self, v: Vertex<Self>) -> IterIncEdge<Self> {
        self.inc[v].iter().cloned()
    }
}

impl<T: Clone, G, B> WithProps<T> for Subgraph<G, B>
    where G: 'static + BasicProps + WithProps<T>,
          B: Borrow<G>
{
    type Vertex = DefaultPropMutVertex<G, T>;
    type Edge = DefaultPropMutEdge<G, T>;

    fn vertex_prop(&self, value: T) -> DefaultPropMutVertex<Self, T> {
        self.g().vertex_prop(value)
    }

    fn edge_prop(&self, value: T) -> DefaultPropMutEdge<Self, T> {
        self.g().edge_prop(value)
    }
}

impl<T: Clone, G, B> PropMutVertexNew<Subgraph<G, B>, T> for DefaultPropMutVertex<G, T>
    where G: 'static + BasicProps + WithProps<T>,
          B: Borrow<G>
{
    fn new_prop_vertex(g: &Subgraph<G, B>, value: T) -> Self {
        DefaultPropMutVertex::<G, T>::new_prop_vertex(g.g(), value)
    }
}

impl<T: Clone, G, B> PropMutEdgeNew<Subgraph<G, B>, T> for DefaultPropMutEdge<G, T>
    where G: 'static + BasicProps + WithProps<T>,
          B: Borrow<G>
{
    fn new_prop_edge(g: &Subgraph<G, B>, value: T) -> Self {
        DefaultPropMutEdge::<G, T>::new_prop_edge(g.g(), value)
    }
}


// Choose

impl<G, B> Choose for Subgraph<G, B>
    where G: 'static + Undirected + BasicProps,
          B: Borrow<G>
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


// Extensions Traits

pub trait WithSubgraph<G: Undirected + BasicProps, B: Borrow<G>> {
    fn spanning_subgraph(self, edges: VecEdge<G>) -> Subgraph<G, B>;

    fn edge_induced_subgraph(self, edges: VecEdge<G>) -> Subgraph<G, B>;

    fn induced_subgraph(self, vertices: VecVertex<G>) -> Subgraph<G, B> where G: IncEdges;
}


impl<G, B> WithSubgraph<G, B> for B
    where G: Undirected + BasicProps,
          B: Borrow<G>
{
    fn spanning_subgraph(self, edges: VecEdge<G>) -> Subgraph<G, B> {
        // TODO: do not copy vertices
        let vertices;
        let mut inc;
        {
            let g: &G = self.borrow();
            vertices = g.vertices().into_vec();
            inc = g.vertex_prop(Vec::<Edge<G>>::new());
            for &e in &edges {
                let (u, v) = g.ends(e);
                inc[u].push(e);
                inc[v].push(g.reverse(e));
            }
        }

        Subgraph {
            g: self,
            vertices: vertices,
            edges: edges,
            inc: inc,
        }
    }

    fn edge_induced_subgraph(self, edges: VecEdge<G>) -> Subgraph<G, B> {
        let mut vin;
        let mut vertices;
        let mut inc;
        {
            let g: &G = self.borrow();
            vin = g.vertex_prop(false);
            vertices = vec![];
            inc = g.vertex_prop(Vec::<Edge<G>>::new());
            for &e in &edges {
                let (u, v) = g.ends(e);
                if !vin[u] {
                    vin[u] = true;
                    vertices.push(u);
                }
                if !vin[v] {
                    vin[v] = true;
                    vertices.push(v);
                }

                inc[u].push(e);
                inc[v].push(g.reverse(e));
            }
        }

        Subgraph {
            g: self,
            vertices: vertices,
            edges: edges,
            inc: inc,
        }
    }

    fn induced_subgraph(self, vertices: VecVertex<G>) -> Subgraph<G, B>
        where G: IncEdges
    {
        let mut edges;
        let mut inc;
        {
            let g: &G = self.borrow();
            edges = vec![];
            inc = g.vertex_prop(Vec::<Edge<G>>::new());
            for &u in &vertices {
                for e in g.inc_edges(u) {
                    let v = g.target(e);
                    // FIXME: this running time is terrible, improve
                    if vertices.contains(&v) {
                        inc[u].push(e);
                        if !edges.contains(&e) {
                            edges.push(e);
                        }
                    }
                }
            }
        }

        Subgraph {
            g: self,
            vertices: vertices,
            edges: edges,
            inc: inc,
        }
    }
}


// TODO: write benchs and optimize

#[cfg(test)]
mod tests {
    use super::*;
    use graph::*;
    use static_::*;
    use fera::IteratorExt;

    fn new_graph
        ()
        -> (StaticGraph, Edge<StaticGraph>, Edge<StaticGraph>, Edge<StaticGraph>, Edge<StaticGraph>)
    {
        let g = graph!(StaticGraph, 5, (0, 1), (0, 2), (1, 2), (3, 4));
        let e = g.edges().into_vec();
        (g, e[0], e[1], e[2], e[3])
    }

    #[test]
    fn test_spanning_subgraph() {
        let (g, _, e02, e12, _) = new_graph();
        let s = g.spanning_subgraph(vec![e02, e12]);
        assert_eq!(vec![0, 1, 2, 3, 4], s.vertices().into_vec());
        assert_eq!(hash_set![e02, e12], s.edges().into_hash_set());
        assert_eq!(hash_set![e02], s.inc_edges(0).into_hash_set());
        assert_eq!(hash_set![e12], s.inc_edges(1).into_hash_set());
        assert_eq!(hash_set![e02, e12], s.inc_edges(2).into_hash_set());
        assert_eq!(hash_set![], s.inc_edges(3).into_hash_set());
        assert_eq!(hash_set![], s.inc_edges(4).into_hash_set());
    }

    #[test]
    fn test_edge_induced_subgraph() {
        let (g, e01, e02, _, _) = new_graph();
        let s = g.edge_induced_subgraph(vec![e01, e02]);
        assert_eq!(hash_set![0, 1, 2], s.vertices().into_hash_set());
        assert_eq!(hash_set![e01, e02], s.edges().into_hash_set());
        assert_eq!(hash_set![e01, e02], s.inc_edges(0).into_hash_set());
        assert_eq!(hash_set![e01], s.inc_edges(1).into_hash_set());
        assert_eq!(hash_set![e02], s.inc_edges(2).into_hash_set());
    }

    #[test]
    fn test_induced_subgraph() {
        let (g, e01, e02, e12, _) = new_graph();
        let s = g.induced_subgraph(vec![0, 1, 2]);
        assert_eq!(hash_set![0, 1, 2], s.vertices().into_hash_set());
        assert_eq!(hash_set![e01, e02, e12], s.edges().into_hash_set());
        assert_eq!(hash_set![e01, e02], s.inc_edges(0).into_hash_set());
        assert_eq!(hash_set![e01, e12], s.inc_edges(1).into_hash_set());
        assert_eq!(hash_set![e02, e12], s.inc_edges(2).into_hash_set());
    }
}
