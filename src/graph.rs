use iter::{IteratorExt, Map1};
use std::ops::IndexMut;


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
    type Type: IndexMut<Self::Vertex, Output=T>;
}

// FIXME: change definition when [E0122] is resolved
// pub type VertexProp<'a, G: VertexPropType<'a, T>, T> = <G as VertexPropType<'a, T>>::Type;
pub type VertexProp<'a, G, T> = <G as VertexPropType<'a, T>>::Type;


// Edge Property

pub trait EdgePropType<'a, T>: Basic {
    type Type: IndexMut<Self::Edge, Output=T>;
}

// FIXME: change definition when [E0122] is resolved
// pub type EdgeProp<'a, G: EdgePropType<'a, T>, T> = <G as EdgePropType<'a, T>>::Type;
pub type EdgeProp<'a, G, T> = <G as EdgePropType<'a, T>>::Type;


// WithVertexProp and WithEdgeProp

macro_rules! with_prop {
    ($t:ty, $($ty:ty),*) => (
        pub trait WithVertexProp: for<'a> VertexPropType<'a, $t> +
                     for<'a> VertexPropType<'a, Option<$t>>
                     $(+ for<'a> VertexPropType<'a, $ty>)*
                     $(+ for<'a> VertexPropType<'a, Option<$ty>>)*
        {
            fn vertex_prop<T: Clone>(&self, value: T) -> VertexProp<Self, T>;
        }

        pub trait WithEdgeProp: for<'a> EdgePropType<'a, $t> +
                     for<'a> EdgePropType<'a, Option<$t>>
                     $(+ for<'a> EdgePropType<'a, $ty>)*
                     $(+ for<'a> EdgePropType<'a, Option<$ty>>)*
        {
            fn edge_prop<T: Clone>(&self, value: T) -> EdgeProp<Self, T>;
        }
    )
}

with_prop! {
    bool,
    char,
    i8, i16, i32, i64, isize,
    u8, u16, u32, u64, usize,
    &'a str, String,
    <Self as Basic>::Vertex,
    <Self as Basic>::Edge
}


// Graph alias

trait_alias!(GraphInc: Basic + Degree + Inc);
trait_alias!(GraphIncWithProps: GraphInc + WithVertexProp + WithEdgeProp);

trait_alias!(GraphAdj: Basic + Degree + Adj);
trait_alias!(GraphAdjWithProps: GraphAdj + WithVertexProp + WithEdgeProp);
