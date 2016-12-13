use props::PropGet;

#[derive(Clone, Copy)]
pub struct FnProp<F>(pub F);

impl<F, I, T> PropGet<I> for FnProp<F>
    where F: Fn(I) -> T,
          T: Sized
{
    type Output = T;

    #[inline(always)]
    fn get(&self, item: I) -> T {
        (self.0)(item)
    }
}
