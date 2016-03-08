use ds::{IteratorExt, MapFn1};

use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{Index, IndexMut};

pub type Vertex<G> = <G as Basic>::Vertex;
pub type OptionVertex<G> = <G as Basic>::OptionVertex;
pub type DefaultPropMutVertex<G, T> = <G as WithProps<T>>::Vertex;
pub type VecVertex<G> = Vec<Vertex<G>>;

pub type Edge<G> = <G as Basic>::Edge;
pub type OptionEdge<G> = <G as Basic>::OptionEdge;
pub type DefaultPropMutEdge<G, T> = <G as WithProps<T>>::Edge;
pub type VecEdge<G> = Vec<Edge<G>>;

pub type IterVertex<'a, G> = <G as Iterators<'a, G>>::Vertex;
pub type IterEdge<'a, G> = <G as Iterators<'a, G>>::Edge;
pub type IterInc<'a, G> = <G as Iterators<'a, G>>::Inc;


// Graph

pub trait Graph: Basic + BasicProps {}

impl<G> Graph for G where G: Basic + BasicProps {}


// Basic

pub trait Basic: Sized where for<'a> Self: Iterators<'a, Self> {
    type Vertex: 'static + Item;
    type OptionVertex: 'static + OptionItem<Vertex<Self>>;

    type Edge: 'static + Item;
    type OptionEdge: 'static + OptionItem<Edge<Self>>;


    // Vertices

    fn num_vertices(&self) -> usize;

    fn vertices(&self) -> IterVertex<Self>;

    fn vertex_none() -> OptionVertex<Self> {
        OptionVertex::<Self>::new_none()
    }

    fn vertex_some(v: Vertex<Self>) -> OptionVertex<Self> {
        OptionVertex::<Self>::new_some(v)
    }


    // Edges

    fn num_edges(&self) -> usize;

    fn edges(&self) -> IterEdge<Self>;

    fn edge_none() -> OptionEdge<Self> {
        OptionEdge::<Self>::new_none()
    }

    fn edge_some(e: Edge<Self>) -> OptionEdge<Self> {
        OptionEdge::<Self>::new_some(e)
    }

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


    // Incidence

    fn degree(&self, v: Vertex<Self>) -> usize;

    fn inc_edges(&self, v: Vertex<Self>) -> IterInc<Self>;
}


// Item

pub trait Item: Copy + Eq + Hash + Debug { }

pub trait OptionItem<T>: Clone + PartialEq {
    fn new_none() -> Self;

    fn new_some(t: T) -> Self;

    fn to_option(&self) -> Option<T>;

    #[inline(always)]
    fn is_none(&self) -> bool {
        *self == Self::new_none()
    }

    #[inline(always)]
    fn is_some(&self) -> bool {
        !self.is_none()
    }

    #[inline(always)]
    fn eq_some(&self, other: T) -> bool {
        *self == Self::new_some(other)
    }
}

impl<T: Clone + PartialEq> OptionItem<T> for Option<T> {
    fn new_none() -> Self {
        None
    }

    fn new_some(t: T) -> Self {
        Some(t)
    }

    #[inline(always)]
    fn to_option(&self) -> Option<T> {
        self.clone()
    }

    #[inline(always)]
    fn is_none(&self) -> bool {
        self.is_none()
    }

    #[inline(always)]
    fn is_some(&self) -> bool {
        self.is_some()
    }
}


// Iterators

pub trait Iterators<'a, G: Basic> {
    type Vertex: Iterator<Item=Vertex<G>>;
    type Edge: Iterator<Item=Edge<G>>;
    type Inc: Iterator<Item=Edge<G>>;
}


// Index

// TODO: Remove Clone bounds from ToIndex
pub trait ToIndex<K>: Clone {
    fn to_index(&self, k: K) -> usize;
}

#[derive(Clone)]
pub struct FnToIndex<F>(pub F);

impl<K, F: Clone + Fn(K) -> usize> ToIndex<K> for FnToIndex<F> {
    fn to_index(&self, k: K) -> usize {
        (self.0)(k)
    }
}

pub trait Indices: Basic {
    type Vertex: ToIndex<Vertex<Self>>;
    type Edge: ToIndex<Edge<Self>>;

    fn prop_vertex_index(&self) -> VertexIndex<Self>;

    fn prop_edge_index(&self) -> EdgeIndex<Self>;
}


// Properties

// TODO: Remove Clone bounds from PropVertex and EdgeVertex
pub trait PropVertex<G: Basic, T>: Index<Vertex<G>, Output = T> + Clone {}

pub trait PropMutVertex<G: Basic, T>: PropVertex<G, T> + IndexMut<Vertex<G>, Output = T> {}

impl<G: Basic, T, A: Index<Vertex<G>, Output = T> + Clone> PropVertex<G, T> for A {}

impl<G: Basic, T, A: PropVertex<G, T> + IndexMut<Vertex<G>, Output = T>> PropMutVertex<G, T> for A {}

pub trait PropMutVertexNew<G: Basic, T>: PropMutVertex<G, T> {
    fn new_prop_vertex(g: &G, value: T) -> Self;
}

pub type VertexIndex<G> = <G as Indices>::Vertex;


pub trait PropEdge<G: Basic, T>: Index<Edge<G>, Output = T> + Clone {}

pub trait PropMutEdge<G: Basic, T>: PropEdge<G, T> + IndexMut<Edge<G>, Output = T> {}

impl<G: Basic, T, A: Index<Edge<G>, Output = T> + Clone> PropEdge<G, T> for A {}

impl<G: Basic, T, A: PropEdge<G, T> + IndexMut<Edge<G>, Output = T>> PropMutEdge<G, T> for A {}

pub trait PropMutEdgeNew<G: Basic, T>: PropMutEdge<G, T> {
    fn new_prop_edge(g: &G, value: T) -> Self;
}

pub type EdgeIndex<G> = <G as Indices>::Edge;


pub fn clone_prop_vertex<G, T, P1, P2>(g: &G, src: &P1) -> P2
    where G: Graph,
          T: Default + Clone,
          P1: PropVertex<G, T>,
          P2: PropMutVertexNew<G, T>,
{
    let mut dst = P2::new_prop_vertex(g, T::default());
    for v in g.vertices() {
        dst[v] = src[v].clone();
    }
    dst
}


pub trait WithProps<T: Clone>: Basic {
    type Vertex: PropMutVertexNew<Self, T>;
    type Edge: PropMutEdgeNew<Self, T>;

    fn vertex_prop(&self, value: T) -> DefaultPropMutVertex<Self, T> {
        DefaultPropMutVertex::<Self, T>::new_prop_vertex(self, value)
    }

    fn edge_prop(&self, value: T) -> DefaultPropMutEdge<Self, T> {
        DefaultPropMutEdge::<Self, T>::new_prop_edge(self, value)
    }
}

#[macro_export]
macro_rules! items {
    ($($item:item)*) => ($($item)*);
}

macro_rules! basic_props1 {
    ($($t1:ty),* ; $($t2:ty),* ) => (
        items! {
            pub trait BasicProps:
                $(WithProps<$t1> +)* { }

            impl<G> BasicProps for G where G:
                $(WithProps<$t2> +)* { }
        }
    )
}

macro_rules! basic_props2 {
    ($($t1:ty),* ; $($t2:ty),* ) => (
        basic_props1!{
            $($t1),+ , $(Vec<$t1>),+, $(DefaultPropMutVertex<Self, $t1>),+ ;
            $($t2),+ , $(Vec<$t2>),+, $(DefaultPropMutVertex<G, $t2>),+
        }
    )
}

macro_rules! basic_props {
    ($($ty:ty),*) => (
        basic_props2!{
            Vertex<Self>, Edge<Self>, OptionVertex<Self>, OptionEdge<Self>, $($ty),+ ;
            Vertex<G>, Edge<G>, OptionVertex<G>, OptionEdge<G>, $($ty),+
        }
    )
}

basic_props! {
    bool,
    char,
    i8, i16, i32, i64, isize,
    u8, u16, u32, u64, usize,
    f32, f64,
    String
}


// Adjacency

pub trait Adj: Basic {
    fn neighbors(&self, v: Vertex<Self>) -> MapFn1<IterInc<Self>, Self, Vertex<Self>> {
        self.inc_edges(v).map1(self, Self::target)
    }
}


// TODO: Allow graphs specific implementation
impl<G> Adj for G where G: Basic {}
