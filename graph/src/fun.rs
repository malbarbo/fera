//! Free functions.

use prelude::*;
use params::IntoOwned;
use std::iter::Sum;

/// Returns the iterator's item with minimum property value or `None` if the iterator is empty.
///
/// If several items have equally minimum property value, the first item with minimum property
/// value is returned.
///
/// # Example
///
/// ```
/// use fera_graph::prelude::*;
/// use fera_graph::min_by_prop;
///
/// let g = CompleteGraph::new(4);
/// let mut w = g.default_vertex_prop(0u32);
/// w[0] = 5;
/// w[1] = 0;
/// w[2] = 3;
/// w[3] = 0;
/// assert_eq!(Some(1), min_by_prop(&w, g.vertices()));
/// assert_eq!(None, min_by_prop(&w, g.vertices().take(0)));
/// assert_eq!(Some(&2), min_by_prop(&w, &[0, 2]));
/// ```
#[inline]
pub fn min_by_prop<I, P, K>(prop: P, iter: I) -> Option<I::Item>
    where I: IntoIterator,
          I::Item: Copy + IntoOwned<K>,
          P: PropGet<K>,
          P::Output: Ord
{
    iter.into_iter().min_by_key(move |&v| prop.get(v.into_owned()))
}

/// Returns the iterator's item with maximum property value or `None` if the iterator is empty.
///
/// If several items have equally maximum property value, the last item with maximum property value
/// is returned.
///
/// # Example
///
/// ```
/// use fera_graph::prelude::*;
/// use fera_graph::max_by_prop;
///
/// let g = CompleteGraph::new(4);
/// let mut w = g.default_vertex_prop(0u32);
/// w[0] = 5;
/// w[1] = 10;
/// w[2] = 3;
/// w[3] = 10;
/// assert_eq!(Some(3), max_by_prop(&w, g.vertices()));
/// assert_eq!(None, max_by_prop(&w, g.vertices().take(0)));
/// assert_eq!(Some(&0), max_by_prop(&w, &[0, 2]));
/// ```
#[inline]
pub fn max_by_prop<I, P, K>(prop: P, iter: I) -> Option<I::Item>
    where I: IntoIterator,
          I::Item: Copy + IntoOwned<K>,
          P: PropGet<K>,
          P::Output: Ord
{
    iter.into_iter().max_by_key(move |&v| prop.get(v.into_owned()))
}

/// Returns the minimum property value associated with the iterator's items or `None` if the
/// iterator is empty.
///
/// # Example
///
/// ```
/// use fera_graph::prelude::*;
/// use fera_graph::min_prop;
///
/// let g = CompleteGraph::new(3);
/// let mut w = g.default_vertex_prop(0u32);
/// w[0] = 5;
/// w[1] = 10;
/// w[2] = 4;
/// assert_eq!(Some(4), min_prop(&w, g.vertices()));
/// assert_eq!(None, min_prop(&w, g.vertices().take(0)));
/// assert_eq!(Some(5), min_prop(&w, &[1, 0]));
/// ```
#[inline]
pub fn min_prop<P, K, I>(prop: P, iter: I) -> Option<P::Output>
    where I: IntoIterator,
          I::Item: IntoOwned<K>,
          P: PropGet<K>,
          P::Output: Ord
{

    iter.into_iter().map(move |v| prop.get(v.into_owned())).min()
}

/// Returns the maximum property value associated with the iterator's items or `None` if the
/// iterator is empty.
///
/// # Example
///
/// ```
/// use fera_graph::prelude::*;
/// use fera_graph::max_prop;
///
/// let g = CompleteGraph::new(3);
/// let mut w = g.default_vertex_prop(0u32);
/// w[0] = 5;
/// w[1] = 10;
/// w[2] = 4;
/// assert_eq!(Some(10), max_prop(&w, g.vertices()));
/// assert_eq!(None, max_prop(&w, g.vertices().take(0)));
/// assert_eq!(Some(5), max_prop(&w, &[2, 0]));
/// ```
#[inline]
pub fn max_prop<P, K, I>(prop: P, iter: I) -> Option<P::Output>
    where I: IntoIterator,
          I::Item: IntoOwned<K>,
          P: PropGet<K>,
          P::Output: Ord
{
    iter.into_iter().map(move |v| prop.get(v.into_owned())).max()
}

/// Returns the sum of the property values of the iterator's items.
///
/// # Example
///
/// ```
/// use fera_graph::prelude::*;
/// use fera_graph::sum_prop;
///
/// let g = CompleteGraph::new(3);
/// let mut w = g.default_vertex_prop(0u32);
/// w[0] = 5;
/// w[1] = 10;
/// w[2] = 4;
/// assert_eq!(19u32, sum_prop(&w, g.vertices()));
/// assert_eq!(0u32, sum_prop(&w, g.vertices().take(0)));
/// assert_eq!(15u32, sum_prop(&w, &[1, 0]));
/// ```
#[inline]
pub fn sum_prop<P, K, O, I>(prop: P, iter: I) -> O
    where I: IntoIterator,
          I::Item: IntoOwned<K>,
          P: PropGet<K>,
          O: Sum<P::Output>
{
    iter.into_iter().map(move |v| prop.get(v.into_owned())).sum()
}
