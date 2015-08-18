#[cfg(test)]
#[macro_use]
pub mod tests;
pub mod static_;
pub mod iter;
pub mod traverse;
pub mod unionfind;
pub mod kruskal;

pub use static_::StaticGraph;
pub use static_::StaticGraphBuilder;

use iter::{IteratorExt, Map1};

// Basic

pub trait Basic {
    type Vertex: Copy + Eq;
    type Edge: Copy + Eq;
    type VertexIter: Iterator<Item=Self::Vertex>;
    type EdgeIter: Iterator<Item=Self::Edge>;

    fn num_vertices(&self) -> usize;
    fn vertices(&self) -> Self::VertexIter;

    fn num_edges(&self) -> usize;
    fn edges(&self) -> Self::EdgeIter;

    fn source(&self, e: Self::Edge) -> Self::Vertex;
    fn target(&self, e: Self::Edge) -> Self::Vertex;

    fn endvertices(&self, e: Self::Edge) -> (Self::Vertex, Self::Vertex) {
        (self.source(e), self.target(e))
    }
}


// Degree

pub trait Degree: Basic {
    fn degree(&self, v: Self::Vertex) -> usize;
}


// Inc

pub trait IncIterType<'a>: Basic {
    type Type: Iterator<Item=Self::Edge>;
}

// FIXME: change definition when [E0122] is resolved
// pub type IncIter<'a, G: Inc> = <G as IncIterType<'a>>::Type;
pub type IncIter<'a, G> = <G as IncIterType<'a>>::Type;

pub trait Inc: Basic + for<'a> IncIterType<'a> {
    fn inc_edges(&self, v: Self::Vertex) -> IncIter<Self>;
}


// Adj

pub trait AdjIterType<'a>: Basic {
    type Type: Iterator<Item=Self::Vertex>;
}

// FIXME: change definition when [E0122] is resolved
// pub type AdjIter<'a, G: Adj> = <G as AdjIterType<'a>>::Type;
pub type AdjIter<'a, G> = <G as AdjIterType<'a>>::Type;

pub trait Adj: Basic + for<'a> AdjIterType<'a> {
    fn neighbors(&self, v: Self::Vertex) -> AdjIter<Self>;
}

// Implementation of Adj traits for Graphs which implements Inc
impl<'a, G: Inc> AdjIterType<'a> for G {
    type Type = Map1<'a, IncIter<'a, G>, G, fn(&G, G::Edge) -> G::Vertex>;
}

impl<G: Inc> Adj for G {
    fn neighbors(&self, v: Self::Vertex) -> AdjIter<Self> {
        self.inc_edges(v).map1(self, Self::target)
    }
}


// Vertex Property

pub trait VertexPropType<'a, T>: Basic {
    type Type: std::ops::IndexMut<Self::Vertex, Output=T>;
}

// FIXME: change definition when [E0122] is resolved
// pub type VertexProp<'a, G: VertexPropType<'a, T>, T> = <G as VertexPropType<'a, T>>::Type;
pub type VertexProp<'a, G, T> = <G as VertexPropType<'a, T>>::Type;

pub trait WithVertexProp:
        for<'a> VertexPropType<'a, bool> +
        for<'a> VertexPropType<'a, char> +
        for<'a> VertexPropType<'a, i8> +
        for<'a> VertexPropType<'a, i16> +
        for<'a> VertexPropType<'a, i32> +
        for<'a> VertexPropType<'a, i64> +
        for<'a> VertexPropType<'a, isize> +
        for<'a> VertexPropType<'a, u8> +
        for<'a> VertexPropType<'a, u16> +
        for<'a> VertexPropType<'a, u32> +
        for<'a> VertexPropType<'a, u64> +
        for<'a> VertexPropType<'a, usize> +
        for<'a> VertexPropType<'a, f32> +
        for<'a> VertexPropType<'a, f64> +
        for<'a> VertexPropType<'a, &'a str> +
        for<'a> VertexPropType<'a, String> +
        for<'a> VertexPropType<'a, <Self as Basic>::Vertex> {
    fn vertex_prop<T: Clone>(&self, value: T) -> VertexProp<Self, T>;
}


// Edge Property

pub trait EdgePropType<'a, T>: Basic {
    type Type: std::ops::IndexMut<Self::Edge, Output=T>;
}

// FIXME: change definition when [E0122] is resolved
// pub type EdgeProp<'a, G: EdgePropType<'a, T>, T> = <G as EdgePropType<'a, T>>::Type;
pub type EdgeProp<'a, G, T> = <G as EdgePropType<'a, T>>::Type;

pub trait WithEdgeProp:
        for<'a> EdgePropType<'a, bool> +
        for<'a> EdgePropType<'a, char> +
        for<'a> EdgePropType<'a, i8> +
        for<'a> EdgePropType<'a, i16> +
        for<'a> EdgePropType<'a, i32> +
        for<'a> EdgePropType<'a, i64> +
        for<'a> EdgePropType<'a, isize> +
        for<'a> EdgePropType<'a, u8> +
        for<'a> EdgePropType<'a, u16> +
        for<'a> EdgePropType<'a, u32> +
        for<'a> EdgePropType<'a, u64> +
        for<'a> EdgePropType<'a, usize> +
        for<'a> EdgePropType<'a, f32> +
        for<'a> EdgePropType<'a, f64> +
        for<'a> EdgePropType<'a, &'a str> +
        for<'a> EdgePropType<'a, String> +
        for<'a> EdgePropType<'a, <Self as Basic>::Edge> {
    fn edge_prop<T: Clone>(&self, value: T) -> EdgeProp<Self, T>;
}


// GraphInc

pub trait GraphInc: Basic + Degree + Inc {
}

impl<G> GraphInc for G where G: Basic + Degree + Inc {
}


// GraphAdj

pub trait GraphAdj: Basic + Degree + Adj {
}

impl<G> GraphAdj for G where G: Basic + Degree + Adj {
}
