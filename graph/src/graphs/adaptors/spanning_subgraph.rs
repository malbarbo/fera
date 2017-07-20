use prelude::*;
use choose::Choose;
use graphs::OutNeighborFromOutEdge;
use params::IntoOwned;
use props::{DelegateEdgeProp, DelegateVertexProp};

use std::iter::Cloned;
use std::slice;

use fera_fun::position_item;
use rand::Rng;

// FIXME: unify SpanningSubgraph with Subgraph
pub struct SpanningSubgraph<'a, G>
    where G: 'a + WithEdge + WithVertexProp<Vec<Edge<G>>>
{
    g: &'a G,
    edges: Vec<Edge<G>>,
    out_edges: DefaultVertexPropMut<G, Vec<Edge<G>>>,
}

impl<'a, G> SpanningSubgraph<'a, G>
    where G: 'a + WithEdge + WithVertexProp<Vec<Edge<G>>>
{
    #[doc(hidden)]
    pub fn new(g: &'a G) -> Self {
        SpanningSubgraph {
            g: g,
            edges: vec![],
            out_edges: g.vertex_prop(vec![]),
        }
    }

    pub fn add_edge(&mut self, e: Edge<G>) {
        let (u, v) = self.g.ends(e);
        self.edges.push(e);
        self.out_edges[u].push(e);
        if self.g.orientation(e).is_undirected() {
            self.out_edges[v].push(self.g.get_reverse(e).unwrap());
        }
    }

    pub fn add_edges<I>(&mut self, iter: I)
        where I: IntoIterator,
              I::Item: IntoOwned<Edge<G>>
    {
        for e in iter {
            self.add_edge(e.into_owned());
        }
    }

    pub fn clear_edges(&mut self)
        where G: VertexList
    {
        // FIXME: this should be linear in |E|
        self.edges.clear();
        for u in self.g.vertices() {
            self.out_edges[u].clear();
        }
    }

    pub fn replace_edge(&mut self, old: Edge<G>, new: Edge<G>) {
        self.remove_edge(old);
        self.add_edge(new);
    }

    pub fn remove_edge(&mut self, e: Edge<G>) {
        let (u, v) = self.g.ends(e);
        assert!(vec_find_swap_remove(&mut self.edges, e));
        assert!(vec_find_swap_remove(&mut self.out_edges[u], e));
        if self.g.orientation(e).is_undirected() {
            assert!(vec_find_swap_remove(&mut self.out_edges[v], e));
        }
    }
}

#[inline]
fn vec_find_swap_remove<T: PartialEq>(vec: &mut Vec<T>, value: T) -> bool {
    if let Some(i) = position_item(&*vec, &value) {
        vec.swap_remove(i);
        true
    } else {
        false
    }
}


// Trait implementations

impl<'a, G> AsRef<G> for SpanningSubgraph<'a, G>
    where G: 'a + WithEdge + WithVertexProp<Vec<Edge<G>>>
{
    #[inline]
    fn as_ref(&self) -> &G {
        self.g
    }
}

impl<'a, 'b, G> VertexTypes<'a, SpanningSubgraph<'b, G>> for SpanningSubgraph<'b, G>
    where G: 'b + WithEdge + WithVertexProp<Vec<Edge<G>>>
{
    type VertexIter = VertexIter<'b, G>;
    type OutNeighborIter = OutNeighborFromOutEdge<'b, G, OutEdgeIter<'a, Self>>;
}

impl<'a, G> WithVertex for SpanningSubgraph<'a, G>
    where G: 'a + WithEdge + WithVertexProp<Vec<Edge<G>>>
{
    type Vertex = Vertex<G>;
    type OptionVertex = OptionVertex<G>;
}

impl<'a, 'b, G> EdgeTypes<'a, SpanningSubgraph<'b, G>> for SpanningSubgraph<'b, G>
    where G: 'b + WithEdge + WithVertexProp<Vec<Edge<G>>>
{
    type EdgeIter = Cloned<slice::Iter<'a, Edge<G>>>;
    type OutEdgeIter = Cloned<slice::Iter<'a, Edge<G>>>;
}

impl<'a, G> WithEdge for SpanningSubgraph<'a, G>
    where G: 'a + WithEdge + WithVertexProp<Vec<Edge<G>>>
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

    fn end_vertices(&self, e: Edge<Self>) -> (Vertex<Self>, Vertex<Self>) {
        self.g.end_vertices(e)
    }

    fn opposite(&self, u: Vertex<Self>, e: Edge<Self>) -> Vertex<Self> {
        self.g.opposite(u, e)
    }

    // The compiler is not smart enough to allow this, so we use the default reverse
    // implemenetation
    //
    // fn reverse(&self, e: Edge<Self>) -> Edge<Self> where Self: WithEdge<Kind = Undirected> {
    //     self.g.reverse(e)
    // }

    fn get_reverse(&self, e: Edge<Self>) -> Option<Edge<Self>> {
        self.g.get_reverse(e)
    }
}

impl<'a, G> VertexList for SpanningSubgraph<'a, G>
    where G: 'a + WithEdge + WithVertexProp<Vec<Edge<G>>> + VertexList
{
    fn num_vertices(&self) -> usize {
        self.g.num_vertices()
    }

    fn vertices(&self) -> VertexIter<Self> {
        self.g.vertices()
    }
}

impl<'a, G> EdgeList for SpanningSubgraph<'a, G>
    where G: 'a + WithEdge + WithVertexProp<Vec<Edge<G>>>
{
    fn num_edges(&self) -> usize {
        self.edges.len()
    }

    fn edges(&self) -> EdgeIter<Self> {
        self.edges.iter().cloned()
    }

    fn get_edge_by_ends(&self, u: Vertex<Self>, v: Vertex<Self>) -> Option<Edge<Self>> {
        self.out_edges(u).find(|e| (u, v) == self.ends(*e))
    }
}

impl<'a, G> Adjacency for SpanningSubgraph<'a, G>
    where G: 'a + WithEdge + WithVertexProp<Vec<Edge<G>>>
{
    fn out_neighbors(&self, v: Vertex<Self>) -> OutNeighborIter<Self> {
        OutNeighborFromOutEdge::new(self.g, self.out_edges(v))
    }

    fn out_degree(&self, v: Vertex<Self>) -> usize {
        self.out_edges[v].len()
    }
}

impl<'a, G> Incidence for SpanningSubgraph<'a, G>
    where G: 'a + WithEdge + WithVertexProp<Vec<Edge<G>>>
{
    fn out_edges(&self, v: Vertex<Self>) -> OutEdgeIter<Self> {
        self.out_edges[v].iter().cloned()
    }
}

impl<'a, G, T> WithVertexProp<T> for SpanningSubgraph<'a, G>
    where G: 'a + WithEdge + WithVertexProp<Vec<Edge<G>>> + WithVertexProp<T>
{
    type VertexProp = DelegateVertexProp<G, T>;
}

impl<'a, G, T> WithEdgeProp<T> for SpanningSubgraph<'a, G>
    where G: 'a + WithEdge + WithVertexProp<Vec<Edge<G>>> + WithEdgeProp<T>
{
    type EdgeProp = DelegateEdgeProp<G, T>;
}

impl<'a, G> BasicVertexProps for SpanningSubgraph<'a, G>
    where G: 'a + WithEdge + WithVertexProp<Vec<Edge<G>>> + BasicVertexProps
{
}

impl<'a, G> BasicEdgeProps for SpanningSubgraph<'a, G>
    where G: 'a + WithEdge + WithVertexProp<Vec<Edge<G>>> + BasicEdgeProps
{
}

impl<'a, G> BasicProps for SpanningSubgraph<'a, G>
    where G: 'a + WithEdge + WithVertexProp<Vec<Edge<G>>> + BasicProps
{
}

impl<'a, G> Choose for SpanningSubgraph<'a, G>
    where G: 'a + WithEdge + WithVertexProp<Vec<Edge<G>>> + Choose
{
    fn choose_vertex<R: Rng>(&self, rng: R) -> Option<Vertex<Self>> {
        self.g.choose_vertex(rng)
    }

    fn choose_out_neighbor<R: Rng>(&self, v: Vertex<Self>, rng: R) -> Option<Vertex<Self>> {
        self.g.choose_out_edge(v, rng).map(|e| self.target(e))
    }

    fn choose_edge<R: Rng>(&self, mut rng: R) -> Option<Edge<Self>> {
        if self.num_edges() == 0 {
            None
        } else {
            // TODO: choose to reverse undirected edges?
            self.edges.get(rng.gen_range(0, self.num_edges())).cloned()
        }
    }

    fn choose_out_edge<R: Rng>(&self, v: Vertex<Self>, mut rng: R) -> Option<Edge<Self>> {
        if self.out_degree(v) == 0 {
            None
        } else {
            self.out_edges[v].get(rng.gen_range(0, self.out_degree(v))).cloned()
        }
    }
}
