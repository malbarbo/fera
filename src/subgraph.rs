use graph::*;
use choose::Choose;
use common::IncidenceOutNeighborIter;
use delegateprop::*;

use fera::IteratorExt;

use std::borrow::Borrow;
use std::iter::Cloned;
use std::slice;

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

impl<'a, G> DelegateProp<G> for Subgraph<'a, G>
    where G: 'a + Graph
{
    fn delegate_prop(&self) -> &G {
        self.g
    }
}

impl<'a, 'b, G> VertexTypes<'a, Subgraph<'b, G>> for Subgraph<'b, G>
    where G: 'b + Graph
{
    type VertexIter = Cloned<slice::Iter<'a, Vertex<G>>>;
    type OutNeighborIter = IncidenceOutNeighborIter<Cloned<slice::Iter<'a, Edge<G>>>, G>;
}

impl<'a, G> WithVertex for Subgraph<'a, G>
    where G: 'a + Graph
{
    type Vertex = Vertex<G>;
    type OptionVertex = OptionVertex<G>;
}

impl<'a, 'b, G> EdgeTypes<'a, Subgraph<'b, G>> for Subgraph<'b, G>
    where G: 'b + Graph
{
    type EdgeIter = Cloned<slice::Iter<'a, Edge<G>>>;
    type OutEdgeIter = Cloned<slice::Iter<'a, Edge<G>>>;
}

impl<'a, G> WithEdge for Subgraph<'a, G>
    where G: 'a + Graph
{
    type Kind = G::Kind;
    type Edge = Edge<G>;
    type OptionEdge = OptionEdge<G>;

    fn source(&self, e: Edge<Self>) -> Vertex<Self> {
        self.g.source(e)
    }

    fn target(&self, e: Edge<Self>) -> Vertex<Self> {
        self.g.target(e)
    }

    fn orientation(&self, e: Edge<Self>) -> Orientation {
        self.g.orientation(e)
    }

    fn ends(&self, e: Edge<Self>) -> (Vertex<Self>, Vertex<Self>) {
        self.g.ends(e)
    }

    fn opposite(&self, u: Vertex<Self>, e: Edge<Self>) -> Vertex<Self> {
        self.g.opposite(u, e)
    }

    fn reverse(&self, e: Edge<Self>) -> Edge<Self> {
        self.g.reverse(e)
    }

    fn get_reverse(&self, e: Edge<Self>) -> Option<Edge<Self>> {
        self.g.get_reverse(e)
    }
}

impl<'a, G> VertexList for Subgraph<'a, G>
    where G: 'a + Graph
{
    fn num_vertices(&self) -> usize {
        self.vertices.len()
    }

    fn vertices(&self) -> VertexIter<Self> {
        self.vertices.iter().cloned()
    }
}

impl<'a, G> EdgeList for Subgraph<'a, G>
    where G: 'a + Graph
{
    fn num_edges(&self) -> usize {
        self.edges.len()
    }

    fn edges(&self) -> EdgeIter<Self> {
        self.edges.iter().cloned()
    }
}

impl<'a, G> Adjacency for Subgraph<'a, G>
    where G: 'a + Graph
{
    fn out_neighbors(&self, v: Vertex<Self>) -> OutNeighborIter<Self> {
        IncidenceOutNeighborIter::new(self.out_edges(v), self.g)
    }

    fn out_degree(&self, v: Vertex<Self>) -> usize {
        self.inc[v].len()
    }
}

impl<'a, G> Incidence for Subgraph<'a, G>
    where G: 'a + Graph
{
    fn out_edges(&self, v: Vertex<Self>) -> OutEdgeIter<Self> {
        self.inc[v].iter().cloned()
    }
}

impl<'a, G, T> WithVertexProp<T> for Subgraph<'a, G>
    where G: 'a + Graph + WithVertexProp<T>
{
    type VertexProp = DelegateVertexProp<G, T>;
}

impl<'a, G, T> WithEdgeProp<T> for Subgraph<'a, G>
    where G: 'a + Graph + WithEdgeProp<T>
{
    type EdgeProp = DelegateEdgeProp<G, T>;
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
        self.inc[v][rng.gen_range(0, self.out_degree(v))]
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
                for e in g.out_edges(u) {
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
        assert_eq!(set![HashSet, e02], s.out_edges(0).into_hash_set());
        assert_eq!(set![HashSet, e12], s.out_edges(1).into_hash_set());
        assert_eq!(set![HashSet, e02, e12], s.out_edges(2).into_hash_set());
        assert_eq!(set![HashSet], s.out_edges(3).into_hash_set());
        assert_eq!(set![HashSet], s.out_edges(4).into_hash_set());
    }

    #[test]
    fn test_edge_induced_subgraph() {
        let (g, e01, e02, _, _) = new_graph();
        let s = g.edge_induced_subgraph(vec![e01, e02]);
        assert_eq!(set![HashSet, 0, 1, 2], s.vertices().into_hash_set());
        assert_eq!(set![HashSet, e01, e02], s.edges().into_hash_set());
        assert_eq!(set![HashSet, e01, e02], s.out_edges(0).into_hash_set());
        assert_eq!(set![HashSet, 1, 2], s.out_neighbors(0).into_hash_set());
        assert_eq!(set![HashSet, e01], s.out_edges(1).into_hash_set());
        assert_eq!(set![HashSet, 0], s.out_neighbors(1).into_hash_set());
        assert_eq!(set![HashSet, e02], s.out_edges(2).into_hash_set());
        assert_eq!(set![HashSet, 0], s.out_neighbors(2).into_hash_set());
    }

    #[test]
    fn test_induced_subgraph() {
        let (g, e01, e02, e12, _) = new_graph();
        let s = g.induced_subgraph(vec![0, 1, 2]);
        assert_eq!(set![HashSet, 0, 1, 2], s.vertices().into_hash_set());
        assert_eq!(set![HashSet, e01, e02, e12], s.edges().into_hash_set());
        assert_eq!(set![HashSet, e01, e02], s.out_edges(0).into_hash_set());
        assert_eq!(set![HashSet, e01, e12], s.out_edges(1).into_hash_set());
        assert_eq!(set![HashSet, e02, e12], s.out_edges(2).into_hash_set());
    }
}
