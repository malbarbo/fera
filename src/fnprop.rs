use graph::*;

use std::marker::PhantomData;

#[derive(Clone, Copy)]
pub struct FnProp<F, G> {
    fun: F,
    _marker: PhantomData<*const G>
}

impl<F, G> FnProp<F, G> {
    pub fn new(fun: F) -> Self {
        FnProp {
            fun: fun,
            _marker: PhantomData,
        }
    }
}

impl<F, G, I, T> PropGet<I> for FnProp<F, G>
    where F: Fn(I) -> T,
          T: Sized
{
    type Output = T;
    #[inline(always)]
    fn get(&self, item: I) -> T {
        (self.fun)(item)
    }
}
