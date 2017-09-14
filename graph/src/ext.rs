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
//! let mut vertices = vec(g.vertices());
//! assert!(!vertices.is_sorted_by_prop(&w));
//!
//! vertices.sort_by_prop(&w);
//! assert!(vertices.is_sorted_by_prop(&w));
//! assert_eq!(vec![3, 0, 1, 2], vertices);
//!
//! // returns the index of the vertex with w = 2,
//! // that is, the index of vertex 1
//! assert_eq!(Ok(2), vertices.binary_search_by_prop(&2, &w));
//!
//! // returns the index that a vertex with w = 5
//! // could be inserted while maintaining sorted order
//! assert_eq!(Err(4), vertices.binary_search_by_prop(&5, &w));
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
    /// Returns `true` if a slice is sorted according to a [property], otherwise, returns `false`.
    ///
    /// [property]: ../props/index.html
    fn is_sorted_by_prop<P, K>(&self, prop: P) -> bool
    where
        P: PropGet<K>,
        for<'a> &'a T: IntoOwned<K>,
        P::Output: Ord;

    /// Sort a slice using a [property].
    ///
    /// This functions calls [`slice::sort_by_key`].
    ///
    /// [property]: ../props/index.html
    /// [`slice::sort_by_key`]:
    /// https://doc.rust-lang.org/stable/std/primitive.slice.html#method.sort_by_key
    fn sort_by_prop<P, K>(&mut self, prop: P)
    where
        P: PropGet<K>,
        for<'a> &'a T: IntoOwned<K>,
        P::Output: Ord;

    /// Sort a slice using a [property].
    ///
    /// This functions calls [`slice::sort_unstable_by_key`].
    ///
    /// [property]: ../props/index.html
    /// [`slice::sort_unstable_by_key`]:
    /// https://doc.rust-lang.org/stable/std/primitive.slice.html#method.sort_unstable_by_key
    fn sort_unstable_by_prop<P, K>(&mut self, prop: P)
    where
        P: PropGet<K>,
        for<'a> &'a T: IntoOwned<K>,
        P::Output: Ord;

    /// Binary searches a slice that is sorted by a [property].
    ///
    /// This functions calls [`slice::binary_search_by_key`].
    ///
    /// [property]: ../props/index.html
    /// [`slice::binary_search_by_key`]:
    /// https://doc.rust-lang.org/stable/std/primitive.slice.html#method.binary_search_by_key
    fn binary_search_by_prop<P, K>(&self, prop_value: &P::Output, prop: P) -> Result<usize, usize>
    where
        P: PropGet<K>,
        for<'a> &'a T: IntoOwned<K>,
        P::Output: Ord;
}

impl<T> GraphsSliceExt<T> for [T] {
    fn is_sorted_by_prop<P, K>(&self, prop: P) -> bool
    where
        P: PropGet<K>,
        for<'a> &'a T: IntoOwned<K>,
        P::Output: Ord,
    {
        let mut iter = self.iter();
        if let Some(item) = iter.next() {
            let mut p = prop.get(item.into_owned());
            for item in iter {
                let q = prop.get(item.into_owned());
                if p > q {
                    return false;
                }
                p = q;
            }
        }
        true
    }

    #[inline]
    fn sort_by_prop<P, K>(&mut self, prop: P)
    where
        P: PropGet<K>,
        for<'a> &'a T: IntoOwned<K>,
        P::Output: Ord,
    {
        self.sort_by_key(|item| prop.get(item.into_owned()))
    }

    #[inline]
    fn sort_unstable_by_prop<P, K>(&mut self, prop: P)
    where
        P: PropGet<K>,
        for<'a> &'a T: IntoOwned<K>,
        P::Output: Ord,
    {
        self.sort_unstable_by_key(|item| prop.get(item.into_owned()))
    }

    #[inline]
    fn binary_search_by_prop<P, K>(&self, prop_value: &P::Output, prop: P) -> Result<usize, usize>
    where
        P: PropGet<K>,
        for<'a> &'a T: IntoOwned<K>,
        P::Output: Ord,
    {
        self.binary_search_by_key(prop_value, |item| prop.get(item.into_owned()))
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
    /// This functions calls [`GraphsSliceExt::sort_by_prop`].
    ///
    /// [property]: ../props/index.html
    /// [`GraphsSliceExt::sort_by_prop`]: trait.GraphsSliceExt.html#tymethod.sort_by_prop
    fn sorted_by_prop<P, K>(self, prop: P) -> Self
    where
        P: PropGet<K>,
        for<'a> &'a T: IntoOwned<K>,
        P::Output: Ord;

    /// Returns a vector sorted by a [property].
    ///
    /// This functions calls [`GraphsSliceExt::sort_unstable_by_prop`].
    ///
    /// [property]: ../props/index.html
    /// [`GraphsSliceExt::sort_unstable_by_prop`]: trait.GraphsSliceExt.html#tymethod.sort_unstable_by_prop
    fn sorted_unstable_by_prop<P, K>(self, prop: P) -> Self
    where
        P: PropGet<K>,
        for<'a> &'a T: IntoOwned<K>,
        P::Output: Ord;
}

impl<T> GraphsVecExt<T> for Vec<T> {
    #[inline]
    fn sorted_by_prop<P, K>(mut self, prop: P) -> Self
    where
        P: PropGet<K>,
        for<'a> &'a T: IntoOwned<K>,
        P::Output: Ord,
    {
        self.sort_by_prop(prop);
        self
    }

    #[inline]
    fn sorted_unstable_by_prop<P, K>(mut self, prop: P) -> Self
    where
        P: PropGet<K>,
        for<'a> &'a T: IntoOwned<K>,
        P::Output: Ord,
    {
        self.sort_unstable_by_prop(prop);
        self
    }
}
