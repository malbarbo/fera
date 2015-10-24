use graph::{Basic, Graph, PropEdge, Edge, Vertex};
use ds::{Map1, IteratorExt};

pub trait IteratorGraphExt: Sized {
    fn endvertices<G>(self, g: &G) -> Map1<Self, G, fn(&G, Edge<G>) -> (Vertex<G>, Vertex<G>)>
        where G: Basic,
              Self: Iterator<Item=Edge<G>>
    {
        self.map1(&g, G::endvertices)
    }

    fn sum_edge<G>(self, w: &PropEdge<G, f32>) -> f32
        where G: Graph,
              Self: Iterator<Item=Edge<G>>,
    {
        self.fold(0.0, |acc, e| acc + w[e])
    }
}

impl<I: Iterator> IteratorGraphExt for I { }
