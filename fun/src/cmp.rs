/// ```
/// use fera_fun::position_max_by_key;
///
/// assert_eq!(Some(4), position_max_by_key(&[0i32, 3, -5, 0, 5], |x| x.abs()));
/// ```
pub fn position_max_by_key<I, F, X>(iter: I, mut f: F) -> Option<usize>
    where I: IntoIterator,
          X: Ord,
          F: FnMut(&I::Item) -> X
{
    iter.into_iter()
        .enumerate()
        .max_by_key(|x| f(&x.1))
        .map(|x| x.0)
}

/// ```
/// use fera_fun::position_min_by_key;
///
/// assert_eq!(Some(0), position_min_by_key(&[0i32, 3, -5, 0, 5], |x| x.abs()));
/// ```
pub fn position_min_by_key<I, F, X>(iter: I, mut f: F) -> Option<usize>
    where I: IntoIterator,
          X: Ord,
          F: FnMut(&I::Item) -> X
{
    iter.into_iter()
        .enumerate()
        .min_by_key(|x| f(&x.1))
        .map(|x| x.0)
}
