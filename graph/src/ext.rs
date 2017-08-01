//! Extension traits for std types.
//!
//! The traits is this module are included in the [prelude].
//!
//! # Examples
//!
//! ```
//! extern crate fera_fun;
//! extern crate fera_graph;
//!
//! use fera_graph::prelude::*;
//! use fera_fun::vec;
//!
//! # fn main() {
//! let g = CompleteGraph::new(4);
//! let mut w = g.default_vertex_prop(0);
//! w[0] = 1;
//! w[1] = 2;
//! w[2] = 3;
//! w[3] = 0;
//!
//! assert_eq!(vec![3, 0, 1, 2], vec(g.vertices()).sorted_by_prop(&w));
//! # }
//! ```
//!
//! [prelude]: ../prelude/index.html

use prelude::*;
use params::IntoOwned;

/// Extension trait for slices.
///
/// See the [module documentation] for examples.
///
/// [module documentation]: index.html
pub trait GraphsSliceExt<T> {
    /// Sort a slice using a [property].
    ///
    /// [property]: ../props/index.html
    fn sort_by_prop<P, K>(&mut self, prop: P)
        where P: PropGet<K>,
              for<'a> &'a T: IntoOwned<K>,
              P::Output: Ord;
}

impl<T> GraphsSliceExt<T> for [T] {
    #[inline]
    fn sort_by_prop<P, K>(&mut self, prop: P)
        where P: PropGet<K>,
              for<'a> &'a T: IntoOwned<K>,
              P::Output: Ord
    {
        self.sort_by_key(|v| prop.get(v.into_owned()))
    }
}

/// Extension trait for vectors.
///
/// See the [module documentation] for examples.
///
/// [module documentation]: index.html
pub trait GraphsVecExt<T> {
    /// Returns a vector sorted by a [property].
    ///
    /// [property]: ../props/index.html
    fn sorted_by_prop<P, K>(self, prop: P) -> Self
        where P: PropGet<K>,
              for<'a> &'a T: IntoOwned<K>,
              P::Output: Ord;
}

impl<T> GraphsVecExt<T> for Vec<T> {
    #[inline]
    fn sorted_by_prop<P, K>(mut self, prop: P) -> Self
        where P: PropGet<K>,
              for<'a> &'a T: IntoOwned<K>,
              P::Output: Ord
    {
        self.sort_by_key(|v| prop.get(v.into_owned()));
        self
    }
}
