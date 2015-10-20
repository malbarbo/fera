use graph::*;

use ds;

pub type UnionFind<G> = ds::unionfind::UnionFind<Vertex<G>,
                                                 PropVertex<G, Vertex<G>>,
                                                 PropVertex<G, usize>>;

pub trait WithUnionFind: Graph {
    fn new_unionfind<'a>(&'a self) -> UnionFind<Self>
        where &'a Self: IterTypes<Self>
    {
        ds::unionfind::UnionFind::new_with_all(self.vertices(),
                                               self.vertex_prop(self.vertices().next().unwrap()),
                                               self.vertex_prop(0usize))
    }
}

impl<G: Graph> WithUnionFind for G { }


#[cfg(test)]
mod tests {
    use super::*;
    use static_::*;

    fn check_groups(ds: &mut UnionFind<StaticGraph>, groups: &[&[usize]]) {
        for group in groups.iter() {
            for &a in group.iter() {
                assert!(ds.in_same_set(group[0], a));
            }
        }
    }

    #[test]
    fn unionfind() {
        let g = StaticGraph::new_with_edges(5, &[]);
        let mut ds = g.new_unionfind();
        ds.union(0, 2);
        check_groups(&mut ds, &[&[0, 2]]);
        ds.union(1, 3);
        check_groups(&mut ds, &[&[0, 2], &[1, 3]]);
        ds.union(2, 4);
        check_groups(&mut ds, &[&[0, 2, 4], &[1, 3]]);
        ds.union(3, 4);
        check_groups(&mut ds, &[&[0, 2, 4, 1, 3]]);
    }
}
