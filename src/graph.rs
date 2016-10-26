pub use fera::optional::Optional;

use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{Index, IndexMut};

pub type Vertex<G> = <G as WithVertex>::Vertex;
pub type OptionVertex<G> = <G as WithVertex>::OptionVertex;
pub type VertexIndexProp<G> = <G as VertexIndex>::VertexIndexProp;
pub type VertexIter<'a, G> = <G as VertexTypes<'a, G>>::VertexIter;
pub type NeighborIter<'a, G> = <G as VertexTypes<'a, G>>::NeighborIter;
pub type DefaultVertexPropMut<G, T> =
    <G as WithVertexProp<T>>::VertexProp;
pub type VecVertex<G> = Vec<Vertex<G>>;

pub type Edge<G> = <G as WithEdge>::Edge;
pub type OptionEdge<G> = <G as WithEdge>::OptionEdge;
pub type EdgeIndexProp<G> = <G as EdgeIndex>::EdgeIndexProp;
pub type EdgeIter<'a, G> = <G as EdgeTypes<'a, G>>::EdgeIter;
pub type IncEdgeIter<'a, G> = <G as EdgeTypes<'a, G>>::IncEdgeIter;
pub type DefaultEdgePropMut<G, T> = <G as WithEdgeProp<T>>::EdgeProp;
pub type VecEdge<G> = Vec<Edge<G>>;

macro_rules! items {
    ($($item:item)*) => ($($item)*);
}

macro_rules! trait_alias {
    ($name:ident = $($base:tt)+) => {
        items! {
            pub trait $name: $($base)+ { }
            impl<T: $($base)+> $name for T { }
        }
    };
}

trait_alias!(Graph = VertexList + EdgeList + BasicProps);
trait_alias!(AdjacencyGraph = Graph + Adjacency);
trait_alias!(IncidenceGraph = AdjacencyGraph + Incidence);

trait_alias!(GraphItem = Copy + Eq + Hash + Debug);

pub trait VertexTypes<'a, G: WithVertex> {
    type VertexIter: Iterator<Item = Vertex<G>>;
    type NeighborIter: Iterator<Item = Vertex<G>>;
}

pub trait WithVertex: Sized + for<'a> VertexTypes<'a, Self> {
    type Vertex: GraphItem;
    type OptionVertex: Optional<Vertex<Self>> + From<Option<Vertex<Self>>> + PartialEq + Copy;
}

pub trait WithPair<P: GraphItem>: WithVertex {
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
            panic!("u is not an end of e");
        }
    }
}

pub trait EdgeTypes<'a, G: WithEdge> {
    type EdgeIter: Iterator<Item = Edge<G>>;
    type IncEdgeIter: Iterator<Item = Edge<G>>;
}

pub trait WithEdge: Sized + WithPair<Edge<Self>> + for<'a> EdgeTypes<'a, Self> {
    type Edge: GraphItem;
    type OptionEdge: Optional<Edge<Self>> + From<Option<Edge<Self>>> + PartialEq + Copy;
}

pub trait VertexList: Sized + WithVertex {
    fn vertices(&self) -> VertexIter<Self>;

    fn num_vertices(&self) -> usize {
        self.vertices().count()
    }

    // TODO: is this necessary?
    fn vertex_none() -> OptionVertex<Self> {
        Default::default()
    }

    // TODO: is this necessary?
    fn vertex_some(v: Vertex<Self>) -> OptionVertex<Self> {
        From::from(v)
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
        Default::default()
    }

    // TODO: is this necessary?
    fn edge_some(e: Edge<Self>) -> OptionEdge<Self> {
        From::from(e)
    }
}

pub trait Adjacency: WithVertex {
    fn neighbors(&self, v: Vertex<Self>) -> NeighborIter<Self>;

    fn degree(&self, v: Vertex<Self>) -> usize {
        self.neighbors(v).count()
    }
}

pub trait Incidence: WithEdge + Adjacency {
    fn inc_edges(&self, v: Vertex<Self>) -> IncEdgeIter<Self>;
}

pub trait EdgeByEnds: WithEdge + WithVertex {
    fn edge_by_ends(&self, u: Vertex<Self>, v: Vertex<Self>) -> Option<Edge<Self>>;
}


// Index

pub trait VertexIndex: WithVertex {
    type VertexIndexProp: VertexPropGet<Self, usize>;

    fn vertex_index(&self) -> VertexIndexProp<Self>;
}

pub trait EdgeIndex: WithEdge {
    type EdgeIndexProp: EdgePropGet<Self, usize>;

    fn edge_index(&self) -> EdgeIndexProp<Self>;
}


// Properties
// TODO: explain why the trait repetition for VertexProp and EdgeProp

pub trait PropGet<I> {
    // FIXME: Rename to Value
    type Output: Sized;
    fn get(&self, key: I) -> Self::Output;
}

// Vertex

pub trait VertexPropGet<G, T>: PropGet<Vertex<G>, Output = T>
    where G: WithVertex
{
}

impl<P, G, T> VertexPropGet<G, T> for P
    where G: WithVertex,
          P: PropGet<Vertex<G>, Output = T>
{
}

pub trait VertexProp<G, T>: Index<Vertex<G>, Output = T> where G: WithVertex {}

impl<P, G, T> VertexProp<G, T> for P
    where G: WithVertex,
          P: Index<Vertex<G>, Output = T>
{
}

pub trait VertexPropMut<G, T>: IndexMut<Vertex<G>, Output = T>
    where G: WithVertex
{
    // TODO: add a way to reset the property, like: set(&mut self, value: T)
}

impl<P, G, T> VertexPropMut<G, T> for P
    where G: WithVertex,
          P: IndexMut<Vertex<G>, Output = T>
{
}

pub trait VertexPropMutNew<G, T>: VertexPropMut<G, T>
    where G: WithVertex
{
    fn new_vertex_prop(g: &G, value: T) -> Self where T: Clone;
}

pub trait WithVertexProp<T>: WithVertex {
    type VertexProp: VertexPropMutNew<Self, T>;

    fn vertex_prop<P>(&self, value: T) -> P
        where P: VertexPropMutNew<Self, T>,
              T: Clone
    {
        P::new_vertex_prop(self, value)
    }

    fn default_vertex_prop(&self, value: T) -> DefaultVertexPropMut<Self, T>
        where T: Clone
    {
        self.vertex_prop(value)
    }
}

// Edge

pub trait EdgePropGet<G, T>: PropGet<Edge<G>, Output = T> where G: WithEdge {}

impl<P, G, T> EdgePropGet<G, T> for P
    where G: WithEdge,
          P: PropGet<Edge<G>, Output = T>
{
}

pub trait EdgeProp<G, T>: Index<Edge<G>, Output = T> where G: WithEdge {}

impl<P, G, T> EdgeProp<G, T> for P
    where G: WithEdge,
          P: Index<Edge<G>, Output = T>
{
}

pub trait EdgePropMut<G, T>: IndexMut<Edge<G>, Output = T> where G: WithEdge {}

impl<P, G, T> EdgePropMut<G, T> for P
    where G: WithEdge,
          P: IndexMut<Edge<G>, Output = T>
{
}

pub trait EdgePropMutNew<G, T>: EdgePropMut<G, T>
    where G: WithEdge
{
    fn new_edge_prop(g: &G, value: T) -> Self where T: Clone;
}

pub trait WithEdgeProp<T>: WithEdge {
    type EdgeProp: EdgePropMutNew<Self, T>;

    fn edge_prop<P>(&self, value: T) -> P
        where P: EdgePropMutNew<Self, T>,
              T: Clone
    {
        P::new_edge_prop(self, value)
    }

    fn default_edge_prop(&self, value: T) -> DefaultEdgePropMut<Self, T>
        where T: Clone
    {
        self.edge_prop(value)
    }
}

// Vertex and Edge

pub trait WithProp<T>: WithVertexProp<T> + WithEdgeProp<T> {}

impl<G, T> WithProp<T> for G where G: WithVertexProp<T> + WithEdgeProp<T> {}


// Generate basic props traits

#[macro_export]
macro_rules! items {
    ($($item:item)*) => ($($item)*);
}

macro_rules! basic_props1 {
    ($($v:ty),* ; $($e:ty),* ; $($c:ty),*) => (
        items! {
            pub trait BasicVertexProps:
                $(WithVertexProp<$v> +)* { }

            pub trait BasicEdgeProps:
                $(WithEdgeProp<$e> +)* { }

            pub trait BasicProps:
                $(WithProp<$c> +)* { }
        }
    )
}

macro_rules! basic_props2 {
    ($($v:ty),* ; $($e:ty),* ; $($c:ty),* ) => (
        basic_props1! {
            $($v),+ , $(Vec<$v>),+ ;
            $($e),+ , $(Vec<$e>),+ ;
            $($c),+ , $(Vec<$c>),+
        }
    )
}

macro_rules! basic_props {
    ($($t:ty),*) => (
        basic_props2! {
            $($t),+, Vertex<Self>, OptionVertex<Self> ;
            $($t),+, Edge<Self>, OptionEdge<Self> ;
            $($t),+, Vertex<Self>, OptionVertex<Self>, Edge<Self>, OptionEdge<Self>
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
