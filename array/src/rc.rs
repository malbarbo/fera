use {Array, DynamicArray, PrefixedArray, PrefixedArrayInfo};

use std::cell::Cell;
use std::iter::FromIterator;
use std::ops::{Index, IndexMut};

/// A reference-counted pointer to an array.
///
/// It allow access to the elements of the array with one indirection, allow the array to grow
/// until the specified capacity and has a function `RcArray::make_mut`.
///
/// It is different from `Rc<Vec>` and `Rc<[T]>`. In `Rc<Vec>` it is necessary to follow two
/// indirection to access an element. In `Rc<[T]>` the array cannot grow and the function
/// `Rc::make_mut` is not applicable.
///
/// TODO: write about `mem::size_of`.
///
/// # Examples
///
/// ```
/// use fera_array::{Array, RcArray};
///
/// let a = RcArray::with_value(0, 5);
/// // a and b share the inner data, this is a O(1) operation
/// let mut b = a.clone();
///
/// // the inner data in cloned (copy on write)
/// b[1] = 1;
/// assert_eq!(0, a[1]);
/// assert_eq!(1, b[1]);
///
/// // the inner data is not shared, so it can be modified without being cloned
/// b[2] = 2;
/// ```
pub struct RcArray<T> {
    inner: PrefixedArray<Info, T>,
}

struct Info {
    count: Cell<usize>,
    cap: usize,
    len: usize,
}

impl PrefixedArrayInfo for Info {
    #[inline]
    fn len(&self) -> usize {
        self.len
    }

    #[inline]
    fn capacity(&self) -> usize {
        self.cap
    }
}

impl<T> RcArray<T> {
    fn new(cap: usize) -> Self {
        let info = Info {
            cap: cap,
            len: 0,
            count: Cell::new(1),
        };
        RcArray { inner: PrefixedArray::allocate(info) }
    }

    /// Returns a slice containing the entire array.
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        self.inner.as_slice()
    }

    /// Returns a mutable slice containing the entire array.
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self.inner.as_mut_slice()
    }

    #[inline]
    fn info(&self) -> &Info {
        self.inner.info()
    }

    #[inline]
    fn info_mut(&mut self) -> &mut Info {
        self.inner.info_mut()
    }

    #[inline]
    fn count(&self) -> &Cell<usize> {
        &self.info().count
    }

    #[inline]
    fn inc(&self) {
        let v = self.count().get() + 1;
        self.count().set(v);
    }

    #[inline]
    fn dec(&self) {
        let v = self.count().get() - 1;
        self.count().set(v);
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self.count().get() == 0
    }

    #[inline]
    fn is_unique(&self) -> bool {
        self.count().get() == 1
    }
}

impl<T: Clone> RcArray<T> {
    /// Makes the array mutable. The inner data is cloned if there is more than one reference to
    /// it, otherwise the array is already mutable.
    pub fn make_mut(&mut self) -> &mut [T] {
        if !self.is_unique() {
            *self = Self::from_iter(self.as_slice().iter().cloned());
        }
        self.as_mut_slice()
    }
}

impl<T> Drop for RcArray<T> {
    fn drop(&mut self) {
        self.dec();
        if self.is_zero() {
            unsafe { self.inner.drop_and_deallocate() }
        }
    }
}

impl<T> Clone for RcArray<T> {
    #[inline]
    fn clone(&self) -> Self {
        self.inc();
        RcArray { inner: unsafe { self.inner.clone_shallow() } }
    }
}

impl<T> FromIterator<T> for RcArray<T> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let iter = iter.into_iter();
        let mut a = RcArray::new(iter.size_hint().0);
        for v in iter {
            unsafe {
                let len = a.info().len();
                a.inner.write(len as usize, v);
                a.info_mut().len = len + 1;
            }
            assert!(a.info().len <= a.info().cap);
        }
        a
    }
}

impl<T> Index<usize> for RcArray<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &T {
        self.as_slice().index(index)
    }
}

impl<T: Clone> IndexMut<usize> for RcArray<T> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut T {
        self.make_mut().index_mut(index)
    }
}

impl<T: Clone> Array<T> for RcArray<T> {
    fn with_value(value: T, n: usize) -> Self
    where
        T: Clone,
    {
        let mut a = RcArray::new(n);
        a.extend_with_element(value, n);
        a
    }

    #[inline]
    fn len(&self) -> usize {
        self.info().len
    }

    #[inline]
    fn get(&self, index: usize) -> Option<&T> {
        self.as_slice().get(index)
    }

    #[inline]
    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.make_mut().get_mut(index)
    }

    #[inline]
    unsafe fn get_unchecked(&self, index: usize) -> &T {
        self.as_slice().get_unchecked(index)
    }

    #[inline]
    unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut T {
        self.make_mut().get_unchecked_mut(index)
    }
}

impl<T: Clone> DynamicArray<T> for RcArray<T> {
    fn with_capacity(cap: usize) -> Self {
        Self::new(cap)
    }

    #[inline]
    unsafe fn set_len(&mut self, len: usize) {
        self.info_mut().len = len;
    }

    #[inline]
    fn capacity(&self) -> usize {
        self.info().cap
    }
}


#[cfg(test)]
mod tests {
    use super::Info;
    use {ArrayTests, DynamicArrayTests, PrefixedArray, RcArray};
    use testdrop::TestDrop;

    #[test]
    fn rc_size() {
        use std::mem;
        assert_eq!(
            mem::size_of::<PrefixedArray<Info, u32>>(),
            mem::size_of::<RcArray<u32>>()
        );
    }

    #[test]
    fn drop_() {
        let test = TestDrop::new();
        let (a, item_a) = test.new_item();
        let (b, item_b) = test.new_item();
        let (c, item_c) = test.new_item();
        let v: RcArray<_> = vec![item_a, item_b, item_c].into_iter().collect();

        let w = v.clone();
        assert_eq!(2, w.count().get());
        drop(w);

        test.assert_no_drop(a);
        test.assert_no_drop(b);
        test.assert_no_drop(c);

        drop(v);

        test.assert_drop(a);
        test.assert_drop(b);
        test.assert_drop(c);
    }

    struct T;

    impl ArrayTests for T {
        type A = RcArray<usize>;
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
