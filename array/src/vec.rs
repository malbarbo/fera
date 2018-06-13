use {Array, DynamicArray};

use std::ops::{Index, IndexMut};

#[derive(Clone)]
pub struct VecArray<T>(Vec<T>);

impl<T> Index<usize> for VecArray<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &T {
        self.0.index(index)
    }
}

impl<T> IndexMut<usize> for VecArray<T> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut T {
        self.0.index_mut(index)
    }
}

impl<T> Array<T> for VecArray<T> {
    #[inline]
    fn with_value(value: T, n: usize) -> Self
        where Self: Sized,
              T: Clone
    {
        VecArray(vec![value; n])
    }

    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[inline]
    fn get(&self, index: usize) -> Option<&T> {
        self.0.get(index)
    }

    #[inline]
    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.0.get_mut(index)
    }

    #[inline]
    unsafe fn get_unchecked(&self, index: usize) -> &T {
        self.0.get_unchecked(index)
    }

    #[inline]
    unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut T {
        self.0.get_unchecked_mut(index)
    }
}

impl<T> DynamicArray<T> for VecArray<T> {
    #[inline]
    fn with_capacity(capacity: usize) -> Self
        where Self: Sized
    {
        VecArray(Vec::with_capacity(capacity))
    }

    #[inline]
    fn capacity(&self) -> usize {
        self.0.capacity()
    }

    #[inline]
    unsafe fn set_len(&mut self, len: usize) {
        self.0.set_len(len);
    }

    #[inline]
    fn reserve(&mut self, additional: usize) {
        self.0.reserve(additional);
    }

    #[inline]
    fn push(&mut self, value: T) {
        self.0.push(value);
    }

    #[inline]
    fn extend<I>(&mut self, iter: I)
        where I: IntoIterator<Item = T>
    {
        Extend::extend(&mut self.0, iter);
    }

    #[inline]
    fn extend_from_slice(&mut self, other: &[T])
        where T: Clone
    {
        self.0.extend_from_slice(other);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use {ArrayTests, DynamicArrayTests};

    struct T;

    impl ArrayTests for T {
        type A = VecArray<usize>;
    }

    delegate_tests!{
        T,
        basic_0,
        basic_001k,
        basic_100k,
        clone_001k,
        clone_100k
    }

    impl DynamicArrayTests for T {}

    delegate_tests!{
        T,
        capacity,
        push_1k,
        clone_push
    }

    #[cfg(all(feature = "nightly", test))]
    mod benchs {
        use super::T;
        use test::Bencher;
        use ArrayBenchs;

        impl ArrayBenchs for T {}

        delegate_benchs!{
            T,
            fold_xor_0001k,
            fold_xor_0010k,
            fold_xor_0100k,
            fold_xor_1000k,
            clone_change_0001k,
            clone_change_0010k,
            clone_change_0100k,
            clone_change_1000k
        }
    }
}
