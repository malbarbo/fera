use graph::*;
use choose::Choose;

use fera::{IteratorExt, MapBind1};

use std::iter::Cloned;
use std::slice::Iter;
use std::borrow::Borrow;

use rand::Rng;

// TODO: Allow a subgraph be reused
// TODO: delegate all (possible) methods to g

pub struct Subgraph<'a, G>
    where G: 'a + Graph
{
    g: &'a G,
    vertices: VecVertex<G>,
    edges: VecEdge<G>,
    inc: DefaultVertexPropMut<G, VecEdge<G>>,
}

impl<'a, G> WithVertex for Subgraph<'a, G>
    where G: 'a + Graph
{
    type Vertex = Vertex<G>;
    type OptionVertex = OptionVertex<G>;
    // FIXME: this is not rigth
    type VertexIndexProp = VertexIndexProp<G>;
}

impl<'a, G> WithEdge for Subgraph<'a, G>
    where G: 'a + Graph
{
    type Edge = Edge<G>;
    type OptionEdge = OptionEdge<G>;
    // FIXME: this is not rigth
    type EdgeIndexProp = EdgeIndexProp<G>;
}

impl<'a, G> WithPair<Edge<Subgraph<'a, G>>> for Subgraph<'a, G>
    where G: 'a + Graph
{
    fn source(&self, e: Edge<Self>) -> Vertex<Self> {
        self.g.source(e)
    }

    fn target(&self, e: Edge<Self>) -> Vertex<Self> {
        self.g.target(e)
    }

    fn ends(&self, e: Edge<Self>) -> (Vertex<Self>, Vertex<Self>) {
        self.g.ends(e)
    }

    fn opposite(&self, u: Vertex<Self>, e: Edge<Self>) -> Vertex<Self> {
        self.g.opposite(u, e)
    }
}

impl<'a, 'b, G> VertexTypes<'a, Subgraph<'b, G>> for Subgraph<'b, G>
    where G: 'b + Graph
{
    type VertexIter = Cloned<Iter<'b, Vertex<G>>>;
    type NeighborIter = MapBind1<'b, IncEdgeIter<'b, Self>, G, Vertex<Self>>;
}

impl<'a, G> VertexList for Subgraph<'a, G>
    where G: 'a + Graph
{
    fn num_vertices(&self) -> usize {
        self.vertices.len()
    }

    fn vertices(&self) -> VertexIter<Self> {
        let iter: Iter<Vertex<G>> = unsafe { ::std::mem::transmute(self.vertices.iter()) };
        iter.cloned()
    }
}

impl<'a, 'b, G> EdgeTypes<'a, Subgraph<'b, G>> for Subgraph<'b, G>
    where G: 'b + Graph
{
    type EdgeIter = Cloned<Iter<'b, Edge<G>>>;
    type IncEdgeIter = Cloned<Iter<'b, Edge<G>>>;
}

impl<'a, G> EdgeList for Subgraph<'a, G>
    where G: 'a + Graph
{
    fn num_edges(&self) -> usize {
        self.edges.len()
    }

    fn edges(&self) -> EdgeIter<Self> {
        let iter: Iter<Edge<G>> = unsafe { ::std::mem::transmute(self.edges.iter()) };
        iter.cloned()
    }

    fn reverse(&self, e: Edge<Self>) -> Edge<Self> {
        self.g.reverse(e)
    }
}

impl<'a, G> Adjacency for Subgraph<'a, G>
    where G: 'a + Graph
{
    fn neighbors(&self, v: Vertex<Self>) -> NeighborIter<Self> {
        self.inc_edges(v).map_bind1(self.g, G::target)
    }

    fn degree(&self, v: Vertex<Self>) -> usize {
        self.inc[v].len()
    }
}

impl<'a, G> Incidence for Subgraph<'a, G>
    where G: 'a + Graph
{
    fn inc_edges(&self, v: Vertex<Self>) -> IncEdgeIter<Self> {
        // TODO: explain why this is safe (also vertices and edges)
        let iter: Iter<Edge<G>> = unsafe { ::std::mem::transmute(self.inc[v].iter()) };
        iter.cloned()
    }
}

impl<'a, G, T> WithVertexProp<T> for Subgraph<'a, G>
    where G: 'a + Graph + WithVertexProp<T>
{
    type VertexProp = DefaultVertexPropMut<G, T>;

    fn default_vertex_prop(&self, value: T) -> DefaultVertexPropMut<Self, T>
        where T: Clone
    {
        self.g.default_vertex_prop(value)
    }
}

impl<'a, G, T> WithEdgeProp<T> for Subgraph<'a, G>
    where G: 'a + Graph + WithEdgeProp<T>
{
    type EdgeProp = DefaultEdgePropMut<G, T>;

    fn default_edge_prop(&self, value: T) -> DefaultEdgePropMut<Self, T>
        where T: Clone
    {
        self.g.default_edge_prop(value)
    }
}

impl<'a, G, T> VertexPropMutNew<Subgraph<'a, G>, T> for DefaultVertexPropMut<G, T>
    where G: 'a + Graph + WithVertexProp<T>
{
    fn new_vertex_prop(g: &Subgraph<'a, G>, value: T) -> Self
        where T: Clone
    {
        DefaultVertexPropMut::<G, T>::new_vertex_prop(g.g, value)
    }
}

impl<'a, G, T> EdgePropMutNew<Subgraph<'a, G>, T> for DefaultEdgePropMut<G, T>
    where G: 'a + Graph + WithEdgeProp<T>
{
    fn new_edge_prop(g: &Subgraph<'a, G>, value: T) -> Self
        where T: Clone
    {
        DefaultEdgePropMut::<G, T>::new_edge_prop(g.g, value)
    }
}

impl<'a, G> BasicVertexProps for Subgraph<'a, G> where G: 'a + Graph {}

impl<'a, G> BasicEdgeProps for Subgraph<'a, G> where G: 'a + Graph {}

impl<'a, G> BasicProps for Subgraph<'a, G> where G: 'a + Graph {}


// Choose

impl<'a, G> Choose for Subgraph<'a, G>
    where G: 'a + IncidenceGraph
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

pub trait WithSubgraph<G: Graph> {
    fn spanning_subgraph(&self, edges: VecEdge<G>) -> Subgraph<G>;

    fn edge_induced_subgraph(&self, edges: VecEdge<G>) -> Subgraph<G>;

    fn induced_subgraph(&self, vertices: VecVertex<G>) -> Subgraph<G> where G: Incidence;
}


impl<G: Graph> WithSubgraph<G> for G {
    fn spanning_subgraph(&self, edges: VecEdge<G>) -> Subgraph<G> {
        // TODO: do not copy vertices
        let vertices;
        let mut inc;
        {
            let g: &G = self.borrow();
            vertices = g.vertices().into_vec();
            inc = g.default_vertex_prop(Vec::<Edge<G>>::new());
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

    fn edge_induced_subgraph(&self, edges: VecEdge<G>) -> Subgraph<G> {
        let mut vin;
        let mut vertices;
        let mut inc;
        {
            let g: &G = self.borrow();
            vin = g.default_vertex_prop(false);
            vertices = vec![];
            inc = g.default_vertex_prop(Vec::<Edge<G>>::new());
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

    fn induced_subgraph(&self, vertices: VecVertex<G>) -> Subgraph<G>
        where G: Incidence
    {
        let mut edges;
        let mut inc;
        {
            let g: &G = self.borrow();
            edges = vec![];
            inc = g.default_vertex_prop(Vec::<Edge<G>>::new());
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
        assert_eq!(set![HashSet, e02, e12], s.edges().into_hash_set());
        assert_eq!(set![HashSet, e02], s.inc_edges(0).into_hash_set());
        assert_eq!(set![HashSet, e12], s.inc_edges(1).into_hash_set());
        assert_eq!(set![HashSet, e02, e12], s.inc_edges(2).into_hash_set());
        assert_eq!(set![HashSet], s.inc_edges(3).into_hash_set());
        assert_eq!(set![HashSet], s.inc_edges(4).into_hash_set());
    }

    #[test]
    fn test_edge_induced_subgraph() {
        let (g, e01, e02, _, _) = new_graph();
        let s = g.edge_induced_subgraph(vec![e01, e02]);
        assert_eq!(set![HashSet, 0, 1, 2], s.vertices().into_hash_set());
        assert_eq!(set![HashSet, e01, e02], s.edges().into_hash_set());
        assert_eq!(set![HashSet, e01, e02], s.inc_edges(0).into_hash_set());
        assert_eq!(set![HashSet, e01], s.inc_edges(1).into_hash_set());
        assert_eq!(set![HashSet, e02], s.inc_edges(2).into_hash_set());
    }

    #[test]
    fn test_induced_subgraph() {
        let (g, e01, e02, e12, _) = new_graph();
        let s = g.induced_subgraph(vec![0, 1, 2]);
        assert_eq!(set![HashSet, 0, 1, 2], s.vertices().into_hash_set());
        assert_eq!(set![HashSet, e01, e02, e12], s.edges().into_hash_set());
        assert_eq!(set![HashSet, e01, e02], s.inc_edges(0).into_hash_set());
        assert_eq!(set![HashSet, e01, e12], s.inc_edges(1).into_hash_set());
        assert_eq!(set![HashSet, e02, e12], s.inc_edges(2).into_hash_set());
    }
}
