// not working
// use super::*;
use super::{
    Basic,
    Degree,
    Adj,
    AdjIter,
    AdjIterType,
    EdgeProp,
    EdgePropType,
    VertexProp,
    VertexPropType,
    WithEdgeProp,
    WithVertexProp,
};

use std;

pub struct StaticGraph {
    num_vertices: usize,
    sources: Vec<usize>,
    targets: Vec<usize>,
    adj: Vec<Vec<usize>>,
}

impl StaticGraph {
    pub fn new_with_edges(num_vertices: usize, edges: &[(usize, usize)]) -> StaticGraph {
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

    pub fn new_empty() -> StaticGraph {
        StaticGraph::new(0, vec![], vec![])
    }

    pub fn builder(num_vertices: usize, num_edges_hint: usize) -> StaticGraphBuilder {
        let sources = Vec::with_capacity(num_edges_hint);
        let targets = Vec::with_capacity(num_edges_hint);
        StaticGraphBuilder {
            g: StaticGraph::new(num_vertices, sources, targets)
        }
    }

    fn add_edge(&mut self, u: usize, v: usize) {
        self.sources.push(u);
        self.targets.push(v);
        self.adj[u].push(v);
        self.adj[v].push(u);
    }
}

pub struct StaticGraphBuilder {
    g: StaticGraph,
}

impl StaticGraphBuilder {
    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.g.add_edge(u, v);
    }

    pub fn num_edges(&self) -> usize {
        self.g.num_edges()
    }

    pub fn finalize(self) -> StaticGraph {
        self.g
    }
}


impl Basic for StaticGraph {
    type Vertex = usize;
    type Edge = usize;
    type VertexIter = std::ops::Range<Self::Vertex>;
    type EdgeIter = std::ops::Range<Self::Vertex>;

    fn num_vertices(&self) ->  usize {
        self.num_vertices
    }

    fn vertices(&self) -> Self::VertexIter {
        0..self.num_vertices
    }

    fn source(&self, e: Self::Edge) -> Self::Vertex {
        self.sources[e]
    }

    fn target(&self, e: Self::Edge) -> Self::Vertex {
        self.targets[e]
    }

    fn num_edges(&self) -> usize {
        self.sources.len()
    }

    fn edges(&self) -> Self::EdgeIter {
        0..self.num_edges()
    }
}

impl Degree for StaticGraph {
    fn degree(&self, v: Self::Vertex) -> usize {
        self.adj[v].len()
    }
}

impl<'a> AdjIterType<'a> for StaticGraph {
    type Type = std::iter::Cloned<std::slice::Iter<'a, Self::Vertex>>;
}

impl Adj for StaticGraph {
    fn neighbors(&self, v: Self::Vertex) -> AdjIter<Self> {
        self.adj[v].iter().cloned()
    }
}

impl<'a, T> VertexPropType<'a, T> for StaticGraph {
    type Type = Vec<T>;
}

impl WithVertexProp for StaticGraph {
    fn vertex_prop<T: Clone>(&self, value: T) -> VertexProp<Self, T> {
        vec![value; self.num_vertices()]
    }
}

impl<'a, T> EdgePropType<'a, T> for StaticGraph {
    type Type = Vec<T>;
}

impl WithEdgeProp for StaticGraph {
    fn edge_prop<T: Clone>(&self, value: T) -> EdgeProp<Self, T> {
        vec![value; self.num_edges()]
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::super::tests_;

    fn new() -> StaticGraph {
        StaticGraph::new_with_edges(5, &[(0, 1), (0, 2), (1, 2), (1, 3)])
    }

    #[test] fn vertices()    { tests_::vertices(&new())    }
    #[test] fn edges()       { tests_::edges(&new())       }
    #[test] fn degree()      { tests_::degree(&new())      }
    #[test] fn neighbors()   { tests_::neighbors(&new())   }
    #[test] fn vertex_prop() { tests_::vertex_prop(&new()) }
    #[test] fn edge_prop()   { tests_::edge_prop(&new())   }

    #[test]
    fn builder() {
        let mut builder = StaticGraph::builder(3, 1);
        assert_eq!(0, builder.num_edges());

        builder.add_edge(0, 1);
        assert_eq!(1, builder.num_edges());

        builder.add_edge(1, 2);
        assert_eq!(2, builder.num_edges());

        let g = builder.finalize();
        assert_eq!(3, g.num_vertices);
        assert_eq!(vec![0, 1], g.sources);
        assert_eq!(vec![1, 2], g.targets);
        assert_eq!(vec![vec![1], vec![0, 2], vec![1]], g.adj);
    }
}
