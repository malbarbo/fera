// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![doc(html_root_url="https://docs.rs/fera-unionfind/0.1.0/")]

//! Union-find ([disjoint-set]) data structure implementation.
//!
//! This implementation use path compression and rank heuristic. With default type parameters the
//! parents and ranks are stored in a [`std::collections::HashMap`]. If the keys are in range
//! `0..n`, use [`UnionFindRange`].
//!
//! The keys should implement `Copy`. If the keys does not implement `Copy`, references to the keys
//! stored elsewhere should be used.
//!
//! This crate can be used through [`fera`] crate.
//!
//!
//! # Examples
//!
//! ```
//! use fera_unionfind::UnionFind;
//!
//! // Explicit type to make it clear
//! let mut s: UnionFind<&'static str> = UnionFind::new();
//!
//! s.make_set("red");
//! s.make_set("green");
//! s.make_set("blue");
//!
//! assert_eq!(3, s.num_sets());
//! assert!(!s.in_same_set("red", "green"));
//! assert!(!s.in_same_set("red", "blue"));
//!
//! s.union("red", "blue");
//!
//! assert_eq!(2, s.num_sets());
//! assert!(!s.in_same_set("red", "green"));
//! assert!(s.in_same_set("red", "blue"));
//! ```
//!
//! Using non `Copy` keys.
//!
//! ```
//! use fera_unionfind::UnionFind;
//!
//! // This is invalid. String does not implement copy.
//! // let mut x: UnionFind<String> = UnionFind::new();
//! // Lets store the keys in a vector and use references (references are Copy).
//! let v = vec!["red".to_string(), "green".to_string(), "blue".to_string()];
//!
//! // The type of s is Union<&'a String> where 'a is the lifetime of v.
//! let mut s = UnionFind::new();
//!
//! s.make_set(&v[0]);
//! s.make_set(&v[1]);
//! s.make_set(&v[2]);
//!
//! assert_eq!(3, s.num_sets());
//! assert!(!s.in_same_set(&v[0], &v[1]));
//! assert!(!s.in_same_set(&v[0], &v[2]));
//!
//! s.union(&v[0], &v[2]);
//!
//! assert_eq!(2, s.num_sets());
//! assert!(!s.in_same_set(&v[0], &v[1]));
//! assert!(s.in_same_set(&v[0], &v[2]));
//! ```
//!
//! Using keys in the range `0..n`.
//!
//! ```
//! use fera_unionfind::UnionFindRange;
//!
//! let mut s = UnionFindRange::with_keys_in_range(..5);
//!
//! // It is not necessary to call UnionFind::make_set
//!
//! assert_eq!(5, s.num_sets());
//! assert!(!s.in_same_set(0, 1));
//! assert!(!s.in_same_set(0, 2));
//!
//! s.union(0, 2);
//!
//! assert_eq!(4, s.num_sets());
//! assert!(!s.in_same_set(0, 1));
//! assert!(s.in_same_set(0, 2));
//!
//! s.reset();
//! assert_eq!(5, s.num_sets());
//! ```
//!
//!
//! [disjoint-set]: https://en.wikipedia.org/wiki/Disjoint-set_data_structure
//! [`fera`]: https://docs.rs/fera
//! [`std::collections::HashMap`]: https://doc.rust-lang.org/stable/std/collections/struct.HashMap.html
//! [`UnionFindRange`]: type.UnionFindRange.html

#![cfg_attr(feature = "cargo-clippy", allow(inline_always))]

// TODO: remove fnv dependency
extern crate fnv;

use std::collections::HashMap;
use std::hash::BuildHasherDefault;
use std::hash::Hash;
use std::marker::PhantomData;
use std::ops::{Index, IndexMut, RangeTo};

type HashMapFnv<K, V> = HashMap<K, V, BuildHasherDefault<fnv::FnvHasher>>;

/// [`UnionFind`] with keys in range `0..n`.
///
/// [`UnionFind`]: struct.UnionFind.html
pub type UnionFindRange = UnionFind<usize, Vec<usize>, Vec<usize>>;

/// A union-find ([disjoint-set]) struct.
///
/// [disjoint-set]: https://en.wikipedia.org/wiki/Disjoint-set_data_structure
#[derive(Clone)]
pub struct UnionFind<Key, Parent = IndexedHashMap<Key, Key>, Rank = IndexedHashMap<Key, usize>>
    where Key: Copy + PartialEq,
          Parent: IndexMut<Key, Output = Key>,
          Rank: IndexMut<Key, Output = usize>
{
    parent: Parent,
    rank: Rank,
    num_sets: usize,
    _marker: PhantomData<Key>,
}

impl<Key, Parent, Rank> UnionFind<Key, Parent, Rank>
    where Key: Copy + PartialEq,
          Parent: IndexMut<Key, Output = Key>,
          Rank: IndexMut<Key, Output = usize>
{
    /// Creates a new `UnionFind`.
    #[doc(hidden)]
    pub fn with_parent_rank_num_sets(parent: Parent, rank: Rank, num_sets: usize) -> Self {
        UnionFind {
            parent: parent,
            rank: rank,
            num_sets: num_sets,
            _marker: PhantomData,
        }
    }

    /// Adds the key in it's own set. The number of sets is increased by 1.
    ///
    /// It's undefined behavior to call this method with a key that is already in a set.
    pub fn make_set(&mut self, x: Key) {
        // TODO: if x has a parent?
        self.set_parent(x, x);
        self.set_rank(x, 0);
        self.num_sets += 1;
    }

    /// Joins the sets with the keys `x` and `y`. The number of sets is decreased by 1.
    ///
    /// # Panics
    ///
    /// If `x` or `y` is not in any set or if both are in the same set.
    pub fn union(&mut self, x: Key, y: Key) {
        let a = self.find_set(x);
        let b = self.find_set(y);
        assert!(a != b);
        self.link(a, b);
    }

    /// Returns `true` if `x` and `y` is in the same set, otherwise `false`.
    ///
    /// # Panics
    ///
    /// If `x` or `y` is not in any set.
    pub fn in_same_set(&mut self, x: Key, y: Key) -> bool {
        self.find_set(x) == self.find_set(y)
    }

    /// Returns the representative of the set that contains `x`.
    ///
    /// # Panics
    ///
    /// If `x` is not in any set.
    pub fn find_set(&mut self, mut x: Key) -> Key {
        while self.parent(x) != x {
            let p = self.parent(self.parent(x));
            self.set_parent(x, p);
            x = p;
        }
        self.parent(x)
    }

    /// Returns the number of distinct sets.
    pub fn num_sets(&self) -> usize {
        self.num_sets
    }

    fn link(&mut self, x: Key, y: Key) {
        self.num_sets -= 1;
        if self.rank(x) > self.rank(y) {
            self.set_parent(y, x);
        } else {
            self.set_parent(x, y);
            if self.rank(x) == self.rank(y) {
                self.inc_rank(y);
            }
        }
    }

    #[inline(always)]
    fn inc_rank(&mut self, k: Key) {
        self.rank[k] += 1;
    }

    #[inline(always)]
    fn rank(&self, k: Key) -> usize {
        self.rank[k]
    }

    #[inline(always)]
    fn set_rank(&mut self, k: Key, rank: usize) {
        self.rank[k] = rank;
    }

    #[inline(always)]
    fn parent(&self, k: Key) -> Key {
        self.parent[k]
    }

    #[inline(always)]
    fn set_parent(&mut self, k: Key, p: Key) {
        self.parent[k] = p;
    }
}

impl<K: Copy + Hash + Eq> UnionFind<K> {
    /// Creates a new [`UnionFind`].
    ///
    /// [`UnionFind`]: struct.UnionFind.html
    pub fn new() -> Self {
        fn zero<K: Clone>(_: &K) -> usize {
            0
        }

        UnionFind::with_parent_rank_num_sets(IndexedHashMap::new(Clone::clone),
                                             IndexedHashMap::new(zero),
                                             0)
    }
}

impl UnionFindRange {
    /// Creates a new `UnionFindRange` with keys in `range`.
    pub fn with_keys_in_range(range: RangeTo<usize>) -> Self {
        UnionFind::with_parent_rank_num_sets((0..range.end).collect(),
                                             vec![0; range.end],
                                             range.end)
    }

    /// Reset the struct putting each key in it's own set.
    // TODO: how to implement this method for any UnionFind?
    pub fn reset(&mut self) {
        let n = self.parent.len();
        for i in 0..n {
            self.parent[i] = i;
            self.rank[i] = 0;
        }
        self.num_sets = n;
    }
}

/// This implements a map that can be used with [`UnionFind`].
///
/// [`UnionFind`]: struct.UnionFind.html
// TODO: allow the hasher to be specified
pub struct IndexedHashMap<K, V>
    where K: Copy + Hash + Eq
{
    map: HashMapFnv<K, V>,
    default: fn(&K) -> V,
}

impl<K, V> IndexedHashMap<K, V>
    where K: Copy + Hash + Eq
{
    fn new(f: fn(&K) -> V) -> Self {
        IndexedHashMap {
            map: HashMapFnv::default(),
            default: f,
        }
    }
}

impl<K, V> Index<K> for IndexedHashMap<K, V>
    where K: Copy + Hash + Eq
{
    type Output = V;
    fn index(&self, index: K) -> &Self::Output {
        &self.map[&index]
    }
}

impl<K, V> IndexMut<K> for IndexedHashMap<K, V>
    where K: Copy + Hash + Eq
{
    fn index_mut(&mut self, index: K) -> &mut Self::Output {
        let f = self.default;
        self.map.entry(index).or_insert_with(|| f(&index))
    }
}


#[cfg(test)]
mod tests {
    use *;

    type UF = UnionFind<usize, Vec<usize>, Vec<usize>>;

    fn check(ds: &mut UF, num_sets: usize, groups: &[&[usize]]) {
        assert_eq!(num_sets, ds.num_sets());
        for group in groups {
            for &a in *group {
                assert!(ds.in_same_set(group[0], a));
            }
        }
    }

    #[test]
    fn unionfind1() {
        let mut ds = UnionFind::with_keys_in_range(..5);
        check(&mut ds, 5, &[&[]]);
        ds.union(0, 2);
        check(&mut ds, 4, &[&[0, 2]]);
        ds.union(1, 3);
        check(&mut ds, 3, &[&[0, 2], &[1, 3]]);
        ds.union(2, 4);
        check(&mut ds, 2, &[&[0, 2, 4], &[1, 3]]);
        ds.union(3, 4);
        check(&mut ds, 1, &[&[0, 2, 4, 1, 3]]);
    }

    #[test]
    fn unionfind2() {
        let mut ds = UnionFind::with_keys_in_range(..16);
        ds.union(0, 1);
        ds.union(2, 3);
        ds.union(4, 5);
        ds.union(6, 7);

        ds.union(1, 2);
        ds.union(5, 6);
        ds.union(3, 7);

        ds.union(8, 9);
        ds.union(10, 11);
        ds.union(12, 13);
        ds.union(14, 15);

        ds.union(9, 10);
        ds.union(13, 14);
        ds.union(11, 15);

        ds.union(7, 15);

        check(&mut ds,
              1,
              &[&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]]);
    }
}
