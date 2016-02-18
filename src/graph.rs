use ds::{IteratorExt, MapFn1};

use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{Index, IndexMut};

pub type Vertex<G> = <G as Basic>::Vertex;
pub type OptionVertex<G> = <G as Basic>::OptionVertex;
pub type PropVertex<G, T> = PropItem<Vertex<G>, Output = T>;
pub type PropMutVertex<G, T> = PropMutItem<Vertex<G>, Output = T>;
pub type DefaultPropMutVertex<G, T> = <G as WithProps<T>>::Vertex;
pub type VecVertex<G> = Vec<Vertex<G>>;

pub type Edge<G> = <G as Basic>::Edge;
pub type OptionEdge<G> = <G as Basic>::OptionEdge;
pub type PropEdge<G, T> = PropItem<Edge<G>, Output = T>;
pub type PropMutEdge<G, T> = PropMutItem<Edge<G>, Output = T>;
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

// TODO: write tests
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


// Properties

pub trait PropItem<T>: Index<T> + Clone {}

pub trait PropMutItem<T>: PropItem<T> + IndexMut<T> {}

impl<T, A: Index<T> + Clone> PropItem<T> for A {}

impl<T, A: PropItem<T> + IndexMut<T>> PropMutItem<T> for A {}

pub trait WithProps<T: Clone>: Basic {
    type Vertex: PropMutItem<Vertex<Self>, Output=T>;
    type Edge: PropMutItem<Edge<Self>, Output=T>;

    fn vertex_prop(&self, value: T) -> DefaultPropMutVertex<Self, T>;

    fn edge_prop(&self, value: T) -> DefaultPropMutEdge<Self, T>;
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
