use prelude::*;
use props::Color;
use trees::Trees;
use fera_fun::set;

use rand::{Rng, XorShiftRng};
use rand::distributions::{IndependentSample, Range};

#[macro_export]
macro_rules! graph {
    () => (
        {
            use $crate::builder::WithBuilder;
            WithBuilder::new_empty(0)
        }
    );

    ($n:expr) => (
        {
            use $crate::builder::WithBuilder;
            WithBuilder::new_empty($n)
        }
    );

    ($n:expr, $(($u:expr, $v:expr)),+) => (
        {
            use $crate::builder::WithBuilder;
            let edges = [$(($u, $v)),*];
            WithBuilder::new_with_edges($n, &edges)
        }
    );

    ($n:expr, $(($u:expr, $v:expr)),+,) => (
        graph!($n, $(($u, $v)),+)
    );

    ($n:expr, $(($u:expr, $v:expr) -> $p:expr),+) => (
        {
            use $crate::builder::WithBuilder;
            let edges = [$(($u, $v, $p)),*];
            WithBuilder::new_with_edges_prop($n, &edges)
        }
    );

    ($n:expr, $(($u:expr, $v:expr) -> $p:expr),+,) => (
        graph!($n, $(($u, $v) -> $p),+)
    );
}

pub trait Builder {
    type Graph: WithEdge;

    fn new(num_vertices: usize, num_edges: usize) -> Self;

    fn add_edge(&mut self, u: usize, v: usize);

    fn finalize(self) -> Self::Graph;

    fn finalize_(self) -> (Self::Graph, Vec<Vertex<Self::Graph>>, Vec<Edge<Self::Graph>>);
}

pub trait WithBuilder: WithEdge {
    type Builder: Builder<Graph = Self>;

    fn builder(num_vertices: usize, num_edges: usize) -> Self::Builder {
        Self::Builder::new(num_vertices, num_edges)
    }

    fn new_empty(n: usize) -> Self {
        Self::Builder::new(n, 0).finalize()
    }

    fn new_with_edges(n: usize, edges: &[(usize, usize)]) -> Self {
        let mut b = Self::Builder::new(n, edges.len());
        for &(u, v) in edges {
            b.add_edge(u, v);
        }
        b.finalize()
    }

    #[doc(hidden)]
    fn new_with_edges_prop<T>(n: usize,
                              edges: &[(usize, usize, T)])
                              -> (Self, DefaultEdgePropMut<Self, T>)
        where T: Copy + Default,
              Self: WithEdgeProp<T>
    {
        // TODO: Should this be optimized?
        let mut b = Self::Builder::new(n, edges.len());
        for &(ref u, ref v, _) in edges {
            b.add_edge(*u, *v);
        }
        let (g, _, ee) = b.finalize_();
        let mut p = g.default_edge_prop(T::default());
        for (e, val) in ee.into_iter().zip(edges.iter().map(|x| x.2)) {
            p[e] = val;
        }
        (g, p)
    }

    fn new_complete(n: usize) -> Self {
        complete::<Self>(n).finalize()
    }

    fn new_complete_binary_tree(height: u32) -> Self {
        complete_binary_tree::<Self>(height).finalize()
    }

    fn new_random_tree<R: Rng>(n: usize, rng: R) -> Self {
        random_tree::<Self, _>(n, rng).finalize()
    }
}

fn complete<G: WithBuilder>(n: usize) -> G::Builder {
    let mut b = G::builder(n, (n * n - n) / 2);
    for u in 0..n {
        for v in u + 1..n {
            b.add_edge(u, v);
        }
    }
    b
}

fn complete_binary_tree<G: WithBuilder>(height: u32) -> G::Builder {
    let num_vertices = 2usize.pow(height + 1) - 1;
    let mut b = G::builder(num_vertices, num_vertices - 1);
    for i in 0..2usize.pow(height) - 1 {
        b.add_edge(i, 2 * i + 1);
        b.add_edge(i, 2 * i + 2);
    }
    b
}

fn random_tree<G, R>(n: usize, mut rng: R) -> G::Builder
    where G: WithBuilder,
          R: Rng
{
    if n == 0 {
        return G::builder(0, 0);
    }
    let range = Range::new(0, n);
    let mut b = G::builder(n, n - 1);
    let mut visited = vec![false; n];
    let mut num_edges = 0;
    let mut u = range.ind_sample(&mut rng);
    visited[u] = true;
    while num_edges + 1 < n {
        let v = range.ind_sample(&mut rng);
        if visited[v] {
            u = v;
        } else {
            num_edges += 1;
            visited[v] = true;
            b.add_edge(u, v);
            u = v;
        }
    }
    b
}


// Tests

pub trait BuilderTests {
    type G: WithBuilder + VertexList + EdgeList;

    fn graph_macro() {
        let g: Self::G = graph!(
            5,
            (1, 2),
            (4, 0),
        );
        assert_eq!(5, g.num_vertices());
        assert_eq!(2, g.num_edges());
    }

    fn graph_prop_macro()
        where Self::G: WithEdgeProp<u32>
    {
        let (g, w): (Self::G, _) = graph!(
            5,
            (1, 2) -> 3,
            (4, 0) -> 4,
        );
        assert_eq!(5, g.num_vertices());
        assert_eq!(2, g.num_edges());
        let mut sum = 0;
        for e in g.edges() {
            sum += w[e];
        }
        assert_eq!(7, sum);
    }

    fn complete() {
        let (g, v, e) = complete::<Self::G>(3).finalize_();
        assert_eq!((v[0], v[1]), g.ends(e[0]));
        assert_eq!((v[0], v[2]), g.ends(e[1]));
        assert_eq!((v[1], v[2]), g.ends(e[2]));

        for (n, &m) in (0..5).zip(&[0, 0, 1, 3, 6, 10]) {
            let (g, v, _) = complete::<Self::G>(n).finalize_();
            assert_eq!(n, g.num_vertices());
            assert_eq!(m, g.num_edges());
            assert_eq!(set(v), set(g.vertices()));
        }
    }

    #[cfg_attr(feature = "cargo-clippy", allow(needless_range_loop))]
    fn complete_binary_tree()
        where Self::G: Incidence + WithVertexProp<Color>
    {
        let (g, _, _) = complete_binary_tree::<Self::G>(0).finalize_();
        assert_eq!(1, g.num_vertices());
        assert_eq!(0, g.num_edges());

        let (g, v, _) = complete_binary_tree::<Self::G>(1).finalize_();
        assert_eq!(3, g.num_vertices());
        assert_eq!(2, g.num_edges());
        assert_eq!(set(vec![(v[0], v[1]), (v[0], v[2])]),
                   set(g.out_edges_ends(v[0])));

        for h in 2..10 {
            let (g, v, _) = complete_binary_tree::<Self::G>(h).finalize_();
            assert!(g.is_tree());
            assert_eq!(2, g.out_degree(v[0]));
            for i in 1..g.num_vertices() / 2 - 1 {
                assert_eq!(3, g.out_degree(v[i]));
            }
            for i in (g.num_vertices() / 2)..g.num_vertices() {
                assert_eq!(1, g.out_degree(v[i]));
            }
        }
    }

    fn random_tree()
        where Self::G: Incidence + WithVertexProp<Color>
    {
        let mut rng = XorShiftRng::new_unseeded();
        for n in 0..100 {
            for _ in 0..10 {
                let g = Self::G::new_random_tree(n, &mut rng);
                assert_eq!(n, g.num_vertices());
                if n > 0 {
                    assert_eq!(n - 1, g.num_edges());
                }
                assert!(g.is_tree());
            }
        }
    }
}

#[macro_export]
macro_rules! graph_builder_tests {
    ($T: ident) => (
        delegate_tests!{
            $T,
            graph_macro,
            graph_prop_macro,
            complete,
            complete_binary_tree,
            random_tree
        }
    )
}
