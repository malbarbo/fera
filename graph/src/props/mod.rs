//! Traits and implementation for properties (key to values mapping).
//!
//! # Examples
//!
//! ```
//! use fera_graph::prelude::*;
//!
//! let g = CompleteGraph::new(4);
//! let mut p = g.default_vertex_prop(0i32);
//! p[3] = -3;
//! assert_eq!(0, p.get(0));
//! assert_eq!(-3, p.get(3));
//! let abs_p = p.map(i32::abs);
//! assert_eq!(3, abs_p.get(3));
//! ```

use prelude::*;
use std::ops::{Index, IndexMut};

mod array;
mod delegate;
#[path="fn.rs"]
mod fn_;
mod hashmap;
mod ignore;

pub use self::array::*;
pub use self::delegate::*;
pub use self::fn_::*;
pub use self::hashmap::*;
pub use self::ignore::*;

use ext::IntoOwned;

/// An abstract property that maps keys in domain `K` to the corresponding values.
pub trait PropGet<K> {
    type Output: Sized;

    /// Returns the value associated with `key`.
    fn get(&self, key: K) -> Self::Output;

    /// Creates a mapped property that maps each property value using `fun`.
    #[inline]
    fn map<F, O>(self, fun: F) -> Map<Self, F>
        where Self: Sized,
              F: Fn(Self::Output) -> O
    {
        Map(self, fun)
    }

    /// Returns a reference to this property.
    #[inline]
    fn by_ref(&self) -> &Self {
        self
    }
}

impl<'a, K, P: PropGet<K>> PropGet<K> for &'a P {
    type Output = P::Output;

    #[inline]
    fn get(&self, key: K) -> Self::Output {
        P::get(self, key)
    }
}


// Indexable properties

// TODO: turn PropIndexMut into a extension trait
/// A property that can be read/write using indexing operations.
pub trait PropIndexMut<Idx>: IndexMut<Idx> {
    /// Set the value associated with each key produced by `iter` to the value associated with the
    /// key in the property `source`.
    fn set_values_from<P, I>(&mut self, iter: I, source: &P)
        where I: IntoIterator,
              I::Item: IntoOwned<Idx>,
              Idx: Clone,
              P: Index<Idx, Output = Self::Output>,
              Self::Output: Clone
    {
        for v in iter {
            let v = v.into_owned();
            self[v.clone()].clone_from(&source[v]);
        }
    }

    /// Set the value associated with keys produced by `iter` to `value`.
    fn set_values<I>(&mut self, iter: I, value: Self::Output)
        where I: IntoIterator,
              I::Item: IntoOwned<Idx>,
              Self::Output: Clone
    {
        for v in iter {
            self[v.into_owned()].clone_from(&value);
        }
    }
}

impl<P: IndexMut<Idx>, Idx> PropIndexMut<Idx> for P {}


// TODO: explain why the trait repetition for VertexProp and EdgeProp (missing trait alias?)

// Vertex

/// A vertex property.
pub trait VertexPropGet<G, T>: PropGet<Vertex<G>, Output = T>
    where G: WithVertex
{
}

impl<P, G, T> VertexPropGet<G, T> for P
    where G: WithVertex,
          P: PropGet<Vertex<G>, Output = T>
{
}

/// A vertex property that can be read using indexing operation.
pub trait VertexProp<G, T>: Index<Vertex<G>, Output = T> where G: WithVertex {}

impl<P, G, T> VertexProp<G, T> for P
    where G: WithVertex,
          P: Index<Vertex<G>, Output = T>
{
}

/// A vertex property that can be read/write using indexing operation.
pub trait VertexPropMut<G, T>: IndexMut<Vertex<G>, Output = T>
    where G: WithVertex
{
}

impl<P, G, T> VertexPropMut<G, T> for P
    where G: WithVertex,
          P: IndexMut<Vertex<G>, Output = T>
{
}

/// A vertex property that can be created using a graph reference and a value.
pub trait VertexPropMutNew<G, T>: VertexPropMut<G, T>
    where G: WithVertex
{
    /// Creates a new vertex prop.
    ///
    /// This method is rarely called explicitly, it is instead used through
    /// [`WithVertex::vertex_prop`].
    ///
    /// [`WithVertex::vertex_prop`]: ../trait.WithVertex.html#method.vertex_prop
    fn new_vertex_prop(g: &G, value: T) -> Self where T: Clone;
}

/// A graph that has a default vertex property type, that is, has a default implementation to
/// associated values with vertices.
pub trait WithVertexProp<T>: WithVertex {
    /// The vertex property type.
    type VertexProp: VertexPropMutNew<Self, T>;

    /// Creates a new default vertex property where the initial value associated with each vertex
    /// is `value`.
    fn default_vertex_prop(&self, value: T) -> DefaultVertexPropMut<Self, T>
        where T: Clone
    {
        self.vertex_prop(value)
    }

    /// Creates a new default vertex property where the initial value associated with each vertex
    /// `v` is produced by `fun(v)`.
    fn default_vertex_prop_from_fn<P, F>(&self, fun: F) -> P
        where Self: VertexList,
              P: VertexPropMutNew<Self, T>,
              F: FnMut(Vertex<Self>) -> T,
              T: Default + Clone
    {
        self.vertex_prop_from_fn(fun)
    }
}


// Edge

/// A edge property.
pub trait EdgePropGet<G, T>: PropGet<Edge<G>, Output = T> where G: WithEdge {}

impl<P, G, T> EdgePropGet<G, T> for P
    where G: WithEdge,
          P: PropGet<Edge<G>, Output = T>
{
}

/// An edge property that can be read using indexing operation.
pub trait EdgeProp<G, T>: Index<Edge<G>, Output = T> where G: WithEdge {}

impl<P, G, T> EdgeProp<G, T> for P
    where G: WithEdge,
          P: Index<Edge<G>, Output = T>
{
}

/// A edge property that can be read/write using indexing operation.
pub trait EdgePropMut<G, T>: IndexMut<Edge<G>, Output = T>
    where G: WithEdge
{
}

impl<P, G, T> EdgePropMut<G, T> for P
    where G: WithEdge,
          P: IndexMut<Edge<G>, Output = T>
{
}

/// An edge property that can be read/write using indexing operation.
pub trait EdgePropMutNew<G, T>: EdgePropMut<G, T>
    where G: WithEdge
{
    /// Creates a new edge prop.
    ///
    /// This method is rarely called explicitly, it is instead used through
    /// [`WithEdge::edge_prop`].
    ///
    /// [`WithEdge::edge_prop`]: ../trait.WithEdge.html#method.edge_prop
    fn new_edge_prop(g: &G, value: T) -> Self where T: Clone;
}

/// A graph that has a default edge property type, that is, has a default implementation to
/// associated values with edges.
pub trait WithEdgeProp<T>: WithEdge {
    type EdgeProp: EdgePropMutNew<Self, T>;

    /// Creates a new default edge property where the initial value associated with each edge is
    /// `value`.
    fn default_edge_prop(&self, value: T) -> DefaultEdgePropMut<Self, T>
        where T: Clone
    {
        self.edge_prop(value)
    }

    /// Creates a new default edge property where the initial value associated with each edge `e`
    /// is produced by `fun(e)`.
    fn default_edge_prop_from_fn<P, F>(&self, fun: F) -> P
        where Self: EdgeList,
              P: EdgePropMutNew<Self, T>,
              F: FnMut(Edge<Self>) -> T,
              T: Default + Clone
    {
        self.edge_prop_from_fn(fun)
    }
}


// Basic properties traits

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
                $(WithVertexProp<$c> + WithEdgeProp<$c> +)* { }
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
    &'static str, String,
    Color
}

/// Indicates the status of an item in a traverse algorithm.
///
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Color {
    /// Generally indicates that an item was not discovered in the search.
    White,
    /// Generally indicates that an item was discovered in the search but there is some pending
    /// work to be done with the item.
    Gray,
    /// Generally indicates that the item was discovered and all the work related with the item is
    /// finished.
    Black,
}

impl Default for Color {
    /// Returns `Color::White`.
    #[inline]
    fn default() -> Color {
        Color::White
    }
}


// Adaptors

/// A property that maps the value of a wrapped property with a function.
///
/// This `struct` is created by [`PropGet::map`].
///
/// [`PropGet::map`]: trait.PropGet.html#method.map
pub struct Map<P, F>(P, F);

impl<K, P, F, O> PropGet<K> for Map<P, F>
    where P: PropGet<K>,
          F: Fn(P::Output) -> O
{
    type Output = O;

    #[inline]
    fn get(&self, k: K) -> Self::Output {
        (self.1)(self.0.get(k))
    }
}
