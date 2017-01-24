use std::collections::HashSet;
use std::hash::Hash;

pub fn vec<I: IntoIterator>(iter: I) -> Vec<I::Item>
    where I: IntoIterator
{
    iter.into_iter().collect()
}

pub fn set<I: IntoIterator>(iter: I) -> HashSet<I::Item>
    where I: IntoIterator,
          I::Item: Hash + Eq
{
    iter.into_iter().collect()
}
