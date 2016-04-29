use fera::{IteratorExt, MapBind1};
pub use fera::optional::Optional;

use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{Index, IndexMut};

pub type Vertex<G> = <G as WithVertex>::Vertex;
pub type OptionVertex<G> = <G as WithVertex>::OptionVertex;
pub type IterVertex<'a, G> = <G as VertexIterators<'a, G>>::Vertex;
pub type DefaultPropMutVertex<G, T> = <G as WithProps<T>>::Vertex;
pub type VecVertex<G> = Vec<Vertex<G>>;

pub type Edge<G> = <G as WithEdge>::Edge;
pub type OptionEdge<G> = <G as WithEdge>::OptionEdge;
pub type IterEdge<'a, G> = <G as EdgeIterators<'a, G>>::Edge;
pub type IterIncEdge<'a, G> = <G as EdgeIterators<'a, G>>::IncEdge;
pub type DefaultPropMutEdge<G, T> = <G as WithProps<T>>::Edge;
pub type VecEdge<G> = Vec<Edge<G>>;

pub trait Graph: Basic + BasicProps {}

impl<G> Graph for G where G: Basic + BasicProps {}

pub trait WithVertex {
    type Vertex: Item;
    type OptionVertex: Optional<Vertex<Self>> + Clone;
}

pub trait WithPair<P: Item>: WithVertex {
    fn source(&self, e: P) -> Vertex<Self>;

    fn target(&self, e: P) -> Vertex<Self>;

    fn ends(&self, e: P) -> (Vertex<Self>, Vertex<Self>) {
        (self.source(e), self.target(e))
    }

    fn opposite(&self, u: Vertex<Self>, e: P) -> Vertex<Self> {
        let (s, t) = self.ends(e);
        if u == s {
            t
        } else if u == t {
            s
        } else {
            panic!("u is not an endvertex of e");
        }
    }
}

pub trait WithEdge: WithPair<Edge<Self>> {
    type Edge: Item;
    type OptionEdge: Optional<Edge<Self>> + Clone;
}

pub trait VertexIterators<'a, G: WithVertex> {
    type Vertex: Iterator<Item = Vertex<G>>;
    type Neighbor: Iterator<Item = Vertex<G>>;
}

pub trait EdgeIterators<'a, G: WithEdge> {
    type Edge: Iterator<Item = Edge<G>>;
    type IncEdge: Iterator<Item = Edge<G>>;
}

pub trait VertexList: Sized + WithVertex
    where for<'a> Self: VertexIterators<'a, Self>
{
    fn vertices(&self) -> IterVertex<Self>;

    fn num_vertices(&self) -> usize {
        self.vertices().count()
    }

    fn vertex_none() -> OptionVertex<Self> {
        OptionVertex::<Self>::default()
    }

    fn vertex_some(v: Vertex<Self>) -> OptionVertex<Self> {
        OptionVertex::<Self>::from(v)
    }
}

pub trait EdgeList: Sized + WithEdge
    where for<'a> Self: EdgeIterators<'a, Self>
{
    fn edges(&self) -> IterEdge<Self>;

    fn num_edges(&self) -> usize {
        self.edges().count()
    }

    fn reverse(&self, e: Edge<Self>) -> Edge<Self>;

    fn edge_none() -> OptionEdge<Self> {
        OptionEdge::<Self>::default()
    }

    fn edge_some(e: Edge<Self>) -> OptionEdge<Self> {
        OptionEdge::<Self>::from(e)
    }
}

pub trait Basic: Sized + VertexList + EdgeList {
    fn degree(&self, v: Vertex<Self>) -> usize;

    fn inc_edges(&self, v: Vertex<Self>) -> IterIncEdge<Self>;
}

pub trait Item: Copy + Eq + Hash + Debug {}


// Iterators

pub trait Iterators<'a, G: Basic> {
    type Vertex: Iterator<Item = Vertex<G>>;
    type Edge: Iterator<Item = Edge<G>>;
    type IncEdge: Iterator<Item = Edge<G>>;
}


// Index

pub trait ToIndex<K> {
    fn to_index(&self, k: K) -> usize;
}

#[derive(Clone)]
pub struct FnToIndex<F>(pub F);

impl<K, F: Fn(K) -> usize> ToIndex<K> for FnToIndex<F> {
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
pub trait PropVertex<G: Basic, T>: Index<Vertex<G>, Output = T> {}

pub trait PropMutVertex<G: Basic, T>
    : PropVertex<G, T> + IndexMut<Vertex<G>, Output = T> {
}

impl<G: Basic, T, A: Index<Vertex<G>, Output = T>> PropVertex<G, T> for A {}

impl<G: Basic, T, A: PropVertex<G, T> + IndexMut<Vertex<G>, Output = T>> PropMutVertex<G, T> for A {}

pub trait PropMutVertexNew<G: Basic, T>: PropMutVertex<G, T> {
    fn new_prop_vertex(g: &G, value: T) -> Self where T: Clone;
}

pub type VertexIndex<G> = <G as Indices>::Vertex;


pub trait PropEdge<G: Basic, T>: Index<Edge<G>, Output = T> {}

pub trait PropMutEdge<G: Basic, T>
    : PropEdge<G, T> + IndexMut<Edge<G>, Output = T> {
}

impl<G: Basic, T, A: Index<Edge<G>, Output = T>> PropEdge<G, T> for A {}

impl<G: Basic, T, A: PropEdge<G, T> + IndexMut<Edge<G>, Output = T>> PropMutEdge<G, T> for A {}

pub trait PropMutEdgeNew<G: Basic, T>: PropMutEdge<G, T> {
    fn new_prop_edge(g: &G, value: T) -> Self where T: Clone;
}

pub type EdgeIndex<G> = <G as Indices>::Edge;


pub fn clone_prop_vertex<G, T, P1, P2>(g: &G, src: &P1) -> P2
    where G: Graph,
          T: Default + Clone,
          P1: PropVertex<G, T>,
          P2: PropMutVertexNew<G, T>
{
    let mut dst = P2::new_prop_vertex(g, T::default());
    for v in g.vertices() {
        dst[v] = src[v].clone();
    }
    dst
}


pub trait WithProps<T>: Basic {
    type Vertex: PropMutVertexNew<Self, T>;
    type Edge: PropMutEdgeNew<Self, T>;

    fn vertex_prop(&self, value: T) -> DefaultPropMutVertex<Self, T>
        where T: Clone
    {
        DefaultPropMutVertex::<Self, T>::new_prop_vertex(self, value)
    }

    fn edge_prop(&self, value: T) -> DefaultPropMutEdge<Self, T>
        where T: Clone
    {
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
    fn neighbors(&self, v: Vertex<Self>) -> MapBind1<IterIncEdge<Self>, Self, Vertex<Self>> {
        self.inc_edges(v).map_bind1(self, Self::target)
    }
}


// TODO: Allow graphs specific implementation
impl<G> Adj for G where G: Basic {}
