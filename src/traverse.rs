use graph::*;
use std::collections::VecDeque;

// Visitor

// TODO: implement visitor for &mut Visitor
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

impl<G, F> Visitor<G> for StartVertexVisitor<F>
    where G: Graph,
          F: FnMut(Vertex<G>) -> bool
{
    fn visit_start_vertex(&mut self, v: Vertex<G>) -> bool {
        self.0(v)
    }
}


pub struct TreeEdgeVisitor<F>(pub F);

impl<G, F> Visitor<G> for TreeEdgeVisitor<F>
    where G: Graph,
          F: FnMut(Edge<G>) -> bool
{
    fn visit_tree_edge(&mut self, e: Edge<G>) -> bool {
        self.0(e)
    }
}


pub struct BackEdgeVisitor<F>(pub F);

impl<G, F> Visitor<G> for BackEdgeVisitor<F>
    where G: Graph,
          F: FnMut(Edge<G>) -> bool
{
    fn visit_back_edge(&mut self, e: Edge<G>) -> bool {
        self.0(e)
    }
}

macro_rules! return_unless {
    ($e:expr) => (
        if !$e {
            return false;
        }
    )
}

macro_rules! break_unless {
    ($e:expr) => (
        if !$e {
            break;
        }
    )
}


// TODO: Add Traverser::reset
// TODO: use a enum instead of true/false to stop/continue the traverse
// TODO: Create TraverserIter

pub trait Traverser<'a, G>
    where G: 'a + Graph
{
    fn graph(&self) -> &G;

    fn is_discovered(&self, v: Vertex<G>) -> bool;

    fn traverse<V: Visitor<G>>(&mut self, v: Vertex<G>, vis: &mut V) -> bool;

    fn traverse_all<V: Visitor<G>>(&mut self, vis: &mut V);

    fn traverse_vertices<I, V>(&mut self, vertices: I, vis: &mut V)
        where I: IntoIterator<Item = Vertex<G>>,
              V: Visitor<G>
    {
        for v in vertices {
            if !self.is_discovered(v) {
                break_unless!(vis.visit_start_vertex(v));
                break_unless!(self.traverse(v, vis));
            }
        }
    }

    // TODO: find_back_edge: write test
    fn find_back_edge(&mut self) -> Option<Edge<G>> {
        let mut back = None;
        self.traverse_all(&mut BackEdgeVisitor(|e| {
            back = Some(e);
            false
        }));
        back
    }

    // TODO: find_parent: write test
    fn find_parent(&mut self) {
        let n = self.graph().num_vertices();
        let mut m = 0;
        self.traverse_all(&mut TreeEdgeVisitor(|_| {
            m += 1;
            m + 1 != n
        }));
    }
}

pub const WHITE: u8 = 0;
pub const GRAY: u8 = 1;
pub const BLACK: u8 = 2;


// Dfs

// TODO: create a builder (also for Bfs)
//     dfs(g)
//         .color(x)    // optional
//         .visited(x)  // optional
//         .stack(x)    // optional
//         .build()
// TODO: make stack generic
pub struct Dfs<'a, G, C = DefaultVertexPropMut<G, u8>, P = DefaultVertexPropMut<G, OptionEdge<G>>>
    where G: 'a + Graph,
          C: VertexPropMut<G, u8>,
          P: VertexPropMut<G, OptionEdge<G>>
{
    pub g: &'a G,
    pub color: C,
    pub parent: P,
    pub stack: Vec<(Vertex<G>, IncEdgeIter<'a, G>)>,
}

impl<'a, G> Dfs<'a, G>
    where G: 'a + Graph
{
    pub fn new(g: &'a G) -> Self {
        Dfs {
            g: g,
            color: g.vertex_prop(WHITE),
            parent: g.vertex_prop(G::edge_none()),
            stack: Vec::new(),
        }
    }
}

impl<'a, G, C, P> Traverser<'a, G> for Dfs<'a, G, C, P>
    where G: 'a + IncidenceGraph,
          C: VertexPropMut<G, u8>,
          P: VertexPropMut<G, OptionEdge<G>>
{
    fn traverse<V: Visitor<G>>(&mut self, v: Vertex<G>, vis: &mut V) -> bool {
        self.stack.push((v, self.g.inc_edges(v)));
        self.open(v);
        'out: while let Some((u, mut inc)) = self.stack.pop() {
            while let Some(e) = inc.next() {
                let v = self.g.target(e);
                if !self.is_discovered(v) {
                    self.open(v);
                    self.mark(e);
                    self.stack.push((u, inc));
                    self.stack.push((v, self.g.inc_edges(v)));
                    return_unless!(vis.visit_tree_edge(e));
                    continue 'out;
                } else if self.is_back(e) {
                    return_unless!(vis.visit_back_edge(e));
                }
            }
            self.close(u);
        }
        true
    }

    fn traverse_all<V: Visitor<G>>(&mut self, vis: &mut V) {
        self.traverse_vertices(self.g.vertices(), vis);
    }

    fn graph(&self) -> &G {
        self.g
    }

    fn is_discovered(&self, v: Vertex<G>) -> bool {
        self.color[v] != WHITE
    }
}

impl<'a, G, C, P> Dfs<'a, G, C, P>
    where G: 'a + Graph,
          C: VertexPropMut<G, u8>,
          P: VertexPropMut<G, OptionEdge<G>>
{
    fn open(&mut self, v: Vertex<G>) {
        self.color[v] = GRAY;
    }

    fn close(&mut self, v: Vertex<G>) {
        self.color[v] = BLACK;
    }

    fn mark(&mut self, e: Edge<G>) {
        self.parent[self.g.target(e)] = self.g.reverse(e).into();
    }

    fn is_back(&self, e: Edge<G>) -> bool {
        let (u, v) = self.g.ends(e);
        self.color[v] == GRAY && self.parent[u] != e.into()
    }
}

// Bfs

// TODO: make queue generic
pub struct Bfs<'a, G, C = DefaultVertexPropMut<G, u8>, P = DefaultVertexPropMut<G, OptionEdge<G>>>
    where G: 'a + Graph,
          C: VertexPropMut<G, u8>,
          P: VertexPropMut<G, OptionEdge<G>>
{
    pub g: &'a G,
    pub color: C,
    pub parent: P,
    pub queue: VecDeque<Vertex<G>>,
}

impl<'a, G> Bfs<'a, G>
    where G: 'a + Graph
{
    pub fn new(g: &'a G) -> Self {
        Bfs {
            g: g,
            color: g.vertex_prop(WHITE),
            parent: g.vertex_prop(G::edge_none()),
            queue: VecDeque::new(),
        }
    }
}

impl<'a, G, C, P> Traverser<'a, G> for Bfs<'a, G, C, P>
    where G: 'a + IncidenceGraph,
          C: VertexPropMut<G, u8>,
          P: VertexPropMut<G, OptionEdge<G>>
{
    fn traverse<V: Visitor<G>>(&mut self, v: Vertex<G>, vis: &mut V) -> bool {
        self.queue.push_back(v);
        self.open(v);
        while let Some(u) = self.queue.pop_front() {
            for e in self.g.inc_edges(u) {
                let v = self.g.target(e);
                if !self.is_discovered(v) {
                    self.open(v);
                    self.mark(e);
                    self.queue.push_back(v);
                    return_unless!(vis.visit_tree_edge(e));
                } else if self.is_back(e) {
                    return_unless!(vis.visit_back_edge(e));
                }
            }
            self.close(u);
        }
        true
    }

    fn traverse_all<V: Visitor<G>>(&mut self, vis: &mut V) {
        self.traverse_vertices(self.g.vertices(), vis);
    }

    fn graph(&self) -> &G {
        self.g
    }

    fn is_discovered(&self, v: Vertex<G>) -> bool {
        self.color[v] != WHITE
    }
}

impl<'a, G, C, P> Bfs<'a, G, C, P>
    where G: 'a + Graph,
          C: VertexPropMut<G, u8>,
          P: VertexPropMut<G, OptionEdge<G>>
{
    fn open(&mut self, v: Vertex<G>) {
        self.color[v] = GRAY;
    }

    fn close(&mut self, v: Vertex<G>) {
        self.color[v] = BLACK;
    }

    fn mark(&mut self, e: Edge<G>) {
        self.parent[self.g.target(e)] = self.g.reverse(e).into();
    }

    fn is_back(&self, e: Edge<G>) -> bool {
        let (u, v) = self.g.ends(e);
        self.color[v] == GRAY && self.parent[u] != e.into()
    }
}


// Dfs parent

// TODO: write tests
// TODO: decide an appropriated place and name
pub trait DfsParent: IncidenceGraph {
    fn dfs_parent(&self) -> DefaultVertexPropMut<Self, OptionEdge<Self>> {
        let mut dfs = Dfs::new(self);
        dfs.find_parent();
        dfs.parent
    }
}

impl<G> DfsParent for G where G: IncidenceGraph {}


// Tests

#[cfg(test)]
mod tests {
    use graph::*;
    use static_::*;
    use fera::IteratorExt;
    use traverse::*;

    fn new() -> StaticGraph {
        // u -> e (u, v)
        // 0 -> 0 (0, 1) 1 (0, 2)
        // 1 -> 1 (1, 0) 2 (1, 2) 3 (1, 3)
        // 2 -> 1 (2, 0) 2 (2, 1) 4 (2, 3)
        // 3 -> 3 (3, 1) 4 (3, 2)
        //
        // 4 -> 5 (4, 5) 6 (4, 6)
        // 5 -> 5 (5, 4) 7 (5, 6)
        // 6 -> 7 (6, 5) 6 (6, 4)
        graph!(StaticGraph,
               7,
               (0, 1),
               (0, 2),
               (1, 2),
               (1, 3),
               (2, 3),
               (4, 5),
               (4, 6),
               (5, 6))
    }

    const TREE: usize = 1;
    const BACK: usize = 2;

    struct TestVisitor<'a, G>
        where G: 'a + Graph
    {
        g: &'a G,
        parent: DefaultVertexPropMut<G, OptionEdge<G>>,
        depth: DefaultVertexPropMut<G, usize>,
        edge_type: DefaultEdgePropMut<G, usize>,
    }

    fn new_test_visitor(g: &StaticGraph) -> TestVisitor<StaticGraph> {
        TestVisitor {
            g: g,
            parent: g.vertex_prop(StaticGraph::edge_none()),
            depth: g.vertex_prop(0),
            edge_type: g.edge_prop(0),
        }
    }

    impl<'a, G> Visitor<G> for TestVisitor<'a, G>
        where G: 'a + Graph
    {
        fn visit_tree_edge(&mut self, e: Edge<G>) -> bool {
            let (u, v) = self.g.ends(e);
            assert_eq!(0, self.edge_type[e]);
            self.parent[v] = self.g.reverse(e).into();
            self.depth[v] = self.depth[u] + 1;
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
        let mut dfs = Dfs::new(&g);
        dfs.traverse_all(&mut vis);

        assert_eq!(g.vertices().map(|v| vis.parent[v].into_option()).into_vec(),
                   g.vertices().map(|v| dfs.parent[v].into_option()).into_vec());

        assert_eq!(vec![None, Some(0), Some(1), Some(2), None, Some(4), Some(5)],
                   g.vertices()
                       .map(|v| vis.parent[v].into_option().map(|e| g.target(e)))
                       .into_vec());

        assert_eq!(vec![0, 1, 2, 3, 0, 1, 2],
                   g.vertices().map(|v| vis.depth[v]).into_vec());

        assert_eq!(vec![TREE, BACK, TREE, BACK, TREE, TREE, BACK, TREE],
                   g.edges().map(|e| vis.edge_type[e]).into_vec());
    }

    #[test]
    fn dfs_start_visitor() {
        let g = new();
        let mut start = vec![];
        Dfs::new(&g).traverse_all(&mut StartVertexVisitor(|v| {
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
        Dfs::new(&g).traverse_all(&mut TreeEdgeVisitor(|e| {
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
        Dfs::new(&g).traverse_all(&mut BackEdgeVisitor(|e| {
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
        let mut bfs = Bfs::new(&g);
        bfs.traverse_all(&mut vis);

        assert_eq!(g.vertices().map(|v| vis.parent[v].into_option()).into_vec(),
                   g.vertices().map(|v| bfs.parent[v].into_option()).into_vec());

        assert_eq!(vec![None, Some(0), Some(0), Some(1), None, Some(4), Some(4)],
                   g.vertices()
                       .map(|v| vis.parent[v].into_option().map(|e| g.target(e)))
                       .into_vec());

        assert_eq!(vec![0, 1, 1, 2, 0, 1, 1],
                   g.vertices().map(|v| vis.depth[v]).into_vec());

        assert_eq!(vec![TREE, TREE, BACK, TREE, BACK, TREE, TREE, BACK],
                   g.edges().map(|e| vis.edge_type[e]).into_vec());
    }

    #[test]
    fn bfs_start_visitor() {
        let g = new();
        let mut start = vec![];
        Bfs::new(&g).traverse_all(&mut StartVertexVisitor(|v| {
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
        Bfs::new(&g).traverse_all(&mut TreeEdgeVisitor(|e| {
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
        Bfs::new(&g).traverse_all(&mut BackEdgeVisitor(|e| {
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
            T::new(g).traverse_all(&mut TreeEdgeVisitor(|_| true));
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
