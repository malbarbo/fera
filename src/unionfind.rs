use graph::*;

use ds::unionfind::GenericUnionFind;

pub type UnionFind<G> = GenericUnionFind<Vertex<G>,
                                         Vertex<G>,
                                         DefaultPropMutVertex<G, Vertex<G>>,
                                         DefaultPropMutVertex<G, usize>,
                                         usize>;

pub trait WithUnionFind: Graph {
    fn new_unionfind(&self) -> UnionFind<Self> {
        GenericUnionFind::new_with_all(self.vertices(),
                                       self.vertex_prop(self.vertices()
                                                            .next()
                                                            .unwrap()),
                                       self.vertex_prop(0usize))
    }
}

impl<G: Graph> WithUnionFind for G { }


#[cfg(test)]
mod tests {
    use super::*;
    use graph::*;
    use static_::*;
    use ds::IteratorExt;

    fn check_groups(ds: &mut UnionFind<StaticGraph>, groups: &[&[Vertex<StaticGraph>]]) {
        for group in groups.iter() {
            for &a in group.iter() {
                assert!(ds.in_same_set(group[0], a));
            }
        }
    }

    #[test]
    fn unionfind() {
        let g = StaticGraph::new_with_edges(5, &[]);
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
