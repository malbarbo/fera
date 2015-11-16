use graph::*;
use choose::Choose;
use ds::IteratorExt;
use std::iter::Cloned;
use std::slice::Iter;
use rand::Rng;

// TODO: Allow a subgraph be reused

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
    fn spanning_subgraph(&self, edges: VecEdge<Self>) -> Subgraph<Self> {
        // TODO: do not copy vertices
        let vertices = self.vertices().into_vec();
        let mut inc = self.vertex_prop(Vec::<Edge<Self>>::new());
        for &e in &edges {
            let (u, v) = self.endvertices(e);
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

    fn edge_induced_subgraph(&self, edges: VecEdge<Self>) -> Subgraph<Self> {
        let mut vin = self.vertex_prop(false);
        let mut vertices = vec![];
        let mut inc = self.vertex_prop(Vec::<Edge<Self>>::new());
        for &e in &edges {
            let (u, v) = self.endvertices(e);
            if !vin[u] {
                vin[u] = true;
                vertices.push(u);
            }
            if !vin[v] {
                vin[v] = true;
                vertices.push(v);
            }

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

    fn induced_subgraph(&self, vertices: VecVertex<Self>) -> Subgraph<Self> {
        let mut edges = vec![];
        let mut inc = self.vertex_prop(Vec::<Edge<Self>>::new());
        for &u in &vertices {
            for e in self.inc_edges(u) {
                let v = self.target(e);
                // FIXME: this running time is terrible, improve
                if vertices.contains(&v) {
                    inc[u].push(e);
                    if !edges.contains(&e) {
                        edges.push(e);
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

impl<G> WithSubgraph for G
    where G: Graph,
          for<'a> &'a G: Types<G> { }


// TODO: write benchs and optimize

#[cfg(test)]
mod tests {
    use super::*;
    use graph::*;
    use static_::*;
    use ds::IteratorExt;

    fn new_graph()
        -> (StaticGraph,
            Edge<StaticGraph>,
            Edge<StaticGraph>,
            Edge<StaticGraph>,
            Edge<StaticGraph>)
    {
        let g = StaticGraph::new_with_edges(5, &[(0, 1), (0, 2), (1, 2), (3, 4)]);
        let e = g.edges().into_vec();
        (g, e[0], e[1], e[2], e[3])
    }

    #[test]
    fn test_spanning_subgraph() {
        let (g, _, e02, e12, _) = new_graph();
        let s = g.spanning_subgraph(vec![e02, e12]);
        assert_eq!(vec![0, 1, 2, 3, 4], s.vertices().into_vec());
        assert_eq!(set![e02, e12], s.edges().into_set());
        assert_eq!(set![e02], s.inc_edges(0).into_set());
        assert_eq!(set![e12], s.inc_edges(1).into_set());
        assert_eq!(set![e02, e12], s.inc_edges(2).into_set());
        assert_eq!(set![], s.inc_edges(3).into_set());
        assert_eq!(set![], s.inc_edges(4).into_set());
    }

    #[test]
    fn test_edge_induced_subgraph() {
        let (g, e01, e02, _, _) = new_graph();
        let s = g.edge_induced_subgraph(vec![e01, e02]);
        assert_eq!(set![0, 1, 2], s.vertices().into_set());
        assert_eq!(set![e01, e02], s.edges().into_set());
        assert_eq!(set![e01, e02], s.inc_edges(0).into_set());
        assert_eq!(set![e01], s.inc_edges(1).into_set());
        assert_eq!(set![e02], s.inc_edges(2).into_set());
    }

    #[test]
    fn test_induced_subgraph() {
        let (g, e01, e02, e12, _) = new_graph();
        let s = g.induced_subgraph(vec![0, 1, 2]);
        assert_eq!(set![0, 1, 2], s.vertices().into_set());
        assert_eq!(set![e01, e02, e12], s.edges().into_set());
        assert_eq!(set![e01, e02], s.inc_edges(0).into_set());
        assert_eq!(set![e01, e12], s.inc_edges(1).into_set());
        assert_eq!(set![e02, e12], s.inc_edges(2).into_set());
    }
}
