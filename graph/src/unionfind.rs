// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Union-find ([disjoint-set]) data structure.
//!
//! [disjoint-set]: https://en.wikipedia.org/wiki/Disjoint-set_data_structure
use prelude::*;
use params::{ParamDerefMut, Owned};

use fera_fun::first;
use fera_unionfind::UnionFind as InnerUnionFind;

// FIXME: only union and reset should need &mut self

pub struct UnionFind<G: Graph> {
    inner: InnerUnionFind<Vertex<G>,
                          DefaultVertexPropMut<G, Vertex<G>>,
                          DefaultVertexPropMut<G, usize>>,
}

impl<G: Graph> UnionFind<G> {
    #[inline]
    pub fn union(&mut self, u: Vertex<G>, v: Vertex<G>) {
        self.inner.union(u, v)
    }

    #[inline]
    pub fn in_same_set(&mut self, u: Vertex<G>, v: Vertex<G>) -> bool {
        self.inner.in_same_set(u, v)
    }

    #[inline]
    pub fn find_set(&mut self, v: Vertex<G>) -> Vertex<G> {
        self.inner.find_set(v)
    }

    #[inline]
    pub fn num_sets(&self) -> usize {
        self.inner.num_sets()
    }

    pub fn reset(&mut self, g: &G) {
        for v in g.vertices() {
            self.inner.make_set(v)
        }
    }
}

pub trait WithUnionFind: Graph {
    fn new_unionfind(&self) -> UnionFind<Self> {
        // FIXME: do not work with null graphs
        let v = first(self.vertices());
        let mut parent = self.default_vertex_prop(v);
        for v in self.vertices() {
            parent[v] = v;
        }
        UnionFind {
            inner: InnerUnionFind::with_parent_rank_num_sets(parent,
                                                             self.vertex_prop(0),
                                                             self.num_vertices()),
        }
    }
}

impl<G: Graph> WithUnionFind for G {}


pub struct NewUnionFind<'a, G: 'a>(pub &'a G);

impl<'a, G: 'a + WithUnionFind> ParamDerefMut for NewUnionFind<'a, G> {
    type Target = UnionFind<G>;
    type Output = Owned<UnionFind<G>>;

    fn build(self) -> Self::Output {
        Owned(self.0.new_unionfind())
    }
}


#[cfg(test)]
mod tests {
    use super::{UnionFind, WithUnionFind};
    use prelude::*;
    use fera_fun::vec;

    fn check_groups(ds: &mut UnionFind<StaticGraph>,
                    num_sets: usize,
                    groups: &[&[Vertex<StaticGraph>]]) {
        assert_eq!(num_sets, ds.num_sets());
        for group in groups {
            for &a in *group {
                assert!(ds.in_same_set(group[0], a));
            }
        }
    }

    #[test]
    fn unionfind() {
        let g: StaticGraph = graph!(5);
        let v = vec(g.vertices());
        let mut ds = g.new_unionfind();
        assert_eq!(5, ds.num_sets());
        ds.union(v[0], v[2]);
        check_groups(&mut ds, 4, &[&[v[0], v[2]]]);
        ds.union(v[1], v[3]);
        check_groups(&mut ds, 3, &[&[v[0], v[2]], &[v[1], v[3]]]);
        ds.union(v[2], v[4]);
        check_groups(&mut ds, 2, &[&[v[0], v[2], v[4]], &[v[1], v[3]]]);
        ds.union(v[3], v[4]);
        check_groups(&mut ds, 1, &[&[v[0], v[2], v[4], v[1], v[3]]]);
    }
}
