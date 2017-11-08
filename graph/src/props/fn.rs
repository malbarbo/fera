// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use props::PropGet;

/// A read only property backed by a function.
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
