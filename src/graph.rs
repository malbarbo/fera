use iter::{IteratorExt, Map1};

use std::fmt::Debug;
use std::hash::Hash;
use std::ops::IndexMut;

use rand::Rng;


pub trait Graph: Basic + BasicProps { }

impl<G> Graph for G
    where G: Basic + BasicProps { }


pub trait Types<G: Graph>: IterTypes<G> + BasicPropTypes<G> { }

impl<G, T> Types<G> for T
    where G: Graph,
          T: IterTypes<G> + BasicPropTypes<G> { }


// Aliases

pub type Vertex<G> = <G as Basic>::Vertex;
pub type Edge<G> = <G as Basic>::Edge;

pub type IterVertex<'a, G> = <&'a G as IterTypes<G>>::Vertex;
pub type IterEdge<'a, G> = <&'a G as IterTypes<G>>::Edge;
pub type IterInc<'a, G> = <&'a G as IterTypes<G>>::Inc;

pub type PropVertex<'a, G, T> = <&'a G as PropTypes<T, G>>::Vertex;
pub type PropEdge<'a, G, T> = <&'a G as PropTypes<T, G>>::Edge;

pub type VecVertex<G> = Vec<Vertex<G>>;
pub type VecEdge<G> = Vec<Edge<G>>;

pub type OptionVertex<G> = Option<Vertex<G>>;
pub type OptionEdge<G> = Option<Edge<G>>;


// Basic

// To be implemented on &'a G
pub trait IterTypes<G: Basic> {
    type Vertex: Iterator<Item=Vertex<G>>;
    type Edge: Iterator<Item=Edge<G>>;
    type Inc: Iterator<Item=Edge<G>>;
}

pub trait Basic: Sized {
    type Vertex: Copy + Eq + Hash + Debug;
    type Edge: Copy + Eq + Hash + Debug;

    // Vertices

    fn num_vertices(&self) -> usize;

    fn vertices<'a>(&'a self) -> IterVertex<Self>
        where &'a Self: IterTypes<Self>;

    fn choose_vertex<R: Rng>(&self, rng: &mut R) -> Vertex<Self>;


    // Edges

    fn num_edges(&self) -> usize;

    fn edges<'a>(&'a self) -> IterEdge<Self>
        where &'a Self: IterTypes<Self>;

    fn source(&self, e: Edge<Self>) -> Vertex<Self>;

    fn target(&self, e: Edge<Self>) -> Vertex<Self>;

    fn endvertices(&self, e: Edge<Self>) -> (Vertex<Self>, Vertex<Self>) {
        (self.source(e), self.target(e))
    }

    fn reverse(&self, e: Edge<Self>) -> Edge<Self>;

    fn opposite(&self, u: Vertex<Self>, e: Edge<Self>) -> Vertex<Self> {
        let (s, t) = self.endvertices(e);
        if u == s {
            t
        } else if u == t {
            s
        } else {
            panic!("u is not an endvertex of e");
        }
    }

    fn choose_edge<R: Rng>(&self, rng: &mut R) -> Edge<Self>;


    // Incidence

    fn degree(&self, v: Vertex<Self>) -> usize;

    fn inc_edges<'a>(&'a self, v: Vertex<Self>) -> IterInc<Self>
        where &'a Self: IterTypes<Self>;

    fn choose_inc_edge<R: Rng>(&self, rng: &mut R, v: Vertex<Self>) -> Edge<Self>;
}


// Properties

// To be implemented on &'a G
pub trait PropTypes<T, G: Basic> {
    type Vertex: IndexMut<Vertex<G>, Output=T>;
    type Edge: IndexMut<Edge<G>, Output=T>;
}


pub trait WithProps<T: Clone>: Basic {
    fn vertex_prop<'a>(&'a self, value: T) -> PropVertex<Self, T>
        where &'a Self: PropTypes<T, Self>;

    fn edge_prop<'a>(&'a self, value: T) -> PropEdge<Self, T>
        where &'a Self: PropTypes<T, Self>;
}

macro_rules! basic_props1 {
    ($($t1:ty),* ; $($t2:ty),* ) => (
        items! {
            pub trait BasicProps:
                $(WithProps<$t1> +)* { }

            impl<G> BasicProps for G where G:
                $(WithProps<$t2> +)* { }

            pub trait BasicPropTypes<G: Basic>:
                $(PropTypes<$t2, G> +)* { }

            impl<G: Basic, T> BasicPropTypes<G> for T where T:
                $(PropTypes<$t2, G> +)* { }
        }
    )
}

macro_rules! basic_props2 {
    ($($t1:ty),* ; $($t2:ty),* ) => (
        basic_props1!{
            $($t1),+ , $(Vec<$t1>),+ , $(Option<$t1>),+ ;
            $($t2),+ , $(Vec<$t2>),+ , $(Option<$t2>),+
        }
    )
}

macro_rules! basic_props {
    ($($ty:ty),*) => (
        basic_props2!{
            Vertex<Self>, Edge<Self>, $($ty),+ ;
            Vertex<G>, Edge<G>, $($ty),+
        }
    )
}

basic_props! {
    bool,
    char,
    i8, i16, i32, i64, isize,
    u8, u16, u32, u64, usize,
    String
}


// Adjacency

pub trait Adj: Basic {
    fn neighbors<'a>(&'a self,
                     v: Vertex<Self>)
                     -> Map1<'a,
                             IterInc<'a, Self>,
                             Self,
                             fn(&'a Self, Edge<Self>) -> Vertex<Self>>
        where &'a Self: IterTypes<Self>
    {
        self.inc_edges(v).map1(self, Self::target)
    }
}

impl<G> Adj for G
    where G: Basic { }
