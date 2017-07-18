use prelude::*;
use std::ops::{Index, IndexMut};

/// A property that ignore writes.
///
/// This `struct` maintains two values, one that is returned when a read only value is requested
/// and one that is returned when a mutable values is used. This does not _really_ ignore writes,
/// but always return the same mutable reference avoiding keeping distinct copies, that away a
/// client should no depend on values obtained by the `index_mut` method.
pub struct IgnoreWriteProp<T> {
    read: T,
    write: T,
}

impl<I, T> Index<I> for IgnoreWriteProp<T> {
    type Output = T;

    #[inline]
    fn index(&self, _: I) -> &Self::Output {
        &self.read
    }
}

impl<I, T> IndexMut<I> for IgnoreWriteProp<T> {
    #[inline]
    fn index_mut(&mut self, _: I) -> &mut Self::Output {
        &mut self.write
    }
}

impl<G, T> VertexPropMutNew<G, T> for IgnoreWriteProp<T>
    where G: WithVertex
{
    fn new_vertex_prop(_: &G, value: T) -> Self
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
    fn new_edge_prop(_: &G, value: T) -> Self
        where T: Clone
    {
        IgnoreWriteProp {
            read: value.clone(),
            write: value,
        }
    }
}
