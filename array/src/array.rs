use std::marker::PhantomData;
use std::ops::{Index, IndexMut};
use std::ptr;

use fera_fun::vec;

// TODO: Should Array be called AbstractArray (https://en.wikipedia.org/wiki/Array_data_type)?
/// A simple trait for array like structs.
pub trait Array<T>: IndexMut<usize, Output = T> {
    /// Creates a new array with `n` repeated `value`.
    fn with_value(value: T, n: usize) -> Self
    where
        Self: Sized,
        T: Clone;

    /// Returns the number of elements in the array.
    fn len(&self) -> usize;

    /// Returns `true` if the array contains `value`, `false` otherwise.
    fn contains(&self, value: &T) -> bool
    where T: PartialEq
    {
        for i in 0..self.len() {
            if unsafe { self.get_unchecked(i) } == value {
                return true
            }
        }
        false
    }

    /// Returns `true` if the length of the array is 0, otherwise `false`.
    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns a reference to the element of the array at `index`, or `None` if the index is out
    /// of bounds.
    fn get(&self, index: usize) -> Option<&T> {
        if index < self.len() {
            Some(unsafe { self.get_unchecked(index) })
        } else {
            None
        }
    }

    /// Returns a mutable reference to the element of the array at `index`, or `None` if the index
    /// is out of bounds.
    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < self.len() {
            Some(unsafe { self.get_unchecked_mut(index) })
        } else {
            None
        }
    }

    /// Returns a reference to the element of the array at `index`, without doing bounds checking.
    unsafe fn get_unchecked(&self, index: usize) -> &T;

    /// Returns a mutable reference to the element of the array at `index`, without doing bounds
    /// checking.
    unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut T;

    // TODO: Allow each implementation to specify the iterator
    /// Returns a iterator over the array.
    fn iter(&self) -> Iter<T, Self>
    where
        Self: Sized,
    {
        Iter {
            cur: 0,
            len: self.len(),
            array: self,
            _marker: PhantomData,
        }
    }
}

/// A simple trait for a dynamic array like struct.
pub trait DynamicArray<T>: Array<T> {
    /// Creates a array with the specified capacity.
    ///
    /// The implementation is free to choose if the array can grow beyond this capacity or not.
    fn with_capacity(capacity: usize) -> Self
    where
        Self: Sized;

    /// Returns the capacity of the array.
    fn capacity(&self) -> usize;

    // TODO: see Vec docs and add some fundamentals methods, like shrink_to_fit, reserve_exact

    /// Change the length of the array.
    unsafe fn set_len(&mut self, len: usize);

    /// Reserve capacity for at least `additional` more elements.
    ///
    /// # Panics
    ///
    /// If the array cannot satisfy the request.
    ///
    /// The default implementation panics if `additional > (self.capacity() - self.len())`.
    #[inline]
    fn reserve(&mut self, additional: usize) {
        if additional > (self.capacity() - self.len()) {
            panic!(
                "cannot grow: cap {}, len {}, additional {}",
                self.capacity(),
                self.len(),
                additional
            )
        }
    }

    /// Write value to the end of the array and increment the length.
    ///
    /// This method is used in the default implementation of `push`, `extend_with_element` and
    /// `extend_from_slice`.
    ///
    /// # Safety
    ///
    /// It's unsafe because the capacity is not checked.
    #[inline]
    unsafe fn push_unchecked(&mut self, value: T) {
        let len = self.len();
        ptr::write(self.get_unchecked_mut(len), value);
        self.set_len(len + 1);
    }

    /// Appends the `value` to the array.
    ///
    /// # Panics
    ///
    /// If the array is full and cannot grow.
    #[inline]
    fn push(&mut self, value: T) {
        self.reserve(1);
        unsafe {
            self.push_unchecked(value);
        }
    }

    // TODO: remove this method, should implement Extend
    /// Appends all elements of `iter` to the array.
    ///
    /// # Panics
    ///
    /// If the array cannot grow to accommodate all elements.
    fn extend<I>(&mut self, iter: I)
    where
        Self: Sized,
        I: IntoIterator<Item = T>,
    {
        let iter = iter.into_iter();
        self.reserve(iter.size_hint().0);
        for value in iter {
            self.push(value);
        }
    }

    /// Appends all elements in a slice to the array.
    ///
    /// # Panics
    ///
    /// If the array cannot grow to accommodate all elements.
    fn extend_from_slice(&mut self, other: &[T])
    where
        T: Clone,
    {
        self.reserve(other.len());

        for i in 0..other.len() {
            unsafe { self.push_unchecked(other.get_unchecked(i).clone()) }
        }
    }

    /// Extend the array by `n` additional clones of `value`.
    ///
    /// # Panics
    ///
    /// If the array cannot grow to accommodate all elements.
    fn extend_with_element(&mut self, value: T, n: usize)
    where
        T: Clone,
    {
        self.reserve(n);

        for _ in 1..n {
            unsafe {
                self.push_unchecked(value.clone());
            }
        }

        if n > 0 {
            // do not clone the last one
            unsafe {
                self.push_unchecked(value);
            }
        }
    }
}


/// An iterator over `Array`.
pub struct Iter<'a, T, A: 'a + Array<T>> {
    cur: usize,
    len: usize,
    array: &'a A,
    _marker: PhantomData<*const T>,
}

impl<'a, T, A> Iterator for Iter<'a, T, A>
where
    A: 'a + Array<T>,
    T: 'a,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur == self.len {
            None
        } else {
            let item = unsafe { self.array.get_unchecked(self.cur) };
            self.cur += 1;
            Some(item)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(self.len()))
    }
}

impl<'a, T, A> ExactSizeIterator for Iter<'a, T, A>
where
    A: 'a + Array<T>,
    T: 'a,
{
    fn len(&self) -> usize {
        self.len - self.cur
    }
}


/// A set of tests for `Array` implementations.
#[doc(hidden)]
pub trait ArrayTests {
    type A: Array<usize>;

    fn basic_0() {
        Self::basic(0);
    }

    fn basic_001k() {
        Self::basic(1024);
    }

    fn basic_100k() {
        Self::basic(100 * 1024);
    }

    fn clone_001k()
    where
        Self::A: Clone,
    {
        Self::clone(1024);
    }

    fn clone_100k()
    where
        Self::A: Clone,
    {
        Self::clone(100 * 1024);
    }

    fn basic(n: usize) {
        let mut exp = vec![0; n];
        let mut actual = Self::A::with_value(0, n);

        Self::assert_all(&exp, &mut actual);

        for i in 0..n {
            actual[i] = i;
            exp[i] = i;
        }
        Self::assert_all(&exp, &mut actual);

        for i in 0..n {
            *actual.get_mut(i).unwrap() = i + 1;
            *exp.index_mut(i) = i + 1;
        }
        Self::assert_all(&exp, &mut actual);

        for i in 0..n {
            unsafe {
                *actual.get_unchecked_mut(i) = i + 2;
            }
            *exp.index_mut(i) = i + 2;
        }
        Self::assert_all(&exp, &mut actual);
    }

    fn clone(n: usize)
    where
        Self::A: Clone,
    {
        let mut exp1 = vec![0; n];
        let mut exp2 = vec![0; n];
        let mut actual1 = Self::A::with_value(0, n);
        let mut actual2 = actual1.clone();

        actual1[0] = 10;
        exp1[0] = 10;
        Self::assert_all(&exp1, &mut actual1);
        Self::assert_all(&exp2, &mut actual2);

        actual2[0] = 20;
        exp2[0] = 20;
        Self::assert_all(&exp1, &mut actual1);
        Self::assert_all(&exp2, &mut actual2);

        actual1[n / 2] = 30;
        exp1[n / 2] = 30;
        actual2[n / 2] = 40;
        exp2[n / 2] = 40;
        Self::assert_all(&exp1, &mut actual1);
        Self::assert_all(&exp2, &mut actual2);
    }

    fn assert_all(exp: &[usize], actual: &mut Self::A) {
        let n = actual.len();
        assert_eq!(exp.len(), actual.len());
        assert_eq!(exp.is_empty(), actual.is_empty());
        assert_eq!(exp, &*vec(actual.iter().cloned()));
        assert_eq!(exp, &*vec((0..n).map(|i| *actual.index(i))));
        assert_eq!(exp, &*vec((0..n).map(|i| *actual.get(i).unwrap())));
        assert_eq!(
            exp,
            &*vec((0..n).map(|i| unsafe { *actual.get_unchecked(i) }))
        );
        assert_eq!(exp, &*vec((0..n).map(|i| *actual.index_mut(i))));
        assert_eq!(exp, &*vec((0..n).map(|i| *actual.get_mut(i).unwrap())));
        assert_eq!(
            exp,
            &*vec((0..n).map(|i| unsafe { *actual.get_unchecked_mut(i) }))
        );

        assert_eq!(None, actual.get(n));
        assert_eq!(None, actual.get_mut(n));
    }
}


/// A set of tests for `Array` implementations.
#[doc(hidden)]
pub trait DynamicArrayTests: ArrayTests
where
    Self::A: DynamicArray<usize>,
{
    fn push_1k() {
        Self::push(1024);
    }

    fn capacity() {
        assert_eq!(0, Self::A::with_capacity(0).len());

        assert!(1000 <= Self::A::with_capacity(1000).capacity());
        assert_eq!(0, Self::A::with_capacity(0).len());
    }

    fn clone_push()
    where
        Self::A: Clone,
    {
        let mut a = Self::A::with_capacity(10);
        let b = a.clone();
        a.push(100);
        assert_eq!(1, a.len());
        assert_eq!(0, b.len());
    }

    fn push(n: usize) {
        let mut actual = Self::A::with_capacity(n);
        let mut exp = Vec::with_capacity(n);

        for i in 0..n {
            Self::assert_all(&exp, &mut actual);
            actual.push(i);
            exp.push(i);
        }

        Self::assert_all(&*exp, &mut actual)
    }
}


#[cfg(all(feature = "nightly", test))]
use test::Bencher;

/// A set of benchmarks for `Array` implementations.
#[cfg(all(feature = "nightly", test))]
pub trait ArrayBenchs: ArrayTests
where
    Self::A: Clone,
{
    fn fold_xor_0001k(b: &mut Bencher) {
        Self::fold_xor(b, 1024)
    }

    fn fold_xor_0010k(b: &mut Bencher) {
        Self::fold_xor(b, 10 * 1024)
    }

    fn fold_xor_0100k(b: &mut Bencher) {
        Self::fold_xor(b, 100 * 1024)
    }

    fn fold_xor_1000k(b: &mut Bencher) {
        Self::fold_xor(b, 1000 * 1024)
    }

    fn fold_xor(b: &mut Bencher, n: usize) {
        let mut v = Self::A::with_value(0, n);
        for i in 0..n {
            v[i] = i;
        }
        b.iter(|| v.iter().fold(0, |acc, e| acc ^ e))
    }

    fn clone_change_0001k(b: &mut Bencher) {
        Self::clone_change(b, 1024, 1);
    }

    fn clone_change_0010k(b: &mut Bencher) {
        Self::clone_change(b, 10 * 1024, 1);
    }

    fn clone_change_0100k(b: &mut Bencher) {
        Self::clone_change(b, 100 * 1024, 1);
    }

    fn clone_change_1000k(b: &mut Bencher) {
        Self::clone_change(b, 1000 * 1024, 1);
    }

    fn clone_change(b: &mut Bencher, n: usize, ins: usize) {
        fn setup<A: Array<usize>>(n: usize, ins: usize) -> (Vec<A>, Vec<usize>, Vec<usize>) {
            use rand::{Rng, StdRng, SeedableRng};
            let mut rng = StdRng::from_seed(&[0]);
            let mut arrays = Vec::with_capacity(ins + 1);
            arrays.push(A::with_value(0, n));
            let ii = vec((0..ins).map(|e| rng.gen_range(0, e + 1)));
            let jj = vec((0..ins).map(|_| rng.gen_range(0, n)));
            (arrays, ii, jj)
        }

        let (mut arrays, ii, jj) = setup::<Self::A>(n, ins);
        b.iter(|| for (&i, &j) in ii.iter().zip(jj.iter()) {
            let mut new = arrays[i].clone();
            new[j] = j + i;
            arrays.push(new);
        });
    }
}
