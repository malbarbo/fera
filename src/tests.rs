use graph::*;
use std::collections::HashSet;
use std::hash::Hash;
use std::fmt::Debug;
use iter::{Map1, IteratorExt};

pub trait IteratorGraph<G: Basic>: Iterator<Item=G::Edge> + Sized {
    fn endvertices(self,
                   g: &G)
                   -> Map1<Self, G, fn(&G, G::Edge) -> (G::Vertex, G::Vertex)> {
        self.map1(&g, G::endvertices)
    }
}

impl<G: Basic, I: Iterator<Item=G::Edge>> IteratorGraph<G> for I {}

pub trait Builder {
    type G: Basic;

    fn new(num_vertices: usize,
           edges: &[(usize, usize)])
           -> (G<Self>, Vec<V<Self>>, Vec<E<Self>>);
}

pub type G<B> = <B as Builder>::G;
pub type V<B> = <<B as Builder>::G as Basic>::Vertex;
pub type E<B> = <<B as Builder>::G as Basic>::Edge;

fn new<B: Builder>() -> (B::G, Vec<V<B>>, Vec<E<B>>) {
    B::new(5, &[(0, 1), (0, 2), (1, 2), (1, 3)])
}

#[macro_export]
macro_rules! test_basic (
    ($B:ty) => {
        #[test] fn vertices(){
            test_vertices::<$B>()
        }

        #[test] fn edges() {
            test_edges::<$B>()
        }
    }
);

#[macro_export]
macro_rules! test_degree (
    ($B:ty) => {
        #[test] fn degree() {
            test_degree::<$B>()
        }
    }
);

#[macro_export]
macro_rules! test_inc (
    ($B:ty) => {
        #[test] fn inc_edges_one_edge() {
            test_inc_edges_one_edge::<$B>()
        }

        #[test] fn inc_edges() {
            test_inc_edges::<$B>()
        }
    }
);

#[macro_export]
macro_rules! test_adj (
    ($B:ty) => {
        #[test] fn neighbors() {
            test_neighbors::<$B>()
        }
    }
);

#[macro_export]
macro_rules! test_vertex_prop (
    ($B:ty) => {
        #[test] fn vertex_prop() {
            test_vertex_prop::<$B>()
        }
    }
);

#[macro_export]
macro_rules! test_edge_prop (
    ($B:ty) => {
        #[test] fn edge_prop() {
            test_edge_prop::<$B>()
        }
    }
);


pub fn test_vertices<B: Builder>()
    where V<B>: Debug
{
    let (g, vertices, _) = new::<B>();
    assert_eq!(5, vertices.len());
    assert_eq!(5, g.num_vertices());
    assert_eq!(vertices, g.vertices().as_vec());
}

pub fn test_edges<B: Builder>()
    where V<B>: Debug,
          E<B>: Debug
{
    let (g, v, edges) = new::<B>();
    assert_eq!(4, edges.len());
    assert_eq!(4, g.num_edges());
    assert_eq!(edges, g.edges().as_vec());
    assert_eq!(vec![(v[0], v[1]), (v[0], v[2]), (v[1], v[2]), (v[1], v[3])],
               g.edges().endvertices(&g).as_vec());
}

pub fn test_degree<B: Builder>()
    where G<B>: Degree
{
    let (g, v, _) = new::<B>();
    assert_eq!(2, g.degree(v[0]));
    assert_eq!(3, g.degree(v[1]));
    assert_eq!(2, g.degree(v[2]));
    assert_eq!(1, g.degree(v[3]));
    assert_eq!(0, g.degree(v[4]));
}

pub fn test_inc_edges_one_edge<B: Builder>()
    where G<B>: Inc,
          V<B>: Debug,
          E<B>: Debug
{
    let (g, v, _) = B::new(2, &[(0, 1)]);
    let e = g.edges().next().unwrap();
    let ab = g.inc_edges(v[0]).next().unwrap();
    let ba = g.inc_edges(v[1]).next().unwrap();
    assert_eq!(e, ab);
    assert_eq!(e, ba);
    assert_eq!(ab, ba);
    assert_eq!(v[0], g.source(ab));
    assert_eq!(v[1], g.target(ab));
    assert_eq!(v[1], g.source(ba));
    assert_eq!(v[0], g.target(ba));
}

pub fn test_inc_edges<B: Builder>()
    where G<B>: Inc,
          V<B>: Debug,
          E<B>: Debug + Hash
{
    let (g, v, e) = new::<B>();
    assert_eq!(set![e[0], e[1]],
               g.inc_edges(v[0]).as_set());
    assert_eq!(set![e[0], e[2], e[3]],
               g.inc_edges(v[1]).as_set());
    assert_eq!(set![e[1], e[2]],
               g.inc_edges(v[2]).as_set());
    assert_eq!(set![e[3]],
               g.inc_edges(v[3]).as_set());
    assert_eq!(set![],
               g.inc_edges(v[4]).as_set());
}

pub fn test_neighbors<B: Builder>()
    where G<B>: Adj,
          V<B>: Debug + Hash
{
    let (g, v, _) = new::<B>();
    assert_eq!(set![v[1], v[2]],
               g.neighbors(v[0]).as_set());
    assert_eq!(set![v[0], v[2], v[3]],
               g.neighbors(v[1]).as_set());
    assert_eq!(set![v[0], v[1]],
               g.neighbors(v[2]).as_set());
    assert_eq!(set![v[1]],
               g.neighbors(v[3]).as_set());
    assert_eq!(set![],
               g.neighbors(v[4]).as_set());
}

pub fn test_vertex_prop<B: Builder>()
    where G<B>: WithVertexProp
{
    let (g, v, _) = new::<B>();
    let mut x = g.vertex_prop(0usize);
    let mut y = g.vertex_prop("a");
    let (a, b, c, d, e) = (v[0], v[1], v[2], v[3], v[4]);
    x[c] = 8;
    y[d] = "b";
    assert_eq!(0, x[a]);
    assert_eq!(0, x[b]);
    assert_eq!(8, x[c]);
    assert_eq!(0, x[d]);
    assert_eq!(0, x[e]);
    assert_eq!("a", y[a]);
    assert_eq!("a", y[b]);
    assert_eq!("a", y[c]);
    assert_eq!("b", y[d]);
    assert_eq!("a", y[e]);
}

pub fn test_edge_prop<B: Builder>()
    where G<B>: WithEdgeProp
{
    let (g, _, e) = new::<B>();
    let mut x = g.edge_prop(0usize);
    let mut y = g.edge_prop("a");
    let (a, b, c, d) = (e[0], e[1], e[2], e[3]);
    x[c] = 8;
    y[d] = "b";
    assert_eq!(0, x[a]);
    assert_eq!(0, x[b]);
    assert_eq!(8, x[c]);
    assert_eq!(0, x[d]);
    assert_eq!("a", y[a]);
    assert_eq!("a", y[b]);
    assert_eq!("a", y[c]);
    assert_eq!("b", y[d]);
}
