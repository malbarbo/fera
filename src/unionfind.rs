use graph::*;

use fera::unionfind;

pub type UnionFind<G> = unionfind::UnionFind<Vertex<G>,
                                             DefaultVertexPropMut<G, Vertex<G>>,
                                             DefaultVertexPropMut<G, usize>>;

pub trait WithUnionFind: Undirected + BasicProps {
    fn new_unionfind(&self) -> UnionFind<Self> {
        let mut ds = UnionFind::<Self>::with_parent_rank(self.vertex_prop(self.vertices()
                                                                              .next()
                                                                              .unwrap()),
                                                         self.vertex_prop(0usize));
        for v in self.vertices() {
            ds.make_set(v);
        }
        ds
    }
}

impl<G> WithUnionFind for G where G: Undirected + BasicProps {}


#[cfg(test)]
mod tests {
    use super::*;
    use graph::*;
    use static_::*;
    use fera::IteratorExt;

    fn check_groups(ds: &mut UnionFind<StaticGraph>, groups: &[&[Vertex<StaticGraph>]]) {
        for group in groups.iter() {
            for &a in group.iter() {
                assert!(ds.in_same_set(group[0], a));
            }
        }
    }

    #[test]
    fn unionfind() {
        let g = graph!(StaticGraph, 5);
        let v = g.vertices().into_vec();
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
