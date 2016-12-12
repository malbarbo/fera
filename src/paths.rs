use graph::*;
use traverse::*;

pub trait Paths: WithEdge {
    fn find_path(&self, u: Vertex<Self>, v: Vertex<Self>) -> Option<VecEdge<Self>>
        where Self: DfsDefault
    {
        if u == v {
            return None;
        }
        let mut path = vec![];
        self.dfs_with_root(u, RecordPath(&mut path, v));
        if path.is_empty() { None } else { Some(path) }
    }
}

impl<G> Paths for G where G: Incidence {}


pub struct RecordPath<'a, G: WithEdge> {
    path: &'a mut Vec<Edge<G>>,
    target: Vertex<G>,
}

#[allow(non_snake_case)]
pub fn RecordPath<'a, G>(path: &'a mut Vec<Edge<G>>, target: Vertex<G>) -> RecordPath<'a, G>
    where G: WithEdge
{
    RecordPath {
        path: path,
        target: target,
    }
}

impl<'a, G: WithEdge> Visitor<G> for RecordPath<'a, G> {
    fn discover_tree_edge(&mut self, g: &G, e: Edge<G>) -> Control {
        self.path.push(e);
        break_if(g.target(e) == self.target)
    }

    fn finish_tree_edge(&mut self, _g: &G, e: Edge<G>) -> Control {
        let r = self.path.pop();
        debug_assert_eq!(Some(e), r);
        Control::Continue
    }
}


#[cfg(test)]
mod tests {
    use super::Paths;
    use static_::StaticGraph;
    use graph::*;
    use fera::IteratorExt;

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
    use super::Paths;
    use static_::StaticGraph;
    use graph::*;
    use builder::*;
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
