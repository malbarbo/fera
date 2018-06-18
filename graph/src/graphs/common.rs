// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use prelude::*;
use rand::Rng;

pub struct OutNeighborFromOutEdge<'a, G: 'a, I> {
    g: &'a G,
    iter: I,
}

impl<'a, G, I> OutNeighborFromOutEdge<'a, G, I>
where
    I: Iterator<Item = Edge<G>>,
    G: 'a + WithEdge,
{
    pub fn new(g: &'a G, iter: I) -> Self {
        OutNeighborFromOutEdge { g: g, iter: iter }
    }
}

impl<'a, G, I> Iterator for OutNeighborFromOutEdge<'a, G, I>
where
    I: Iterator<Item = Edge<G>>,
    G: 'a + WithEdge,
{
    type Item = Vertex<G>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|e| self.g.target(e))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, G, I> ExactSizeIterator for OutNeighborFromOutEdge<'a, G, I>
where
    I: Iterator<Item = Edge<G>> + ExactSizeIterator,
    G: 'a + WithEdge,
{
    #[inline]
    fn len(&self) -> usize {
        self.iter.len()
    }
}

pub(crate) fn gen_range_bool<R: Rng>(to: usize, mut rng: R) -> Option<(usize, bool)> {
    if to == 0 {
        return None;
    }
    if let Some(to) = to.checked_mul(2) {
        let i = rng.gen_range(0, to);
        Some((i / 2, i % 2 == 0))
    } else {
        Some((rng.gen_range(0, to), rng.gen()))
    }
}
