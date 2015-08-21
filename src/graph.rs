use iter::{IteratorExt, Map1};
use std::ops::IndexMut;


// Basic

pub trait Types {
    type Vertex: Copy + Eq;
    type Edge: Copy + Eq;
}

pub trait Basic<'a>: Types {
    type VertexIter: Iterator<Item=Self::Vertex>;
    type EdgeIter: Iterator<Item=Self::Edge>;

    fn num_vertices(&self) -> usize;
    fn vertices(&'a self) -> Self::VertexIter;

    fn num_edges(&self) -> usize;
    fn edges(&'a self) -> Self::EdgeIter;

    fn source(&self, e: Self::Edge) -> Self::Vertex;
    fn target(&self, e: Self::Edge) -> Self::Vertex;

    fn endvertices(&self, e: Self::Edge) -> (Self::Vertex, Self::Vertex) {
        (self.source(e), self.target(e))
    }
}


// Degree

pub trait Degree<'a>: Basic<'a> {
    fn degree(&self, v: Self::Vertex) -> usize;
}


// Inc

pub type IncIter<'a, G> = <G as Inc<'a>>::Type;

pub trait Inc<'a>: Basic<'a> {
    type Type: Iterator<Item=Self::Edge>;
    fn inc_edges(&'a self, v: Self::Vertex) -> IncIter<Self>;
}


// Adj

pub type AdjIter<'a, G> = <G as Adj<'a>>::Type;

pub trait Adj<'a>: Basic<'a> {
    type Type: Iterator<Item=Self::Vertex>;
    fn neighbors(&'a self, v: Self::Vertex) -> AdjIter<Self>;
}

impl<'a, G> Adj<'a> for G
    where G: Inc<'a>
{
    type Type = Map1<'a, IncIter<'a, G>, G, fn(&G, G::Edge) -> G::Vertex>;
    fn neighbors(&'a self, v: Self::Vertex) -> AdjIter<Self> {
        self.inc_edges(v).map1(self, Self::target)
    }
}


// Vertex Property

pub trait VertexPropType<'a, T>: Basic<'a> {
    type Type: IndexMut<Self::Vertex, Output=T>;
}

// FIXME: change definition when [E0122] is resolved
// pub type VertexProp<'a, G: VertexPropType<'a, T>, T> = <G as VertexPropType<'a, T>>::Type;
pub type VertexProp<'a, G, T> = <G as VertexPropType<'a, T>>::Type;


// Edge Property

pub trait EdgePropType<'a, T>: Basic<'a> {
    type Type: IndexMut<Self::Edge, Output=T>;
}

// FIXME: change definition when [E0122] is resolved
// pub type EdgeProp<'a, G: EdgePropType<'a, T>, T> = <G as EdgePropType<'a, T>>::Type;
pub type EdgeProp<'a, G, T> = <G as EdgePropType<'a, T>>::Type;


// WithVertexProp and WithEdgeProp

macro_rules! with_prop {
    ($t:ty, $($ty:ty),*) => (
        pub trait WithVertexProp<'a>: VertexPropType<'a, $t> +
                     VertexPropType<'a, Option<$t>>
                     $(+ VertexPropType<'a, $ty>)*
                     $(+ VertexPropType<'a, Vec<$ty>>)*
                     $(+ VertexPropType<'a, Option<$ty>>)*
        {
            fn vertex_prop<T: Clone>(&'a self, value: T) -> VertexProp<Self, T>;
        }

        pub trait WithEdgeProp<'a>: EdgePropType<'a, $t> +
                     EdgePropType<'a, Option<$t>>
                     $(+ EdgePropType<'a, $ty>)*
                     $(+ EdgePropType<'a, Vec<$ty>>)*
                     $(+ EdgePropType<'a, Option<$ty>>)*
        {
            fn edge_prop<T: Clone>(&'a self, value: T) -> EdgeProp<Self, T>;
        }
    )
}

with_prop! {
    bool,
    char,
    i8, i16, i32, i64, isize,
    u8, u16, u32, u64, usize,
    &'a str, String,
    <Self as Types>::Vertex,
    <Self as Types>::Edge
}


// Graph alias

trait_alias!(GraphInc: Basic<'a> + Degree<'a> + Inc<'a>);
trait_alias!(GraphIncWithProps: GraphInc<'a> + WithVertexProp<'a> + WithEdgeProp<'a>);

trait_alias!(GraphAdj: Basic<'a> + Degree<'a> + Adj<'a>);
trait_alias!(GraphAdjWithProps: GraphAdj<'a> + WithVertexProp<'a> + WithEdgeProp<'a>);
