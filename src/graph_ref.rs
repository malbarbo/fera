use graph::*;

impl<'a, 'b, G: WithVertex> VertexTypes<'a, &'b G> for &'b G {
    type VertexIter = VertexIter<'a, G>;
    type NeighborIter = NeighborIter<'a, G>;
}

impl<'a, G: WithVertex> WithVertex for &'a G {
    type Vertex = Vertex<G>;
    type OptionVertex = OptionVertex<G>;
    type VertexIndexProp = VertexIndexProp<G>;
}

impl<'a, G: WithEdge> WithPair<Edge<&'a G>> for &'a G {
    fn source(&self, e: Edge<Self>) -> Vertex<Self> {
        G::source(self, e)
    }

    fn target(&self, e: Edge<Self>) -> Vertex<Self> {
        G::target(self, e)
    }

    fn ends(&self, e: Edge<Self>) -> (Vertex<Self>, Vertex<Self>) {
        G::ends(self, e)
    }

    fn opposite(&self, u: Vertex<Self>, e: Edge<Self>) -> Vertex<Self> {
        G::opposite(self, u, e)
    }
}

impl<'a, 'b, G: WithEdge> EdgeTypes<'a, &'b G> for &'b G {
    type EdgeIter = EdgeIter<'a, G>;
    type IncEdgeIter = IncEdgeIter<'a, G>;
}

impl<'a, G: WithEdge> WithEdge for &'a G {
    type Edge = Edge<G>;
    type OptionEdge = OptionEdge<G>;
    type EdgeIndexProp = EdgeIndexProp<G>;
}

impl<'a, G: VertexList> VertexList for &'a G {
    fn vertices(&self) -> VertexIter<Self> {
        G::vertices(self)
    }

    fn num_vertices(&self) -> usize {
        G::num_vertices(self)
    }

    fn vertex_none() -> OptionVertex<Self> {
        G::vertex_none()
    }

    fn vertex_some(v: Vertex<Self>) -> OptionVertex<Self> {
        G::vertex_some(v)
    }
}

impl<'a, G: EdgeList> EdgeList for &'a G {
    fn edges(&self) -> EdgeIter<Self> {
        G::edges(self)
    }

    fn num_edges(&self) -> usize {
        G::num_edges(self)
    }

    fn reverse(&self, e: Edge<Self>) -> Edge<Self> {
        G::reverse(self, e)
    }

    fn edge_none() -> OptionEdge<Self> {
        G::edge_none()
    }

    fn edge_some(e: Edge<Self>) -> OptionEdge<Self> {
        G::edge_some(e)
    }
}

impl<'a, G: Adjacency> Adjacency for &'a G {
    fn neighbors(&self, v: Vertex<Self>) -> NeighborIter<Self> {
        G::neighbors(self, v)
    }

    fn degree(&self, v: Vertex<Self>) -> usize {
        G::degree(self, v)
    }
}

impl<'a, G: Incidence> Incidence for &'a G {
    fn inc_edges(&self, v: Vertex<Self>) -> IncEdgeIter<Self> {
        G::inc_edges(self, v)
    }
}

impl<'a, G: VertexIndex> VertexIndex for &'a G {
    fn vertex_index(&self) -> VertexIndexProp<Self> {
        G::vertex_index(self)
    }
}

impl<'a, G: EdgeIndex> EdgeIndex for &'a G {
    fn edge_index(&self) -> EdgeIndexProp<Self> {
        G::edge_index(self)
    }
}

// TODO: complete the Grapf implementation for &G

// impl<'a, G: WithVertexProp<T>, T> WithVertexProp<T> for &'a G {
//     type VertexProp = DefaultVertexPropMut<G, T>;
//
//     fn vertex_prop<P>(&self, value: T) -> P
//         where P: VertexPropMutNew<Self, T>,
//               T: Clone
//     {
//         G::vertex_prop(*self, value)
//     }
//
//     fn default_vertex_prop(&self, value: T) -> DefaultVertexPropMut<Self, T>
//         where T: Clone
//     {
//         G::default_vertex_prop(self, value)
//     }
// }

// FIXME: this implementation conflicts with the one in arrayprop.rs
//
// impl<'a, T, G> VertexPropMutNew<&'a G, T> for DefaultVertexPropMut<G, T>
//     where G: WithVertexProp<T>,
// {
//     fn new_vertex_prop(g: &&'a G, value: T) -> Self where T: Clone {
//         unimplemented!()
//     }
// }
