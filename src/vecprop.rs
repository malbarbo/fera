use graph::*;

use fera::pvec::*;
use fera::array::Array;

use std::ops::{Deref, Index, IndexMut};


// TODO: Define a feature to disable bounds check.

#[derive(Clone)]
pub struct VecProp<I, D> {
    to_index: I,
    data: D,
}

impl<I, D> Deref for VecProp<I, D> {
    type Target = D;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<I: ToIndex<K>, K, D: Index<usize>> Index<K> for VecProp<I, D> {
    type Output = D::Output;

    #[inline(always)]
    fn index(&self, key: K) -> &Self::Output {
        self.data.index(self.to_index.to_index(key))
    }
}

impl<I: ToIndex<K>, K, D: IndexMut<usize>> IndexMut<K> for VecProp<I, D> {
    #[inline(always)]
    fn index_mut(&mut self, key: K) -> &mut Self::Output {
        self.data.index_mut(self.to_index.to_index(key))
    }
}


// Vec

pub type VecVertexProp<G, T> = VecProp<VertexIndexProp<G>, Vec<T>>;
pub type VecEdgeProp<G, T> = VecProp<EdgeIndexProp<G>, Vec<T>>;

impl<G: VertexIndex + VertexList, T> VertexPropMutNew<G, T> for VecVertexProp<G, T> {
    fn new_vertex_prop(g: &G, value: T) -> Self where T: Clone {
        VecVertexProp::<G, T> {
            to_index: g.vertex_index(),
            data: vec![value; g.num_vertices()],
        }
    }
}

impl<G: EdgeIndex + EdgeList, T> EdgePropMutNew<G, T> for VecEdgeProp<G, T> {
    fn new_edge_prop(g: &G, value: T) -> Self where T: Clone {
        VecEdgeProp::<G, T> {
            to_index: g.edge_index(),
            data: vec![value; g.num_edges()],
        }
    }
}


// PVec

macro_rules! def_pvec_vertex_prop {
    ($alias:ident, $vec:ident) => (
        pub type $alias<G, T> = VecProp<VertexIndexProp<G>, $vec<T>>;

        impl<G, T> VertexPropMutNew<G, T> for VecProp<VertexIndexProp<G>, $vec<T>>
            where G: VertexIndex + VertexList,
                  T: 'static + Clone
        {

            fn new_vertex_prop(g: &G, value: T) -> Self {
                VecProp {
                    to_index: g.vertex_index(),
                    data: $vec::with_value(value, g.num_vertices()),
                }
            }
        }
    )
}

def_pvec_vertex_prop!(PVecVertexProp, PVec);
def_pvec_vertex_prop!(PVec0VertexProp, PVec0);
def_pvec_vertex_prop!(PVec1VertexProp, PVec1);
def_pvec_vertex_prop!(PVec2VertexProp, PVec2);
def_pvec_vertex_prop!(PVec3VertexProp, PVec3);
def_pvec_vertex_prop!(PVec4VertexProp, PVec4);

macro_rules! def_pvec_edge_prop {
    ($alias:ident, $vec:ident) => (
        pub type $alias<G, T> = VecProp<EdgeIndexProp<G>, $vec<T>>;

        impl<G, T: 'static> EdgePropMutNew<G, T> for VecProp<EdgeIndexProp<G>, $vec<T>>
            where G: EdgeIndex + EdgeList,
                  T: 'static + Clone
        {
            fn new_edge_prop(g: &G, value: T) -> Self {
                VecProp {
                    to_index: g.edge_index(),
                    data: $vec::with_value(value, g.num_edges()),
                }
            }
        }
    )
}

def_pvec_edge_prop!(PVecEdgeProp, PVec);
def_pvec_edge_prop!(PVec0EdgeProp, PVec0);
def_pvec_edge_prop!(PVec1EdgeProp, PVec1);
def_pvec_edge_prop!(PVec2EdgeProp, PVec2);
def_pvec_edge_prop!(PVec3EdgeProp, PVec3);
def_pvec_edge_prop!(PVec4EdgeProp, PVec4);
