use prelude::*;

use fera_unionfind::UnionFind as InnerUnionFind;

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

    pub fn reset(&mut self, g: &G) {
        for v in g.vertices() {
            self.inner.make_set(v)
        }
    }
}

pub trait WithUnionFind: Graph {
    fn new_unionfind(&self) -> UnionFind<Self> {
        let v = self.vertices().next().unwrap();
        let mut ds = InnerUnionFind::with_parent_rank(self.vertex_prop(v), self.vertex_prop(0));
        for v in self.vertices() {
            ds.make_set(v);
        }
        UnionFind { inner: ds }
    }
}

impl<G: Graph> WithUnionFind for G {}


#[cfg(test)]
mod tests {
    use super::{UnionFind, WithUnionFind};
    use prelude::*;
    use fera_fun::vec;

    fn check_groups(ds: &mut UnionFind<StaticGraph>, groups: &[&[Vertex<StaticGraph>]]) {
        for group in groups {
            for &a in *group {
                assert!(ds.in_same_set(group[0], a));
            }
        }
    }

    #[test]
    fn unionfind() {
        let g = graph!(StaticGraph, 5);
        let v = vec(g.vertices());
        let mut ds = g.new_unionfind();
        ds.union(v[0], v[2]);
        check_groups(&mut ds, &[&[v[0], v[2]]]);
        ds.union(v[1], v[3]);
        check_groups(&mut ds, &[&[v[0], v[2]], &[v[1], v[3]]]);
        ds.union(v[2], v[4]);
        check_groups(&mut ds, &[&[v[0], v[2], v[4]], &[v[1], v[3]]]);
        ds.union(v[3], v[4]);
        check_groups(&mut ds, &[&[v[0], v[2], v[4], v[1], v[3]]]);
    }
}
