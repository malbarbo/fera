//! An extension trait for `std::vec::Vec`.

use rand::Rng;

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
    /// This is unsafe because some values maybe not be dropped or some values maybe dropped
    /// without being initialized. See [`std::mem::uninitialized`] for more informations.
    ///
    /// [`std::mem::uninitialized`]: https://doc.rust-lang.org/stable/std/mem/fn.uninitialized.html
    unsafe fn new_uninitialized(len: usize) -> Self;

    fn appended(self, other: &mut Self) -> Self;

    fn deduped(self) -> Self where T: PartialEq;

    fn resized(self, new_len: usize, value: T) -> Self where T: Clone;

    fn reversed(self) -> Self;

    fn shrinked_to_fit(self) -> Self;

    fn shuffled<R: Rng>(self, rng: R) -> Self;

    fn sorted(self) -> Self where T: Ord;

    fn sorted_by<F>(self, compare: F) -> Self where F: FnMut(&T, &T) -> Ordering;

    fn sorted_by_key<F, K>(self, key: F) -> Self
        where F: FnMut(&T) -> K,
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
        where T: PartialEq
    {
        self.dedup();
        self
    }

    fn appended(mut self, other: &mut Self) -> Self {
        self.append(other);
        self
    }

    fn resized(mut self, new_len: usize, value: T) -> Self
        where T: Clone
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

    fn shuffled<R: Rng>(mut self, mut rng: R) -> Self {
        rng.shuffle(&mut self[..]);
        self
    }

    fn sorted(mut self) -> Self
        where T: Ord
    {
        self.sort();
        self
    }

    fn sorted_by<F>(mut self, compare: F) -> Self
        where F: FnMut(&T, &T) -> Ordering
    {
        self.sort_by(compare);
        self
    }

    fn sorted_by_key<F, K>(mut self, key: F) -> Self
        where F: FnMut(&T) -> K,
              K: Ord
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
    use rand::XorShiftRng;

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
        assert_eq!(vec![1isize, 2, -3],
                   vec![-3isize, 2, 1].sorted_by_key(|a| a.abs()));
    }

    #[test]
    fn test_shuffled() {
        let mut rng = XorShiftRng::new_unseeded();
        let v = vec![1, 2, 3, 4, 5];
        let s = v.clone().shuffled(&mut rng);
        assert!(v != s);
        assert_eq!(v, s.sorted());
    }

    #[test]
    fn test_truncated() {
        assert_eq!(vec![1, 2], vec![1, 2, 3, 4].truncated(2));
    }
}
