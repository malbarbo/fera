use prelude::*;
use ext::IntoOwned;
use std::iter::Sum;

#[inline]
pub fn min_by_prop<I, P, K>(iter: I, prop: P) -> Option<I::Item>
    where I: IntoIterator,
          for<'a> &'a I::Item: IntoOwned<K>,
          P: PropGet<K>,
          P::Output: Ord
{
    iter.into_iter().min_by_key(|v| prop.get(v.into_owned()))
}

#[inline]
pub fn max_by_prop<I, P, K>(prop: P, iter: I) -> Option<I::Item>
    where I: IntoIterator,
          for<'a> &'a I::Item: IntoOwned<K>,
          P: PropGet<K>,
          P::Output: Ord
{
    iter.into_iter().max_by_key(|v| prop.get(v.into_owned()))
}

#[inline]
pub fn min_prop<P, K, I>(prop: P, iter: I) -> Option<P::Output>
    where I: IntoIterator,
          I::Item: IntoOwned<K>,
          P: PropGet<K>,
          P::Output: Ord
{

    iter.into_iter().map(|v| prop.get(v.into_owned())).min()
}

#[inline]
pub fn max_prop<P, K, I>(prop: P, iter: I) -> Option<P::Output>
    where I: IntoIterator,
          I::Item: IntoOwned<K>,
          P: PropGet<K>,
          P::Output: Ord
{
    iter.into_iter().map(|v| prop.get(v.into_owned())).max()
}

#[inline]
pub fn sum_prop<P, K, O, I>(prop: P, iter: I) -> O
    where I: IntoIterator,
          I::Item: IntoOwned<K>,
          P: PropGet<K>,
          O: Sum<P::Output>
{
    iter.into_iter().map(|v| prop.get(v.into_owned())).sum()
}
