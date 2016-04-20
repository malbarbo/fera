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

pub type VecPropVertex<G, T> = VecProp<VertexIndex<G>, Vec<T>>;
pub type VecPropEdge<G, T> = VecProp<EdgeIndex<G>, Vec<T>>;

impl<G: Indices, T: Clone> PropMutVertexNew<G, T> for VecPropVertex<G, T> {
    fn new_prop_vertex(g: &G, value: T) -> Self {
        VecPropVertex::<G, T> {
            to_index: g.prop_vertex_index(),
            data: vec![value; g.num_vertices()],
        }
    }
}

impl<G: Indices, T: Clone> PropMutEdgeNew<G, T> for VecPropEdge<G, T> {
    fn new_prop_edge(g: &G, value: T) -> Self {
        VecPropEdge::<G, T> {
            to_index: g.prop_edge_index(),
            data: vec![value; g.num_edges()],
        }
    }
}


// PVec

macro_rules! def_pvec_prop_vertex {
    ($alias:ident, $vec:ident) => (
        pub type $alias<G, T> = VecProp<VertexIndex<G>, $vec<T>>;

        impl<G: Indices, T: 'static + Clone> PropMutVertexNew<G, T> for VecProp<VertexIndex<G>, $vec<T>> {
            fn new_prop_vertex(g: &G, value: T) -> Self {
                VecProp {
                    to_index: g.prop_vertex_index(),
                    data: $vec::with_value(value, g.num_vertices()),
                }
            }
        }
    )
}

def_pvec_prop_vertex!(PVecPropVertex, PVec);
def_pvec_prop_vertex!(PVec0PropVertex, PVec0);
def_pvec_prop_vertex!(PVec1PropVertex, PVec1);
def_pvec_prop_vertex!(PVec2PropVertex, PVec2);
def_pvec_prop_vertex!(PVec3PropVertex, PVec3);
def_pvec_prop_vertex!(PVec4PropVertex, PVec4);

macro_rules! def_pvec_prop_edge {
    ($alias:ident, $vec:ident) => (
        pub type $alias<G, T> = VecProp<EdgeIndex<G>, $vec<T>>;

        impl<G: Indices, T: 'static + Clone> PropMutEdgeNew<G, T> for VecProp<EdgeIndex<G>, $vec<T>> {
            fn new_prop_edge(g: &G, value: T) -> Self {
                VecProp {
                    to_index: g.prop_edge_index(),
                    data: $vec::with_value(value, g.num_edges()),
                }
            }
        }
    )
}

def_pvec_prop_edge!(PVecPropEdge, PVec);
def_pvec_prop_edge!(PVec0PropEdge, PVec0);
def_pvec_prop_edge!(PVec1PropEdge, PVec1);
def_pvec_prop_edge!(PVec2PropEdge, PVec2);
def_pvec_prop_edge!(PVec3PropEdge, PVec3);
def_pvec_prop_edge!(PVec4PropEdge, PVec4);
