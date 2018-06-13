use std::mem;
use std::ptr;
use std::slice;

/// This is used as starting point to implement other types. See for example the code for `RcArray`.
pub struct PrefixedArray<I: PrefixedArrayInfo, T> {
    inner: *mut Inner<I, T>,
}

pub trait PrefixedArrayInfo {
    fn len(&self) -> usize;

    fn capacity(&self) -> usize;
}

#[repr(C)]
struct Inner<I, T> {
    info: I,
    data: [T; 0],
}

impl<I: PrefixedArrayInfo, T> PrefixedArray<I, T> {
    pub fn allocate(info: I) -> Self {
        assert_ne!(0, ::std::mem::size_of::<T>(), "ZST not supported");
        let mut data = Vec::<I>::with_capacity(Self::vec_cap(info.capacity()));
        let inner = data.as_mut_ptr();
        unsafe {
            ptr::write(&mut *inner, info);
        }
        mem::forget(data);
        PrefixedArray { inner: inner as _ }
    }

    fn vec_cap(len: usize) -> usize {
        // TODO: wrapping
        // TODO: allocate bytes, alignment?
        let bytes = mem::size_of::<I>() + mem::size_of::<T>() * len;
        div_round_up(bytes, mem::size_of::<I>())
    }

    /// Write `value` to inner data at `offset` using `std::ptr::write`.
    #[inline]
    pub unsafe fn write(&mut self, offset: usize, value: T) {
        ptr::write(self.data_mut().offset(offset as isize), value);
    }

    /// Returns the inner data as a slice with length `info.len()`.
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.data(), self.len()) }
    }

    /// Returns the inner data as a mutable slice with length `info.len()`.
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.data_mut(), self.len()) }
    }

    /// Creates a new `PrefixedArray` that share the inner data with `self`.
    #[inline]
    pub unsafe fn clone_shallow(&self) -> Self {
        PrefixedArray { inner: self.inner }
    }

    /// Drop the array elements, the info and deallocate.
    #[inline]
    pub unsafe fn drop_and_deallocate(&mut self) {
        let cap = self.info().capacity();
        ptr::drop_in_place(self.as_mut_slice());
        ptr::drop_in_place(&mut *self);
        Vec::from_raw_parts(self.inner, 0, Self::vec_cap(cap));
    }

    pub fn info(&self) -> &I {
        &self.inner().info
    }

    pub fn info_mut(&mut self) -> &mut I {
        &mut self.inner_mut().info
    }

    fn data(&self) -> *const T {
        self.inner().data.as_ptr()
    }

    fn data_mut(&mut self) -> *mut T {
        self.inner_mut().data.as_mut_ptr()
    }

    fn len(&self) -> usize {
        self.info().len()
    }

    fn inner(&self) -> &Inner<I, T> {
        unsafe { &*self.inner }
    }

    fn inner_mut(&mut self) -> &mut Inner<I, T> {
        unsafe { &mut *self.inner }
    }
}

#[inline]
fn div_round_up(divident: usize, divisor: usize) -> usize {
    1 + ((divident - 1) / divisor)
}

#[cfg(test)]
mod tests {
    use super::{PrefixedArray, PrefixedArrayInfo, div_round_up};
    use testdrop::{self, TestDrop};

    #[test]
    fn test_div_round_up() {
        assert_eq!(6, div_round_up(6, 1));
        assert_eq!(7, div_round_up(7, 1));
        assert_eq!(8, div_round_up(8, 1));

        assert_eq!(3, div_round_up(6, 2));
        assert_eq!(4, div_round_up(7, 2));
        assert_eq!(4, div_round_up(8, 2));

        assert_eq!(2, div_round_up(6, 3));
        assert_eq!(3, div_round_up(7, 3));
        assert_eq!(3, div_round_up(8, 3));
        assert_eq!(3, div_round_up(9, 3));
        assert_eq!(4, div_round_up(10, 3));
    }

    struct Info {
        len: usize,
        cap: usize,
    }

    impl PrefixedArrayInfo for Info {
        fn len(&self) -> usize {
            self.len
        }

        fn capacity(&self) -> usize {
            self.cap
        }
    }

    #[test]
    fn drop_and_deallocate() {
        let test = TestDrop::new();
        let v = &mut PrefixedArray::allocate(Info { len: 0, cap: 5 });

        let (a, item_a) = test.new_item();
        let (b, item_b) = test.new_item();
        let (c, item_c) = test.new_item();
        let (d, item_d) = test.new_item();
        let (e, item_e) = test.new_item();

        fn test_write<'a>(
            v: &mut PrefixedArray<Info, testdrop::Item<'a>>,
            id: usize,
            item: testdrop::Item<'a>,
            index: usize,
        ) {
            unsafe {
                let len = v.inner().info.len;
                v.write(len, item);
                v.inner_mut().info.len = len + 1;
            }
            assert_eq!(index + 1, v.len());
            assert_eq!(index + 1, v.as_slice().len());
            assert_eq!(index + 1, v.as_mut_slice().len());
            assert_eq!(id, v.as_slice()[index].id());
            assert_eq!(id, v.as_mut_slice()[index].id());
        }

        test_write(v, a, item_a, 0);
        test_write(v, b, item_b, 1);
        test_write(v, c, item_c, 2);
        test_write(v, d, item_d, 3);
        test_write(v, e, item_e, 4);

        // change len so d and e will not be dropped
        v.inner_mut().info.len = 3;

        unsafe {
            v.drop_and_deallocate();
        }

        test.assert_drop(a);
        test.assert_drop(b);
        test.assert_drop(c);
        test.assert_no_drop(d);
        test.assert_no_drop(e);
    }
}
