//! Iterators for edge and vertex set complements.

use prelude::*;
use params::IntoOwned;

pub trait Sets {
    fn vertices_complement<I>(&self, vertices: I) -> VerticesComplement<Self>
    where
        Self: VertexList + WithVertexProp<bool>,
        I: IntoIterator,
        I::Item: IntoOwned<Vertex<Self>>,
    {
        let mut set = self.default_vertex_prop(false);
        set.set_values(vertices, true);
        VerticesComplement {
            vertices: self.vertices(),
            set,
        }
    }

    fn edges_complement<I>(&self, edges: I) -> EdgesComplement<Self>
    where
        Self: EdgeList + WithEdgeProp<bool>,
        I: IntoIterator,
        I::Item: IntoOwned<Edge<Self>>,
    {
        let mut set = self.default_edge_prop(false);
        set.set_values(edges, true);
        EdgesComplement {
            edges: self.edges(),
            set,
        }
    }
}

impl<G> Sets for G {}

pub struct VerticesComplement<'a, G>
where
    G: 'a + WithVertex + WithVertexProp<bool>,
{
    vertices: VertexIter<'a, G>,
    set: DefaultVertexPropMut<G, bool>,
}

impl<'a, G> Iterator for VerticesComplement<'a, G>
where
    G: 'a + WithVertex + WithVertexProp<bool>,
{
    type Item = Vertex<G>;

    fn next(&mut self) -> Option<Self::Item> {
        for e in self.vertices.by_ref() {
            if !self.set[e] {
                return Some(e);
            }
        }
        None
    }
}

pub struct EdgesComplement<'a, G>
where
    G: 'a + WithEdge + WithEdgeProp<bool>,
{
    edges: EdgeIter<'a, G>,
    set: DefaultEdgePropMut<G, bool>,
}

impl<'a, G> Iterator for EdgesComplement<'a, G>
where
    G: 'a + WithEdge + WithEdgeProp<bool>,
{
    type Item = Edge<G>;

    fn next(&mut self) -> Option<Self::Item> {
        for e in self.edges.by_ref() {
            if !self.set[e] {
                return Some(e);
            }
        }
        None
    }
}
