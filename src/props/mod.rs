use prelude::*;
use std::ops::{Index, IndexMut};

mod array;
mod delegate;
#[path="fn.rs"]
mod fn_;
mod hashmap;

pub use self::array::*;
pub use self::delegate::*;
pub use self::fn_::*;
pub use self::hashmap::*;

use extensions::IntoOwned;

pub trait PropGet<K> {
    type Output: Sized;

    fn get(&self, key: K) -> Self::Output;

    #[inline]
    fn map<F, O>(self, fun: F) -> Map<Self, F>
        where Self: Sized,
              F: Fn(Self::Output) -> O
    {
        Map(self, fun)
    }

    #[inline]
    fn by_ref(&self) -> &Self {
        self
    }
}

impl<'a, K, P: PropGet<K>> PropGet<K> for &'a P {
    type Output = P::Output;

    fn get(&self, key: K) -> Self::Output {
        P::get(self, key)
    }
}


// Indexable props

pub trait PropIndexMut<Idx>: IndexMut<Idx> {
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

pub trait VertexPropMut<G, T>: PropIndexMut<Vertex<G>, Output = T>
    where G: WithVertex
{
    // TODO: Write test
    // FIXME: What happen if the graph changes?
    fn reset(&mut self, g: &G, value: T)
        where G: VertexList,
              T: Clone
    {
        for v in g.vertices() {
            self[v].clone_from(&value);
        }
    }
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

    fn vertex_prop_from_fn<P, F>(&self, mut fun: F) -> P
        where Self: VertexList,
              P: VertexPropMutNew<Self, T>,
              F: FnMut(Vertex<Self>) -> T,
              T: Default + Clone
    {
        // FIXME: Can we remove T: Default + Clone?
        let mut p: P = self.vertex_prop(T::default());
        for v in self.vertices() {
            p[v] = fun(v);
        }
        p
    }

    fn default_vertex_prop(&self, value: T) -> DefaultVertexPropMut<Self, T>
        where T: Clone
    {
        self.vertex_prop(value)
    }

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

pub trait EdgePropMut<G, T>: PropIndexMut<Edge<G>, Output = T>
    where G: WithEdge
{
    fn reset(&mut self, g: &G, value: T)
        where G: EdgeList,
              T: Clone
    {
        for e in g.edges() {
            self[e].clone_from(&value);
        }
    }
}

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

    fn edge_prop_from_fn<P, F>(&self, mut fun: F) -> P
        where Self: EdgeList,
              P: EdgePropMutNew<Self, T>,
              F: FnMut(Edge<Self>) -> T,
              T: Default + Clone
    {
        let mut p: P = self.edge_prop(T::default());
        for e in self.edges() {
            p[e] = fun(e);
        }
        p
    }

    fn default_edge_prop(&self, value: T) -> DefaultEdgePropMut<Self, T>
        where T: Clone
    {
        self.edge_prop(value)
    }

    fn default_edge_prop_from_fn<P, F>(&self, fun: F) -> P
        where Self: EdgeList,
              P: EdgePropMutNew<Self, T>,
              F: FnMut(Edge<Self>) -> T,
              T: Default + Clone
    {
        self.edge_prop_from_fn(fun)
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
    String,
    Color
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Color {
    White,
    Gray,
    Black,
}

impl Default for Color {
    fn default() -> Color {
        Color::White
    }
}


// Adaptors

pub struct Map<P, F>(pub P, pub F);

impl<K, P, F, O> PropGet<K> for Map<P, F>
    where P: PropGet<K>,
          F: Fn(P::Output) -> O
{
    type Output = O;

    fn get(&self, k: K) -> Self::Output {
        (self.1)(self.0.get(k))
    }
}
