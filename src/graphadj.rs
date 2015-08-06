use std;
use super::*;

pub struct StaticGraph {
    num_vertices: usize,
    sources: Vec<usize>,
    targets: Vec<usize>,
    adj: Vec<Vec<usize>>,
}

impl StaticGraph {
    pub fn new_edges(num_vertices: usize, edges: &[(usize, usize)]) -> StaticGraph {
        StaticGraph::new(num_vertices,
                         edges.iter().map(|e| e.0).collect(),
                         edges.iter().map(|e| e.1).collect())
    }

    pub fn new(num_vertices: usize, sources: Vec<usize>, targets: Vec<usize>) -> StaticGraph {
        let mut adj = vec![vec![]; num_vertices];
        for (u, v) in sources.iter().zip(targets.iter()) {
            // TODO: is u and v valid?
            adj[*u].push(*v);
            adj[*v].push(*u);
        }
        StaticGraph{
            num_vertices: num_vertices,
            sources: sources,
            targets: targets,
            adj: adj,
        }
    }
}


impl<'a> Basic<'a> for StaticGraph {
    type Vertex = usize;
    type Edge = usize;
    type VertexIter = std::ops::Range<Self::Vertex>;
    type EdgeIter = std::ops::Range<Self::Vertex>;

    fn num_vertices(&'a self) ->  usize {
        self.num_vertices
    }

    fn vertices(&'a self) -> Self::VertexIter {
        0..self.num_vertices
    }

    fn source(&'a self, e: Self::Edge) -> Self::Vertex {
        self.sources[e]
    }

    fn target(&'a self, e: Self::Edge) -> Self::Vertex {
        self.targets[e]
    }

    fn num_edges(&'a self) -> usize {
        self.sources.len()
    }

    fn edges(&'a self) -> Self::EdgeIter {
        0..self.num_edges()
    }
}

impl<'a> Degree<'a> for StaticGraph {
    fn degree(&'a self, v: Self::Vertex) -> usize {
        self.adj[v].len()
    }
}

impl<'a> Adj<'a> for StaticGraph {
    type NeighborsIter = std::iter::Cloned<std::slice::Iter<'a, Self::Vertex>>;
    fn neighbors(&'a self, v: Self::Vertex) -> Self::NeighborsIter {
        self.adj[v].iter().cloned()
    }
}

impl<'a, T> VertexPropType<'a, T> for StaticGraph {
    type Type = std::vec::Vec<T>;
}

impl WithVertexProp for StaticGraph {
    fn vertex_prop<T: Clone>(&self, value: T) -> <Self as VertexPropType<T>>::Type {
        vec![value; self.num_vertices()]
    }
}

impl<'a, T> EdgePropType<'a, T> for StaticGraph {
    type Type = std::vec::Vec<T>;
}

impl WithEdgeProp for StaticGraph {
    fn edge_prop<T: Clone>(&self, value: T) -> <Self as EdgePropType<T>>::Type {
        vec![value; self.num_edges()]
    }
}


#[cfg(test)]
mod tests_ {
    use super::*;
    use super::super::tests;

    fn new() -> StaticGraph {
        StaticGraph::new_edges(5, &[(0, 1), (0, 2), (1, 2), (1, 3)])
    }

    #[test] fn vertices()    { tests::vertices(&new())  }
    #[test] fn edges()       { tests::edges(&new())     }
    #[test] fn degree()      { tests::degree(&new())    }
    #[test] fn neighbors()   { tests::neighbors(&new()) }
    #[test] fn vertex_prop() { tests::vertices(&new())  }
    #[test] fn edge_prop()   { tests::vertices(&new())  }
}
