use prelude::*;

// TODO: Create an IntoIteratorOwned
// FIXME: move to other module
pub trait IntoOwned<Owned> {
    fn into_owned(self) -> Owned;
}

impl<T> IntoOwned<T> for T {
    #[inline]
    fn into_owned(self) -> T {
        self
    }
}

impl<'a, T: Clone> IntoOwned<T> for &'a T {
    #[inline]
    fn into_owned(self) -> T {
        T::clone(self)
    }
}

impl<'a, T: Clone> IntoOwned<T> for &'a mut T {
    #[inline]
    fn into_owned(self) -> T {
        T::clone(self)
    }
}


pub trait GraphsSliceExt<T> {
    fn sort_by_prop<P, K>(&mut self, prop: P)
        where P: PropGet<K>,
              for<'a> &'a T: IntoOwned<K>,
              P::Output: Ord;
}

impl<T> GraphsSliceExt<T> for [T] {
    #[inline]
    fn sort_by_prop<P, K>(&mut self, prop: P)
        where P: PropGet<K>,
              for<'a> &'a T: IntoOwned<K>,
              P::Output: Ord
    {
        self.sort_by_key(|v| prop.get(v.into_owned()))
    }
}


pub trait GraphsVecExt<T> {
    fn sorted_by_prop<P, K>(self, prop: P) -> Self
        where P: PropGet<K>,
              for<'a> &'a T: IntoOwned<K>,
              P::Output: Ord;
}

impl<T> GraphsVecExt<T> for Vec<T> {
    #[inline]
    fn sorted_by_prop<P, K>(mut self, prop: P) -> Self
        where P: PropGet<K>,
              for<'a> &'a T: IntoOwned<K>,
              P::Output: Ord
    {
        self.sort_by_key(|v| prop.get(v.into_owned()));
        self
    }
}
