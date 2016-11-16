use graph::*;
use traverse::*;

// TODO: this can be methods on Traverser
pub trait Props: IncidenceGraph {
    fn is_acyclic(&self) -> bool {
        let mut acyclic = true;
        self.dfs(OnDiscoverBackEdge(|_| {
            acyclic = false;
            Control::Break
        }));
        acyclic
    }

    fn is_connected(&self) -> bool {
        self.num_vertices() == 0 ||
        {
            let mut count = 0;
            self.dfs(OnDiscoverRootVertex(|_| {
                count += 1;
                break_if(count != 1)
            }));
            count == 1
        }
    }

    // FIXME: does not work for multigraph
    fn is_tree(&self) -> bool {
        self.num_vertices() == 0 ||
        {
            self.num_edges() == self.num_vertices() - 1 && self.is_acyclic()
        }
    }
}

impl<G> Props for G where G: IncidenceGraph {}

#[cfg(test)]
mod tests {
    use static_::*;
    use super::*;

    struct Case {
        g: StaticGraph,
        is_connected: bool,
        is_acyclic: bool,
        is_tree: bool,
    }

    fn cases() -> Vec<Case> {
        vec![
            Case { // 0
                g: graph!(StaticGraph),
                is_connected: true,
                is_acyclic: true,
                is_tree: true,
            },
            Case { // 1
                g: graph!(StaticGraph, 1),
                is_connected: true,
                is_acyclic: true,
                is_tree: true,
            },
            Case { // 2
                g: graph!(StaticGraph, 2),
                is_connected: false,
                is_acyclic: true,
                is_tree: false,
            },
            Case { // 3
                g: graph!(StaticGraph, 2, (0, 1)),
                is_connected: true,
                is_acyclic: true,
                is_tree: true,
            },
            Case { // 4
                g: graph!(StaticGraph, 3, (2, 1)),
                is_connected: false,
                is_acyclic: true,
                is_tree: false,
            },
            Case { // 5
                g: graph!(StaticGraph, 3, (2, 1)),
                is_connected: false,
                is_acyclic: true,
                is_tree: false,
            },
            Case { // 6
                g: graph!(StaticGraph, 3, (0, 1), (1, 2)),
                is_connected: true,
                is_acyclic: true,
                is_tree: true,
            },
            Case { // 7
                g: graph!(StaticGraph, 3, (0, 1), (0, 2), (1, 2)),
                is_connected: true,
                is_acyclic: false,
                is_tree: false,
            },
            Case { // 8
                g: graph!(StaticGraph, 4, (0, 1), (0, 2)),
                is_connected: false,
                is_acyclic: true,
                is_tree: false,
            },
            Case { // 9
                g: graph!(StaticGraph, 4, (1, 2), (2, 3), (3, 1)),
                is_connected: false,
                is_acyclic: false,
                is_tree: false,
            },
        ]
    }

    #[test]
    fn is_connected() {
        for (i, case) in cases().iter().enumerate() {
            assert!(case.is_connected == case.g.is_connected(),
                    format!("Case {}", i));
        }
    }

    #[test]
    fn is_acyclic() {
        for (i, case) in cases().iter().enumerate() {
            assert!(case.is_acyclic == case.g.is_acyclic(),
                    format!("Case {}", i));
        }
    }

    #[test]
    fn is_tree() {
        for (i, case) in cases().iter().enumerate() {
            assert!(case.is_tree == case.g.is_tree(), format!("Case {}", i));
        }
    }
}

#[cfg(all(feature = "nightly", test))]
mod benchs {
    use super::*;
    use static_::*;
    use builder::WithBuilder;
    use rand::{SeedableRng, StdRng};
    use test::Bencher;

    fn bench_is_acyclic(b: &mut Bencher, n: usize) {
        let mut rng = StdRng::from_seed(&[123]);
        let g = StaticGraph::random_tree(n, &mut rng);
        b.iter(|| {
            assert!(g.is_acyclic());
        })
    }

    #[bench]
    fn bench_is_acyclic_10(b: &mut Bencher) {
        bench_is_acyclic(b, 10);
    }

    #[bench]
    fn bench_is_acyclic_100(b: &mut Bencher) {
        bench_is_acyclic(b, 100);
    }

    #[bench]
    fn bench_is_acyclic_1000(b: &mut Bencher) {
        bench_is_acyclic(b, 1000);
    }

    fn bench_is_connected(b: &mut Bencher, n: usize) {
        let mut rng = StdRng::from_seed(&[123]);
        let g = StaticGraph::random_tree(n, &mut rng);
        b.iter(|| {
            assert!(g.is_connected());
        })
    }

    #[bench]
    fn bench_is_connected_10(b: &mut Bencher) {
        bench_is_connected(b, 10);
    }

    #[bench]
    fn bench_is_connected_100(b: &mut Bencher) {
        bench_is_connected(b, 100);
    }

    #[bench]
    fn bench_is_connected_1000(b: &mut Bencher) {
        bench_is_connected(b, 1000);
    }
}
