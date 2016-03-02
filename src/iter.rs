use graph::*;
use ds::{MapFn1, IteratorExt};

// TODO: which method here is really util?
// TODO: write tests
// TODO: put methods that depends on g in a extension trait on Graph?
// TODO: turn consumers methods into functions?

pub trait IteratorGraphExt: Sized {
    fn endvertices<G>(self, g: &G) -> MapFn1<Self, G, (Vertex<G>, Vertex<G>)>
        where G: Basic,
              Self: Iterator<Item=Edge<G>>
    {
        self.map1(&g, G::endvertices)
    }

    fn reverse_edge<G>(self, g: &G) -> MapFn1<Self, G, Edge<G>>
        where G: Basic,
              Self: Iterator<Item=Edge<G>>
    {
        self.map1(&g, G::reverse)
    }

    fn sum_edge<G>(self, w: &DefaultPropMutEdge<G, f64>) -> f64
        where G: Graph,
              Self: Iterator<Item=Edge<G>>,
    {
        self.fold(0.0, |acc, e| acc + w[e])
    }

    fn max_edge<G>(self, w: &DefaultPropMutEdge<G, f64>) -> Edge<G>
        where G: Graph,
              Self: Iterator<Item=Edge<G>>,
    {
        use std::f64;
        let mut mw = f64::MIN;
        let mut max = None;
        for e in self {
            if w[e] > mw {
                mw = w[e];
                max = Some(e);
            }
        }
        max.unwrap()
    }

    fn max_edge_position<G, W>(self, w: &W) -> usize
        where G: Graph,
              W: PropEdge<G, f64>,
              Self: Iterator<Item=Edge<G>>,
    {
        use std::f64;
        let mut mw = f64::MIN;
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
