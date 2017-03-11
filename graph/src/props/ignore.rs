use prelude::*;
use std::ops::{Index, IndexMut};

pub struct IgnoreWriteProp<T> {
    read: T,
    write: T,
}

impl<I, T> Index<I> for IgnoreWriteProp<T> {
    type Output = T;

    fn index(&self, _: I) -> &Self::Output {
        &self.read
    }
}

impl<I, T> IndexMut<I> for IgnoreWriteProp<T> {
    fn index_mut(&mut self, _: I) -> &mut Self::Output {
        &mut self.write
    }
}

impl<G, T> VertexPropMutNew<G, T> for IgnoreWriteProp<T>
    where G: WithVertex
{
    fn new_vertex_prop(_g: &G, value: T) -> Self
        where T: Clone
    {
        IgnoreWriteProp {
            read: value.clone(),
            write: value,
        }
    }
}

impl<G, T> EdgePropMutNew<G, T> for IgnoreWriteProp<T>
    where G: WithEdge
{
    fn new_edge_prop(_g: &G, value: T) -> Self
        where T: Clone
    {
        IgnoreWriteProp {
            read: value.clone(),
            write: value,
        }
    }
}
