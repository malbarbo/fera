use std::collections::HashSet;
use std::hash::Hash;
use std::iter::{Cloned, Enumerate};

mod cmp;
pub use cmp::*;

pub fn cloned<'a, I, T>(iter: I) -> Cloned<I::IntoIter>
    where I: IntoIterator<Item = &'a T>,
          T: 'a + Clone
{
    iter.into_iter().cloned()
}

pub fn enumerate<I>(iter: I) -> Enumerate<I::IntoIter>
    where I: IntoIterator,
{
    iter.into_iter().enumerate()
}

/// Returns the first item from an iterator.
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

/// Creates a Vector from a iterator.
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

/// Creates a HashSet from a iterator.
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
