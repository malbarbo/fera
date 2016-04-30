pub use fera::optional::Optional;

use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{Index, IndexMut};

pub type Vertex<G> = <G as WithVertex>::Vertex;
pub type OptionVertex<G> = <G as WithVertex>::OptionVertex;
pub type IterVertex<'a, G> = <G as VertexIterators<'a, G>>::Vertex;
pub type IterNeighbor<'a, G> = <G as VertexIterators<'a, G>>::Neighbor;
pub type DefaultPropMutVertex<G, T> = <G as WithProps<T>>::Vertex;
pub type VecVertex<G> = Vec<Vertex<G>>;

pub type Edge<G> = <G as WithEdge>::Edge;
pub type OptionEdge<G> = <G as WithEdge>::OptionEdge;
pub type IterEdge<'a, G> = <G as EdgeIterators<'a, G>>::Edge;
pub type IterIncEdge<'a, G> = <G as EdgeIterators<'a, G>>::IncEdge;
pub type DefaultPropMutEdge<G, T> = <G as WithProps<T>>::Edge;
pub type VecEdge<G> = Vec<Edge<G>>;

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

    // TODO: is this necessary?
    fn vertex_none() -> OptionVertex<Self> {
        OptionVertex::<Self>::default()
    }

    // TODO: is this necessary?
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

pub trait Neighbors: VertexList {
    fn neighbors(&self, v: Vertex<Self>) -> IterNeighbor<Self>;

    fn degree(&self, v: Vertex<Self>) -> usize {
        self.neighbors(v).count()
    }
}

pub trait IncEdges: Undirected + Neighbors {
    fn inc_edges(&self, v: Vertex<Self>) -> IterIncEdge<Self>;
}

pub trait Item: Copy + Eq + Hash + Debug {}

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

pub trait Indices: VertexList + EdgeList {
    type Vertex: ToIndex<Vertex<Self>>;
    type Edge: ToIndex<Edge<Self>>;

    fn prop_vertex_index(&self) -> VertexIndex<Self>;

    fn prop_edge_index(&self) -> EdgeIndex<Self>;
}


// Properties

// TODO: Remove Clone bounds from PropVertex and EdgeVertex
pub trait PropVertex<G, T>: Index<Vertex<G>, Output = T> where G: VertexList {}

pub trait PropMutVertex<G, T>
    : PropVertex<G, T> + IndexMut<Vertex<G>, Output = T>
    where G: VertexList
{
}

impl<G, T, A> PropVertex<G, T> for A
    where G: VertexList,
          A: Index<Vertex<G>, Output = T>
{
}

impl<G, T, A> PropMutVertex<G, T> for A
    where G: VertexList,
          A: PropVertex<G, T> + IndexMut<Vertex<G>, Output = T>
{
}

pub trait PropMutVertexNew<G, T>: PropMutVertex<G, T>
    where G: VertexList
{
    fn new_prop_vertex(g: &G, value: T) -> Self where T: Clone;
}

pub type VertexIndex<G> = <G as Indices>::Vertex;


pub trait PropEdge<G, T>: Index<Edge<G>, Output = T> where G: EdgeList {}

pub trait PropMutEdge<G, T>: PropEdge<G, T> + IndexMut<Edge<G>, Output = T>
    where G: EdgeList
{
}

impl<G, T, A> PropEdge<G, T> for A
    where G: EdgeList,
          A: Index<Edge<G>, Output = T>
{
}

impl<G, T, A> PropMutEdge<G, T> for A
    where G: EdgeList,
          A: PropEdge<G, T> + IndexMut<Edge<G>, Output = T>
{
}

pub trait PropMutEdgeNew<G, T>: PropMutEdge<G, T>
    where G: EdgeList
{
    fn new_prop_edge(g: &G, value: T) -> Self where T: Clone;
}

pub type EdgeIndex<G> = <G as Indices>::Edge;


pub trait WithProps<T>: Undirected {
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
