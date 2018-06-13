use {Array, DynamicArray};

use std::marker::PhantomData;
use std::ops::{Index, IndexMut};

/// An `Array` implemented as an array of arrays.
///
/// It contains a main `Array` of type `M`. Each element (nested array) of the main array is an
/// `Array` of type `N`. The type of the elements of the nested arrays is `T`.
///
/// This type offers a flatted view of the elements in the nested arrays. Each time an element at a
/// specified index is accessed, the index is decomposed in two indices (using `S: Split`), the
/// first one is used to index the main array and the second one to index the nested array.
///
/// This type is interesting to implemented copy on write arrays. See `CowNestedArray`.
#[derive(Clone)]
pub struct NestedArray<T, S, M, N> {
    len: usize,
    cap: usize,
    data: M,
    split: S,
    _marker: PhantomData<(T, N)>,
}

impl<T, S, M, N> NestedArray<T, S, M, N>
where
    S: Split,
    M: Array<N>,
    N: Array<T>,
{
    #[inline]
    fn max_nested(&self) -> usize {
        self.split.max_nested()
    }

    #[inline]
    fn split(&self, i: usize) -> (usize, usize) {
        self.split.split(i)
    }

    #[cfg(test)]
    fn compute_len(&self) -> usize {
        self.data.iter().map(|d| d.len()).sum()
    }
}

impl<T, S, M, N> Index<usize> for NestedArray<T, S, M, N>
where
    S: Split,
    M: Array<N>,
    N: Array<T> + Clone,
{
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.len);
        unsafe { self.get_unchecked(index) }
    }
}

impl<T, S, M, N> IndexMut<usize> for NestedArray<T, S, M, N>
where
    S: Split,
    M: Array<N>,
    N: Array<T> + Clone,
{
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < self.len);
        unsafe { self.get_unchecked_mut(index) }
    }
}

impl<T, S, M, N> Array<T> for NestedArray<T, S, M, N>
where
    S: Split,
    M: Array<N>,
    N: Array<T> + Clone,
{
    fn with_value(value: T, n: usize) -> Self
    where
        T: Clone,
    {
        let s = S::new(n);
        let max = s.max_nested();
        let (div, rem) = div_rem(n, max);

        let data = {
            if n == 0 {
                M::with_value(N::with_value(value, 0), 0)
            } else if n < s.max_nested() {
                M::with_value(N::with_value(value, n), 1)
            } else if rem == 0 {
                M::with_value(N::with_value(value, max), div)
            } else {
                // FIXME: N is being cloned one extra time
                let mut data = M::with_value(N::with_value(value.clone(), max), div + 1);
                // Fix the last element of data which should have rem elements but have max
                data[div] = N::with_value(value, rem);
                data
            }
        };

        NestedArray {
            len: n,
            cap: n,
            data: data,
            split: s,
            _marker: PhantomData,
        }
    }

    #[inline]
    fn len(&self) -> usize {
        self.len
    }

    #[inline]
    unsafe fn get_unchecked(&self, index: usize) -> &T {
        let (i, j) = self.split(index);
        self.data.get_unchecked(i).get_unchecked(j)
    }

    #[inline]
    unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut T {
        let (i, j) = self.split(index);
        self.data.get_unchecked_mut(i).get_unchecked_mut(j)
    }
}

impl<T, S, M, N> DynamicArray<T> for NestedArray<T, S, M, N>
where
    S: Split,
    M: DynamicArray<N>,
    N: DynamicArray<T> + Clone,
{
    fn with_capacity(capacity: usize) -> Self
    where
        Self: Sized,
    {
        let s = S::new(capacity);
        let max = s.max_nested();
        let (div, rem) = div_rem(capacity, max);

        let mut data = if rem == 0 {
            M::with_capacity(div)
        } else {
            M::with_capacity(div + 1)
        };

        let mut cap = 0;

        while cap + max <= capacity {
            cap += max;
            data.push(N::with_capacity(max));
        }

        if rem != 0 {
            data.push(N::with_capacity(rem));
        }

        NestedArray {
            len: 0,
            cap: capacity,
            data: data,
            split: s,
            _marker: PhantomData,
        }
    }

    // TODO: implement reserve

    fn capacity(&self) -> usize {
        self.cap
    }

    unsafe fn set_len(&mut self, len: usize) {
        if self.len == len {
            return;
        }

        let (i, j) = self.split(len);
        let max = self.max_nested();
        let old = self.data.len();

        if j == 0 {
            self.data.set_len(i);
            if i != 0 {
                self.data[i - 1].set_len(max);
            }
        } else {
            self.data.set_len(i + 1);
            self.data[i].set_len(j);
        }

        let new = self.data.len();

        if new != 0 && old < new - 1 {
            for i in old..new - 1 {
                self.data[i].set_len(max)
            }
        }

        self.len = len;
    }
}

/// Describe how to split the indices for a `NestedArray`.
pub trait Split {
    /// Creates a new `Split` for a `NestedArray` with `n` elements.
    fn new(n: usize) -> Self;

    /// Split `index` in two indices (main array, nested array).
    fn split(&self, index: usize) -> (usize, usize);

    /// Returns the maximum index that can be produced to be used in a nested arrays.
    fn max_nested(&self) -> usize;
}

#[derive(Copy, Clone)]
pub struct SqrtSplit(usize);

impl Split for SqrtSplit {
    fn new(n: usize) -> Self {
        SqrtSplit(::std::cmp::max(1, (n as f64).sqrt() as usize))
    }

    fn split(&self, index: usize) -> (usize, usize) {
        div_rem(index, self.0)
    }

    fn max_nested(&self) -> usize {
        self.0
    }
}

#[derive(Copy, Clone)]
pub struct BalancedShiftSplit(usize);

impl Split for BalancedShiftSplit {
    fn new(n: usize) -> Self {
        let mut s = 1;

        while (1 << s) * (1 << s) < n {
            s += 1;
        }

        BalancedShiftSplit(s)
    }

    fn split(&self, index: usize) -> (usize, usize) {
        (index >> self.0, index & (self.max_nested() - 1))
    }

    fn max_nested(&self) -> usize {
        1 << self.0
    }
}

fn div_rem(a: usize, b: usize) -> (usize, usize) {
    (a / b, a % b)
}

/// Implementations of `Split` using bit shift.
pub mod shift {
    use super::Split;
    macro_rules! def_shift {
        ($name:ident, $num:expr) => {
            #[allow(missing_docs)]
            #[derive(Clone)]
            pub struct $name;

            impl Split for $name {
                #[inline]
                fn new(_n: usize) -> Self {
                    $name
                }

                #[inline]
                fn split(&self, index: usize) -> (usize, usize) {
                    (index >> $num, index & (self.max_nested() - 1))
                }

                #[inline]
                fn max_nested(&self) -> usize {
                    (1 << $num)
                }
            }
        };
    }

    macro_rules! def_shifts {
        ($($name:ident $num:expr)+) => (
            $(def_shift!{$name, $num})+
        )
    }

    def_shifts!{
        Shift1   1 Shift2   2 Shift3   3 Shift4   4 Shift5   5 Shift6   6 Shift7   7
        Shift8   8 Shift9   9 Shift10 10 Shift11 11 Shift12 12 Shift13 13 Shift14 14
        Shift15 15 Shift16 16 Shift17 17 Shift18 18 Shift19 19 Shift20 20 Shift21 21
        Shift22 22 Shift23 23 Shift24 24 Shift25 25 Shift26 26 Shift27 27 Shift28 28
        Shift29 29 Shift30 30 Shift31 31
    }

    #[cfg(target_pointer_width = "64")]
    def_shifts!{
        Shift32 32 Shift33 33 Shift34 34 Shift35 35
        Shift36 36 Shift37 37 Shift38 38 Shift39 39 Shift40 40 Shift41 41 Shift42 42
        Shift43 43 Shift44 44 Shift45 45 Shift46 46 Shift47 47 Shift48 48 Shift49 49
        Shift50 50 Shift51 51 Shift52 52 Shift53 53 Shift54 54 Shift55 55 Shift56 56
        Shift57 57 Shift58 58 Shift59 59 Shift60 60 Shift61 61 Shift62 62 Shift63 63
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use {ArrayTests, BalancedShiftSplit, DynamicArray, DynamicArrayTests, VecArray};

    #[test]
    fn sqrt_split() {
        let s = SqrtSplit::new(10);
        assert_eq!(3, s.max_nested());
        assert_eq!((0, 0), s.split(0));
        assert_eq!((0, 1), s.split(1));
        assert_eq!((0, 2), s.split(2));
        assert_eq!((1, 0), s.split(3));
        assert_eq!((1, 1), s.split(4));
        assert_eq!((2, 0), s.split(6));
        assert_eq!((3, 0), s.split(9));
    }

    #[test]
    fn sqrt_split_zero() {
        let s = SqrtSplit::new(0);
        assert_eq!(1, s.max_nested());
        assert_eq!((0, 0), s.split(0));
        assert_eq!((1024, 0), s.split(1024));
    }

    #[test]
    fn shift_split_zero() {
        let s = BalancedShiftSplit::new(0);
        assert_eq!(2, s.max_nested());
        assert_eq!((0, 0), s.split(0));
        assert_eq!((512, 0), s.split(1024));
    }

    #[test]
    fn shift_split() {
        let s = BalancedShiftSplit::new(10);
        assert_eq!(4, s.max_nested());
        assert_eq!((0, 0), s.split(0));
        assert_eq!((0, 3), s.split(3));
        assert_eq!((1, 0), s.split(4));
        assert_eq!((1, 1), s.split(5));
        assert_eq!((2, 0), s.split(8));
        assert_eq!((2, 1), s.split(9));
    }

    #[test]
    fn set_len() {
        let mut v: NestedArray<
            usize,
            BalancedShiftSplit,
            VecArray<VecArray<usize>>,
            VecArray<usize>,
        > = NestedArray::with_capacity(14);

        assert_eq!(0, v.len());
        assert_eq!(0, v.compute_len());

        for i in 0..15 {
            unsafe {
                v.set_len(i);
            }
            assert_eq!(i, v.len());
            assert_eq!(i, v.compute_len());
        }

        for i in 0..15 {
            unsafe {
                v.set_len(14 - i);
            }
            assert_eq!(14 - i, v.len());
            assert_eq!(14 - i, v.compute_len());
        }

        unsafe {
            v.set_len(13);
        }
        assert_eq!(13, v.len());
        assert_eq!(13, v.compute_len());

        unsafe {
            v.set_len(5);
        }

        assert_eq!(5, v.len());
        assert_eq!(5, v.compute_len());
    }

    struct T;

    impl ArrayTests for T {
        type A = NestedArray<usize, BalancedShiftSplit, VecArray<VecArray<usize>>, VecArray<usize>>;
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
