// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![doc(html_root_url="https://docs.rs/fera-fun/0.1.0/")]

//! Free functions for fun programming.
//!
//! This crate can be used through [`fera`] crate.
//!
//! [`fera`]: https://docs.rs/fera

use std::borrow::Borrow;
use std::collections::HashSet;
use std::hash::Hash;

/// Returns the first item of an iterator.
///
/// # Panics
///
/// If there is no first item.
///
/// # Examples
///
/// ```
/// use fera_fun::first;
///
/// assert_eq!(&4, first(&[4, 3, 8]));
/// assert_eq!(2, first(2..));
/// ```
pub fn first<I>(iter: I) -> I::Item
    where I: IntoIterator
{
    iter.into_iter().next().unwrap()
}

/// Returns the first position of an item of an iterator or `None` if the iterator does not
/// produces the item.
///
/// ```
/// use fera_fun::position_of;
///
/// assert_eq!(Some(1), position_of(&[0, 3, 3, 0, 5], &3));
/// ```
pub fn position_of<I, T>(iter: I, item: &T) -> Option<usize>
    where I: IntoIterator,
          I::Item: Borrow<T>,
          T: PartialEq
{
    iter.into_iter()
        .position(|x| x.borrow() == item)
}

/// Returns the last position of the maximum element of a non empty iterator or `None` if iterator
/// is empty.
///
/// ```
/// use fera_fun::position_max_by_key;
///
/// assert_eq!(Some(4), position_max_by_key(&[0i32, 3, -5, 0, 5], |x| x.abs()));
/// ```
pub fn position_max_by_key<I, F, X>(iter: I, mut f: F) -> Option<usize>
    where I: IntoIterator,
          X: Ord,
          F: FnMut(&I::Item) -> X
{
    iter.into_iter()
        .enumerate()
        .max_by_key(|x| f(&x.1))
        .map(|x| x.0)
}

/// Returns the first position of the minimum element of a non empty iterator or `None` if iterator
/// is empty.
///
/// ```
/// use fera_fun::position_min_by_key;
///
/// assert_eq!(Some(0), position_min_by_key(&[0i32, 3, -5, 0, 5], |x| x.abs()));
/// ```
pub fn position_min_by_key<I, F, X>(iter: I, mut f: F) -> Option<usize>
    where I: IntoIterator,
          X: Ord,
          F: FnMut(&I::Item) -> X
{
    iter.into_iter()
        .enumerate()
        .min_by_key(|x| f(&x.1))
        .map(|x| x.0)
}

/// Creates a `Vector` from a iterator.
///
/// ```
/// use fera_fun::vec;
///
/// assert_eq!(vec![1, 2, 3], vec(1..4));
/// ```
pub fn vec<I>(iter: I) -> Vec<I::Item>
    where I: IntoIterator
{
    iter.into_iter().collect()
}

/// Creates a `HashSet` from a iterator.
///
/// ```
/// use fera_fun::set;
///
/// assert_eq!(set(&[4, 3, 8, 3]), set(&[3, 4, 8]))
/// ```
pub fn set<I>(iter: I) -> HashSet<I::Item>
    where I: IntoIterator,
          I::Item: Hash + Eq
{
    iter.into_iter().collect()
}
