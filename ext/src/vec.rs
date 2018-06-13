// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! An extension trait for `std::vec::Vec`.

use rand::prelude::*;

use std::cmp::Ordering;

/// An extension trait for [`std::vec::Vec`].
///
/// Methods with the suffix `ed` are like the original methods, but consumes the vector. This is
/// interesting to chain methods call. For example:
///
/// ```
/// use fera_ext::VecExt;
///
/// assert_eq!(vec![1, 2, 3], vec![4, 3, 1, 3, 4, 2].sorted().deduped().truncated(3));
/// ```
///
/// [`std::vec::Vec`]: https://doc.rust-lang.org/stable/std/vec/struct.Vec.html
// TODO: create a plugin to generated *ed methods
#[allow(missing_docs)]
pub trait VecExt<T> {
    /// Creates a new vector with `len` uninitialized elements.
    ///
    /// # Safety
    ///
    /// This is unsafe because some values may not be dropped or some values may be dropped
    /// without being initialized. See [`std::mem::uninitialized`] for more informations.
    ///
    /// [`std::mem::uninitialized`]: https://doc.rust-lang.org/stable/std/mem/fn.uninitialized.html
    unsafe fn new_uninitialized(len: usize) -> Self;

    fn appended(self, other: &mut Self) -> Self;

    fn deduped(self) -> Self
    where
        T: PartialEq;

    fn deduped_by<F>(self, same_bucket: F) -> Self
    where
        F: FnMut(&mut T, &mut T) -> bool;

    fn deduped_by_key<F, K>(self, key: F) -> Self
    where
        F: FnMut(&mut T) -> K,
        K: PartialEq<K>;

    fn resized(self, new_len: usize, value: T) -> Self
    where
        T: Clone;

    fn reversed(self) -> Self;

    fn shrinked_to_fit(self) -> Self;

    /// Shuffle this vector using `SmallRng::from_entropy()`.
    fn shuffled(self) -> Self;

    /// Shuffle this vector using `rng`.
    fn shuffled_with<R: Rng>(self, rng: R) -> Self;

    fn sorted(self) -> Self
    where
        T: Ord;

    fn sorted_by<F>(self, compare: F) -> Self
    where
        F: FnMut(&T, &T) -> Ordering;

    fn sorted_by_key<F, K>(self, key: F) -> Self
    where
        F: FnMut(&T) -> K,
        K: Ord;

    fn truncated(self, len: usize) -> Self;
}

impl<T> VecExt<T> for Vec<T> {
    unsafe fn new_uninitialized(size: usize) -> Self {
        let mut v = Vec::with_capacity(size);
        v.set_len(size);
        v
    }

    fn deduped(mut self) -> Self
    where
        T: PartialEq,
    {
        self.dedup();
        self
    }

    fn deduped_by<F>(mut self, same_bucket: F) -> Self
    where
        F: FnMut(&mut T, &mut T) -> bool,
    {
        self.dedup_by(same_bucket);
        self
    }

    fn deduped_by_key<F, K>(mut self, key: F) -> Self
    where
        F: FnMut(&mut T) -> K,
        K: PartialEq<K>,
    {
        self.dedup_by_key(key);
        self
    }

    fn appended(mut self, other: &mut Self) -> Self {
        self.append(other);
        self
    }

    fn resized(mut self, new_len: usize, value: T) -> Self
    where
        T: Clone,
    {
        self.resize(new_len, value);
        self
    }

    fn reversed(mut self) -> Self {
        self.reverse();
        self
    }

    fn shrinked_to_fit(mut self) -> Self {
        self.shrink_to_fit();
        self
    }

    fn shuffled(self) -> Self {
        self.shuffled_with(SmallRng::from_entropy())
    }

    fn shuffled_with<R: Rng>(mut self, mut rng: R) -> Self {
        rng.shuffle(&mut self[..]);
        self
    }

    fn sorted(mut self) -> Self
    where
        T: Ord,
    {
        self.sort();
        self
    }

    fn sorted_by<F>(mut self, compare: F) -> Self
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        self.sort_by(compare);
        self
    }

    fn sorted_by_key<F, K>(mut self, key: F) -> Self
    where
        F: FnMut(&T) -> K,
        K: Ord,
    {
        self.sort_by_key(key);
        self
    }

    fn truncated(mut self, len: usize) -> Self {
        self.truncate(len);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deduped() {
        assert_eq!(vec![1, 2, 3, 1], vec![1, 1, 2, 3, 3, 1].deduped());
    }

    #[test]
    fn test_appended() {
        assert_eq!(vec![1, 2, 3, 4, 5], vec![1, 2].appended(&mut vec![3, 4, 5]));
    }

    #[test]
    fn test_resized() {
        assert_eq!(vec![3, 4, 1, 1, 1], vec![3, 4, 1].resized(5, 1));
        assert_eq!(vec![3, 4], vec![3, 4, 1].resized(2, 1));
    }

    #[test]
    fn test_reversed() {
        assert_eq!(vec![3, 2, 1], vec![1, 2, 3].reversed());
    }

    #[test]
    fn test_sorted() {
        assert_eq!(vec![1, 2, 3], vec![3, 1, 2].sorted());
    }

    #[test]
    fn test_sorted_by() {
        assert_eq!(vec![3, 2, 1], vec![1, 2, 3].sorted_by(|a, b| b.cmp(a)));
    }

    #[test]
    fn test_sorted_by_key() {
        assert_eq!(
            vec![1isize, 2, -3],
            vec![-3isize, 2, 1].sorted_by_key(|a| a.abs())
        );
    }

    #[test]
    fn test_shuffled() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let s = v.clone().shuffled();
        assert!(v != s);
        assert_eq!(v, s.sorted());
    }

    #[test]
    fn test_truncated() {
        assert_eq!(vec![1, 2], vec![1, 2, 3, 4].truncated(2));
    }
}
