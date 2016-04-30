pub use fera::optional::Optional;

use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{Index, IndexMut};

pub type Vertex<G> = <G as WithVertex>::Vertex;
pub type OptionVertex<G> = <G as WithVertex>::OptionVertex;
pub type VertexIndexProp<G> = <G as WithVertex>::VertexIndexProp;
pub type VertexIter<'a, G> = <G as VertexTypes<'a, G>>::VertexIter;
pub type NeighborIter<'a, G> = <G as VertexTypes<'a, G>>::NeighborIter;
pub type DefaultVertexPropMut<G, T> = <G as WithProps<T>>::VertexProp;
pub type VecVertex<G> = Vec<Vertex<G>>;

pub type Edge<G> = <G as WithEdge>::Edge;
pub type OptionEdge<G> = <G as WithEdge>::OptionEdge;
pub type EdgeIndexProp<G> = <G as WithEdge>::EdgeIndexProp;
pub type EdgeIter<'a, G> = <G as EdgeTypes<'a, G>>::EdgeIter;
pub type IncEdgeIter<'a, G> = <G as EdgeTypes<'a, G>>::IncEdgeIter;
pub type DefaultEdgePropMut<G, T> = <G as WithProps<T>>::EdgeProp;
pub type VecEdge<G> = Vec<Edge<G>>;


pub trait Item: Copy + Eq + Hash + Debug {}

pub trait VertexTypes<'a, G: WithVertex> {
    type VertexIter: Iterator<Item = Vertex<G>>;
    type NeighborIter: Iterator<Item = Vertex<G>>;
}

pub trait WithVertex: Sized
    where for<'a> Self: VertexTypes<'a, Self>
{
    type Vertex: Item;
    type OptionVertex: Optional<Vertex<Self>> + Clone;
    type VertexIndexProp: ToIndex<Vertex<Self>>;
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

pub trait EdgeTypes<'a, G: WithEdge> {
    type EdgeIter: Iterator<Item = Edge<G>>;
    type IncEdgeIter: Iterator<Item = Edge<G>>;
}

pub trait WithEdge: Sized + WithPair<Edge<Self>>
    where for<'a> Self: EdgeTypes<'a, Self>
{
    type Edge: Item;
    type OptionEdge: Optional<Edge<Self>> + Clone;
    type EdgeIndexProp: ToIndex<Edge<Self>>;
}

pub trait VertexList: Sized + WithVertex {
    fn vertices(&self) -> VertexIter<Self>;

    fn num_vertices(&self) -> usize {
        self.vertices().count()
    }

    // TODO: is this necessary?
    fn vertex_none() -> OptionVertex<Self> {
        OptionVertex::<Self>::default()
    }

    // TODO: is this necessary?
    fn vertex_some(v: Vertex<Self>) -> OptionVertex<Self> {
        OptionVertex::<Self>::from(v)
    }
}

pub trait EdgeList: Sized + WithEdge {
    fn edges(&self) -> EdgeIter<Self>;

    fn num_edges(&self) -> usize {
        self.edges().count()
    }

    fn reverse(&self, e: Edge<Self>) -> Edge<Self>;

    // TODO: is this necessary?
    fn edge_none() -> OptionEdge<Self> {
        OptionEdge::<Self>::default()
    }

    // TODO: is this necessary?
    fn edge_some(e: Edge<Self>) -> OptionEdge<Self> {
        OptionEdge::<Self>::from(e)
    }
}

pub trait Undirected: VertexList + EdgeList {}

pub trait Adjacency: WithVertex {
    fn neighbors(&self, v: Vertex<Self>) -> NeighborIter<Self>;

    fn degree(&self, v: Vertex<Self>) -> usize {
        self.neighbors(v).count()
    }
}

pub trait Incidence: WithEdge + Adjacency {
    fn inc_edges(&self, v: Vertex<Self>) -> IncEdgeIter<Self>;
}

// Index

pub trait ToIndex<K> {
    fn to_index(&self, k: K) -> usize;
}

#[derive(Clone)]
pub struct FnToIndex<F>(pub F);

impl<K, F> ToIndex<K> for FnToIndex<F>
    where F: Fn(K) -> usize
{
    fn to_index(&self, k: K) -> usize {
        (self.0)(k)
    }
}

pub trait VertexIndex: WithVertex {
    fn vertex_index(&self) -> VertexIndexProp<Self>;
}

pub trait EdgeIndex: WithEdge {
    fn edge_index(&self) -> EdgeIndexProp<Self>;
}


// Properties

pub trait VertexProp<G, T>: Index<Vertex<G>, Output = T> where G: WithVertex {}

pub trait VertexPropMut<G, T>
    : VertexProp<G, T> + IndexMut<Vertex<G>, Output = T>
    where G: WithVertex
{
}

impl<G, T, A> VertexProp<G, T> for A
    where G: WithVertex,
          A: Index<Vertex<G>, Output = T>
{
}

impl<G, T, A> VertexPropMut<G, T> for A
    where G: WithVertex,
          A: VertexProp<G, T> + IndexMut<Vertex<G>, Output = T>
{
}

pub trait VertexPropMutNew<G, T>: VertexPropMut<G, T>
    where G: WithVertex
{
    fn new_vertex_prop(g: &G, value: T) -> Self where T: Clone;
}

pub trait EdgeProp<G, T>: Index<Edge<G>, Output = T> where G: WithEdge {}

pub trait EdgePropMut<G, T>: EdgeProp<G, T> + IndexMut<Edge<G>, Output = T>
    where G: WithEdge
{
}

impl<G, T, A> EdgeProp<G, T> for A
    where G: WithEdge,
          A: Index<Edge<G>, Output = T>
{
}

impl<G, T, A> EdgePropMut<G, T> for A
    where G: WithEdge,
          A: EdgeProp<G, T> + IndexMut<Edge<G>, Output = T>
{
}

pub trait EdgePropMutNew<G, T>: EdgePropMut<G, T>
    where G: WithEdge
{
    fn new_edge_prop(g: &G, value: T) -> Self where T: Clone;
}

pub trait WithProps<T>: Undirected {
    type VertexProp: VertexPropMutNew<Self, T>;
    type EdgeProp: EdgePropMutNew<Self, T>;

    fn vertex_prop(&self, value: T) -> DefaultVertexPropMut<Self, T>
        where T: Clone
    {
        DefaultVertexPropMut::<Self, T>::new_vertex_prop(self, value)
    }

    fn edge_prop(&self, value: T) -> DefaultEdgePropMut<Self, T>
        where T: Clone
    {
        DefaultEdgePropMut::<Self, T>::new_edge_prop(self, value)
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
            $($t1),+ , $(Vec<$t1>),+, $(DefaultVertexPropMut<Self, $t1>),+ ;
            $($t2),+ , $(Vec<$t2>),+, $(DefaultVertexPropMut<G, $t2>),+
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
