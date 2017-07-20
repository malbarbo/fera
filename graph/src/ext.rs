//! Extension traits for std types.

use prelude::*;
use params::IntoOwned;

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
