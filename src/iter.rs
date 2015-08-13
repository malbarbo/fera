use std;

pub struct Map1<'a, I, D: 'a, F> {
    iter: I,
    data: &'a D,
    f: F,
}

impl<'a, A, I, D, F> Iterator for Map1<'a, I, D, F>
    where I:Iterator,
          F: FnMut(&'a D, I::Item) -> A {
    type Item = A;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|a| (self.f)(self.data, a))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
   }
}

pub trait IteratorExt: Iterator + Sized {
   fn map1<D, F>(self, data: &D, f: F) -> Map1<Self, D, F> {
      Map1 {
         iter: self,
         data: data,
         f: f
      }
   }

   fn as_vec(self) -> Vec<Self::Item> {
      self.collect()
   }

   fn as_set(self) -> std::collections::HashSet<Self::Item>
      where Self::Item: std::hash::Hash + Eq {
      self.collect()
   }
}

impl<I: Iterator> IteratorExt for I {}
