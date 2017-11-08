// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use prelude::*;
use std::ops::{Index, IndexMut};

/// A property that ignore writes.
///
/// This `struct` maintains two values, one that is returned when a shared reference is requested
/// and one that is returned when a mutable reference is requested. Before returning the mutable
/// reference, the value is cloned from the original value, so it seems that all previous writes
/// was ignored.
///
/// # Example
///
/// ```
/// use fera_graph::prelude::*;
/// use fera_graph::props::IgnoreWriteProp;
///
/// let g = CompleteGraph::new(5);
/// let mut p: IgnoreWriteProp<u32> = g.vertex_prop(3);
///
/// assert_eq!(3, p[0]);
/// p[0] = 20;
/// // the previous write was "ignored"
/// assert_eq!(3, p[0]);
/// ```
pub struct IgnoreWriteProp<T> {
    read: T,
    write: T,
}

impl<I, T: Clone> Index<I> for IgnoreWriteProp<T> {
    type Output = T;

    #[inline]
    fn index(&self, _: I) -> &Self::Output {
        &self.read
    }
}

impl<I, T: Clone> IndexMut<I> for IgnoreWriteProp<T> {
    #[inline]
    fn index_mut(&mut self, _: I) -> &mut Self::Output {
        self.write.clone_from(&self.read);
        &mut self.write
    }
}

impl<G, T> VertexPropMutNew<G, T> for IgnoreWriteProp<T>
    where G: WithVertex,
          T: Clone
{
    fn new_vertex_prop(_: &G, value: T) -> Self {
        IgnoreWriteProp {
            read: value.clone(),
            write: value,
        }
    }
}

impl<G, T> EdgePropMutNew<G, T> for IgnoreWriteProp<T>
    where G: WithEdge,
          T: Clone,
{
    fn new_edge_prop(_: &G, value: T) -> Self {
        IgnoreWriteProp {
            read: value.clone(),
            write: value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let n = 3;
        let val = 7;
        let g = CompleteGraph::new(n);
        let mut p: IgnoreWriteProp<u32> = g.vertex_prop(val);

        for i in 0..n {
            assert_eq!(val, p[i]);
            p[i] = 20;
            for j in 0.. n {
                assert_eq!(val, p[j]);
            }
        }
    }
}
