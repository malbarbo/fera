use prelude::*;
use std::iter::Sum;

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

pub trait GraphsIteratorExt: Sized + IntoIterator {
    #[inline]
    fn min_by_prop<P, K>(self, prop: P) -> Option<Self::Item>
        where P: PropGet<K>,
              for<'a> &'a Self::Item: IntoOwned<K>,
              P::Output: Ord
    {
        self.into_iter().min_by_key(|v| prop.get(v.into_owned()))
    }

    #[inline]
    fn max_by_prop<P, K>(self, prop: P) -> Option<Self::Item>
        where P: PropGet<K>,
              for<'a> &'a Self::Item: IntoOwned<K>,
              P::Output: Ord
    {
        self.into_iter().max_by_key(|v| prop.get(v.into_owned()))
    }

    #[inline]
    fn min_prop<P, K>(self, prop: P) -> Option<P::Output>
        where P: PropGet<K>,
              Self::Item: IntoOwned<K>,
              P::Output: Ord
    {
        self.into_iter().map(|v| prop.get(v.into_owned())).min()
    }

    #[inline]
    fn max_prop<P, K>(self, prop: P) -> Option<P::Output>
        where P: PropGet<K>,
              Self::Item: IntoOwned<K>,
              P::Output: Ord
    {
        self.into_iter().map(|v| prop.get(v.into_owned())).max()
    }

    #[inline]
    fn sum_prop<P, K, O>(self, prop: P) -> O
        where P: PropGet<K>,
              Self::Item: IntoOwned<K>,
              O: Sum<P::Output>
    {
        self.into_iter().map(|v| prop.get(v.into_owned())).sum()
    }

    #[inline]
    fn ends<G>(self, g: &G) -> Ends<G, Self::IntoIter>
        where G: WithEdge,
              Self::Item: IntoOwned<Edge<G>>
    {
        Ends {
            g: g,
            iter: self.into_iter(),
        }
    }
}


impl<I: IntoIterator> GraphsIteratorExt for I {}

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


// Adaptors

pub struct Ends<'a, G: 'a, I> {
    g: &'a G,
    iter: I,
}

impl<'a, G, I> Iterator for Ends<'a, G, I>
    where G: WithEdge,
          I: Iterator,
          I::Item: IntoOwned<Edge<G>>
{
    type Item = (Vertex<G>, Vertex<G>);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|e| self.g.ends(e.into_owned()))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, G, I> ExactSizeIterator for Ends<'a, G, I>
    where G: WithEdge,
          I: Iterator,
          I::Item: IntoOwned<Edge<G>>
{
}
