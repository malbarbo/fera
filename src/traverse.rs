use graph::*;
use std::collections::VecDeque;

// Visitor

pub trait Visitor<G>
    where G: Graph
{
    fn visit_start_vertex(&mut self, _v: Vertex<G>) -> bool {
        true
    }

    fn visit_tree_edge(&mut self, _e: Edge<G>) -> bool {
        true
    }

    fn visit_back_edge(&mut self, _e: Edge<G>) -> bool {
        true
    }
}

pub struct StartVertexVisitor<F>(pub F);
pub struct TreeEdgeVisitor<F>(pub F);
pub struct BackEdgeVisitor<F>(pub F);

impl<G, F> Visitor<G> for StartVertexVisitor<F>
    where G: Graph,
          F: FnMut(Vertex<G>) -> bool
{
    fn visit_start_vertex(&mut self, v: Vertex<G>) -> bool {
        self.0(v)
    }
}

impl<G, F> Visitor<G> for TreeEdgeVisitor<F>
    where G: Graph,
          F: FnMut(Edge<G>) -> bool
{
    fn visit_tree_edge(&mut self, e: Edge<G>) -> bool {
        self.0(e)
    }
}

impl<G, F> Visitor<G> for BackEdgeVisitor<F>
    where G: Graph,
          F: FnMut(Edge<G>) -> bool
{
    fn visit_back_edge(&mut self, e: Edge<G>) -> bool {
        self.0(e)
    }
}

// TODO: change if_false to unless?

macro_rules! return_if_false {
    ($e:expr) => (
        if !$e {
            return false;
        }
    )
}

macro_rules! break_if_false {
    ($e:expr) => (
        if !$e {
            break;
        }
    )
}


// TODO: Allow a Traverser to be reused

// Traversers

pub trait Traverser<'a, G>: Sized
    where G: 'a + Graph,
{
    fn new(g: &'a G) -> Self;

    fn is_discovered(&mut self, v: Vertex<G>) -> bool;

    fn traverse<V: Visitor<G>>(&mut self, v: Vertex<G>, vis: &mut V) -> bool;

    // TODO: should this return the visitor?
    fn run<V: Visitor<G>>(g: &'a G, vis: &mut V) {
        Self::run_with_vertices(g, g.vertices(), vis);
    }

    fn run_with_vertices<I, V>(g: &'a G, vertices: I, vis: &mut V)
        where I: Iterator<Item = Vertex<G>>,
              V: Visitor<G>
    {
        let mut t = Self::new(g);
        for v in vertices {
            if !t.is_discovered(v) {
                break_if_false!(vis.visit_start_vertex(v));
                break_if_false!(t.traverse(v, vis));
            }
        }
    }

    fn run_start<V: Visitor<G>>(g: &'a G, v: Vertex<G>, vis: &mut V) {
        Self::new(g).traverse(v, vis);
    }
}

use std;
const WHITE: usize = std::usize::MAX;
const BLACK: usize = std::usize::MAX - 1;

pub struct State<'a, G>
    where G: 'a + Graph,
{
    g: &'a G,
    // depth if opened, color if closed
    depth: DefaultPropMutVertex<G, usize>,
}

impl<'a, G> State<'a, G>
    where G: 'a + Graph,
{
    fn new(g: &'a G) -> Self {
        State {
            g: g,
            depth: g.vertex_prop(WHITE),
        }
    }

    fn open(&mut self, v: Vertex<G>) {
        self.depth[v] = 0;
    }

    fn mark(&mut self, u: Vertex<G>, v: Vertex<G>) {
        self.depth[v] = self.depth[u] + 1;
    }

    fn close(&mut self, v: Vertex<G>) {
        self.depth[v] = BLACK;
    }

    fn is_back(&self, v: Vertex<G>, parent: Vertex<G>) -> bool {
        self.depth[parent] < BLACK && self.depth[v] != self.depth[parent] + 1
    }

    fn is_discovered(&mut self, v: Vertex<G>) -> bool {
        self.depth[v] != WHITE
    }
}


// Dfs

pub struct Dfs<'a, G>(State<'a, G>)
    where G: 'a + Graph;

impl<'a, G> Traverser<'a, G> for Dfs<'a, G>
    where G: 'a + Graph,
{
    fn new(g: &'a G) -> Self {
        Dfs(State::new(g))
    }

    fn is_discovered(&mut self, v: Vertex<G>) -> bool {
        self.0.is_discovered(v)
    }

    fn traverse<V: Visitor<G>>(&mut self, v: Vertex<G>, vis: &mut V) -> bool {
        let s = &mut self.0;
        // TODO: which initial capacity?
        let mut stack = Vec::with_capacity(self.0.g.num_edges() / 2);
        stack.push((v, s.g.inc_edges(v)));
        s.open(v);
        'out: while let Some((u, mut inc)) = stack.pop() {
            while let Some(e) = inc.next() {
                let v = s.g.target(e);
                if !s.is_discovered(v) {
                    return_if_false!(vis.visit_tree_edge(e));
                    s.mark(u, v);
                    stack.push((u, inc));
                    stack.push((v, s.g.inc_edges(v)));
                    continue 'out;
                } else if s.is_back(u, v) {
                    return_if_false!(vis.visit_back_edge(e));
                }
            }
            s.close(u);
        }
        true
    }
}


// Bfs

pub struct Bfs<'a, G>(State<'a, G>)
    where G: 'a + Graph;

impl<'a, G> Traverser<'a, G> for Bfs<'a, G>
    where G: 'a + Graph,
{
    fn new(g: &'a G) -> Self {
        Bfs(State::new(g))
    }

    fn is_discovered(&mut self, v: Vertex<G>) -> bool {
        self.0.is_discovered(v)
    }

    fn traverse<V: Visitor<G>>(&mut self, v: Vertex<G>, vis: &mut V) -> bool {
        let mut s = &mut self.0;
        // TODO: which initial capacity?
        let mut queue = VecDeque::with_capacity(self.0.g.num_vertices() / 2);
        queue.push_back(v);
        s.open(v);
        while let Some(u) = queue.pop_front() {
            for e in s.g.inc_edges(u) {
                let v = s.g.target(e);
                if !s.is_discovered(v) {
                    return_if_false!(vis.visit_tree_edge(e));
                    s.mark(u, v);
                    queue.push_back(v);
                } else if s.is_back(u, v) {
                    return_if_false!(vis.visit_back_edge(e));
                }
            }
            s.close(u);
        }
        true
    }
}


// Dfs parent

// TODO: write test
pub trait DfsParent: Graph {
    fn dfs_parent(&self) -> DefaultPropMutVertex<Self, OptionEdge<Self>> {
        let mut parent = self.vertex_prop(Self::edge_none());
        let mut num_edges = 0;
        Dfs::run(self,
                 &mut TreeEdgeVisitor(|e| {
                     parent[self.target(e)] = Self::edge_some(self.reverse(e));
                     num_edges += 1;
                     num_edges + 1 != self.num_vertices()
                 }));
        parent
    }
}

impl<G> DfsParent for G where G: Graph { }


// Tests

#[cfg(test)]
mod tests {
    use graph::*;
    use static_::*;
    use fera::IteratorExt;
    use traverse::*;

    fn new() -> StaticGraph {
        graph!(
            StaticGraph,
            7,
            (0, 1), (0, 2), (1, 2), (1, 3), (2, 3), (4, 5), (4, 6), (5, 6)
        )
        // u -> e (u, v)
        // 0 -> 0 (0, 1) 1 (0, 2)
        // 1 -> 1 (1, 0) 2 (1, 2) 3 (1, 3)
        // 2 -> 1 (2, 0) 2 (2, 1) 4 (2, 3)
        // 3 -> 3 (3, 1) 4 (3, 2)
        //
        // 4 -> 5 (4, 5) 6 (4, 6)
        // 5 -> 5 (5, 4) 7 (5, 6)
        // 6 -> 7 (6, 5) 6 (6, 4)
    }

    const TREE: usize = 1;
    const BACK: usize = 2;

    struct TestVisitor<'a, G>
        where G: 'a + Graph,
    {
        g: &'a G,
        parent: DefaultPropMutVertex<G, OptionVertex<G>>,
        d: DefaultPropMutVertex<G, usize>,
        edge_type: DefaultPropMutEdge<G, usize>,
    }

    fn new_test_visitor(g: &StaticGraph) -> TestVisitor<StaticGraph> {
        TestVisitor {
            g: g,
            parent: g.vertex_prop(StaticGraph::vertex_none()),
            d: g.vertex_prop(0),
            edge_type: g.edge_prop(0),
        }
    }

    impl<'a, G> Visitor<G> for TestVisitor<'a, G>
        where G: 'a + Graph,
    {
        fn visit_tree_edge(&mut self, e: Edge<G>) -> bool {
            assert_eq!(0, self.edge_type[e]);
            self.parent[self.g.target(e)] = G::vertex_some(self.g.source(e));
            self.d[self.g.target(e)] = self.d[self.g.source(e)] + 1;
            self.edge_type[e] = TREE;
            true
        }

        fn visit_back_edge(&mut self, e: Edge<G>) -> bool {
            assert_eq!(0, self.edge_type[e]);
            self.edge_type[e] = BACK;
            true
        }
    }

    #[test]
    fn dfs() {
        let g = new();
        let mut vis = new_test_visitor(&g);
        Dfs::run(&g, &mut vis);

        assert_eq!(vec![None, Some(0), Some(1), Some(2), None, Some(4), Some(5)],
                   vis.parent.to_vec().into_iter().map(|v| v.to_option()).into_vec());

        assert_eq!(vec![0, 1, 2, 3, 0, 1, 2], vis.d.to_vec());

        assert_eq!(vec![TREE, BACK, TREE, BACK, TREE, TREE, BACK, TREE],
                   vis.edge_type.to_vec());
    }

    #[test]
    fn dfs_start_visitor() {
        let g = new();
        let mut start = vec![];
        Dfs::run(&g,
                 &mut StartVertexVisitor(|v| {
                     start.push(v);
                     true
                 }));
        let v = g.vertices().into_vec();
        assert_eq!(vec![v[0], v[4]], start);
    }

    #[test]
    fn dfs_tree_visitor() {
        let g = new();
        let mut edges = vec![];
        Dfs::run(&g,
                 &mut TreeEdgeVisitor(|e| {
                     edges.push(e);
                     edges.len() != 2
                 }));
        let e = g.edges().into_vec();
        assert_eq!(vec![e[0], e[2]], edges);
    }

    #[test]
    fn dfs_back_visitor() {
        let g = new();
        let mut edges = vec![];
        Dfs::run(&g,
                 &mut BackEdgeVisitor(|e| {
                     edges.push(e);
                     edges.len() != 2
                 }));
        let e = g.edges().into_vec();
        assert_eq!(vec![e[1], e[3]], edges);
    }

    #[test]
    fn bfs() {
        let g = new();
        let mut vis = new_test_visitor(&g);
        Bfs::run(&g, &mut vis);

        assert_eq!(vec![None, Some(0), Some(0), Some(1), None, Some(4), Some(4)],
                   vis.parent.to_vec().into_iter().map(|v| v.to_option()).into_vec());

        assert_eq!(vec![0, 1, 1, 2, 0, 1, 1], vis.d.to_vec());

        assert_eq!(vec![TREE, TREE, BACK, TREE, BACK, TREE, TREE, BACK],
                   vis.edge_type.to_vec());
    }

    #[test]
    fn bfs_start_visitor() {
        let g = new();
        let mut start = vec![];
        Bfs::run(&g,
                 &mut StartVertexVisitor(|v| {
                     start.push(v);
                     true
                 }));
        let v = g.vertices().into_vec();
        assert_eq!(vec![v[0], v[4]], start);
    }

    #[test]
    fn bfs_tree_visitor() {
        let g = new();
        let mut edges = vec![];
        Bfs::run(&g,
                 &mut TreeEdgeVisitor(|e| {
                     edges.push(e);
                     edges.len() != 2
                 }));
        let e = g.edges().into_vec();
        assert_eq!(vec![e[0], e[1]], edges);
    }

    #[test]
    fn bfs_back_visitor() {
        let g = new();
        let mut edges = vec![];
        Bfs::run(&g,
                 &mut BackEdgeVisitor(|e| {
                     edges.push(e);
                     edges.len() != 2
                 }));
        let e = g.edges().into_vec();
        assert_eq!(vec![e[2], e[4]], edges);
    }
}

#[cfg(all(feature = "nightly", test))]
mod benchs {
    use static_::*;
    use builder::WithBuilder;
    use traverse::*;
    use rand::{SeedableRng, StdRng};
    use test::Bencher;

    fn bench_traverser<'a, T>(b: &mut Bencher, g: &'a StaticGraph)
        where T: Traverser<'a, StaticGraph>
    {
        b.iter(|| {
            T::run(g, &mut TreeEdgeVisitor(|_| true));
        });
    }

    #[bench]
    fn bench_dfs_complete(b: &mut Bencher) {
        let g = StaticGraph::complete(100);
        bench_traverser::<Dfs<_>>(b, &g);
    }

    #[bench]
    fn bench_dfs_tree(b: &mut Bencher) {
        let g = StaticGraph::tree(100, &mut StdRng::from_seed(&[123]));
        bench_traverser::<Dfs<_>>(b, &g);
    }

    #[bench]
    fn bench_bfs_complete(b: &mut Bencher) {
        let g = StaticGraph::complete(100);
        bench_traverser::<Bfs<_>>(b, &g);
    }

    #[bench]
    fn bench_bfs_tree(b: &mut Bencher) {
        let g = StaticGraph::tree(100, &mut StdRng::from_seed(&[123]));
        bench_traverser::<Bfs<_>>(b, &g);
    }
}
