use graph::*;
use ds::{Map1, IteratorExt};

// TODO: which method here is really util?
// TODO: write tests
// TODO: put methods that depends on g in a extension trait on Graph?
// TODO: turn consumers methods into functions?

pub trait IteratorGraphExt: Sized {
    fn endvertices<G>(self, g: &G) -> Map1<Self, G, fn(&G, Edge<G>) -> (Vertex<G>, Vertex<G>)>
        where G: Basic,
              Self: Iterator<Item=Edge<G>>
    {
        self.map1(&g, G::endvertices)
    }

    fn reverse_edge<G>(self, g: &G) -> Map1<Self, G, fn(&G, Edge<G>) -> Edge<G>>
        where G: Basic,
              Self: Iterator<Item=Edge<G>>
    {
        self.map1(&g, G::reverse)
    }

    fn sum_edge<G>(self, w: &PropEdge<G, f32>) -> f32
        where G: Graph,
              Self: Iterator<Item=Edge<G>>,
    {
        self.fold(0.0, |acc, e| acc + w[e])
    }

    fn max_edge<G>(self, w: &PropEdge<G, f32>) -> Edge<G>
        where G: Graph,
              Self: Iterator<Item=Edge<G>>,
    {
        use std::f32;
        let mut mw = f32::MIN;
        let mut max = None;
        for e in self {
            if w[e] > mw {
                mw = w[e];
                max = Some(e);
            }
        }
        max.unwrap()
    }

    fn max_edge_position<G>(self, w: &PropEdge<G, f32>) -> usize
        where G: Graph,
              Self: Iterator<Item=Edge<G>>,
    {
        use std::f32;
        let mut mw = f32::MIN;
        let mut pos = None;
        for (i, e) in self.enumerate() {
            if w[e] > mw {
                mw = w[e];
                pos = Some(i);
            }
        }
        pos.unwrap()
    }
}

impl<I: Iterator> IteratorGraphExt for I { }
