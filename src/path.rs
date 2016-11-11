use graph::*;
use traverse::*;

pub type Path<G> = VecEdge<G>;

pub type ParentTree<G> = DefaultVertexPropMut<G, OptionEdge<G>>;

pub trait FindPath: IncidenceGraph {
    fn find_path_on_parent_tree(&self,
                                tree: &ParentTree<Self>,
                                u: Vertex<Self>,
                                v: Vertex<Self>)
                                -> Option<Path<Self>> {
        if u == v {
            return None;
        }
        let mut v = v;
        let mut path = vec![];
        // TODO: detect loop
        while let Some(&e) = tree[v].to_option_ref() {
            v = self.source(e);
            path.push(e);
            if v == u {
                path.reverse();
                return Some(path);
            }
        }
        None
    }

    fn find_path(&self, u: Vertex<Self>, v: Vertex<Self>) -> Option<Path<Self>> {
        if u == v {
            return None;
        }
        let mut found = false;
        let mut tree = self.default_vertex_prop(Self::edge_none());
        self.dfs_with_root(u,
                           DiscoverTreeEdge(|e| {
                               let t = self.target(e);
                               tree[t] = Self::edge_some(e);
                               found = t == v;
                               break_if(found)
                           }));
        if found {
            self.find_path_on_parent_tree(&tree, u, v)
        } else {
            None
        }
    }
}

impl<G> FindPath for G where G: IncidenceGraph {}

#[cfg(test)]
mod tests {
    use graph::*;
    use static_::*;
    use fera::IteratorExt;
    use path::*;

    #[test]
    fn find_path() {
        let g = graph!(StaticGraph, 6, (0, 1), (0, 2), (1, 4), (2, 3), (2, 4));
        let e = g.edges().into_vec();

        assert_eq!(None, g.find_path(0, 0));

        assert_eq!(None, g.find_path(0, 5));

        assert_eq!(vec![e[0]], g.find_path(0, 1).unwrap());

        assert_eq!(vec![e[0], e[1], e[4]], g.find_path(1, 4).unwrap());
    }
}

#[cfg(all(feature = "nightly", test))]
mod benchs {
    use super::*;
    use static_::*;
    use graph::*;
    use builder::WithBuilder;
    use rand::{SeedableRng, StdRng};
    use test::Bencher;

    fn bench_find_path_n(b: &mut Bencher, n: usize) {
        let mut rng = StdRng::from_seed(&[123]);
        let g = StaticGraph::random_tree(n, &mut rng);
        b.iter(|| {
            for e in g.edges() {
                let (u, v) = g.ends(e);
                assert!(g.find_path(v, u).is_some());
            }
        })
    }

    #[bench]
    fn bench_find_path_10(b: &mut Bencher) {
        bench_find_path_n(b, 10);
    }

    #[bench]
    fn bench_find_path_100(b: &mut Bencher) {
        bench_find_path_n(b, 100);
    }

    #[bench]
    fn bench_find_path_1000(b: &mut Bencher) {
        bench_find_path_n(b, 1000);
    }
}
