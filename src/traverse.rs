use graph::*;
use std::collections::VecDeque;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Control {
    Continue,
    Break,
}

impl From<()> for Control {
    fn from(_: ()) -> Self {
        Control::Continue
    }
}

pub fn continue_if(x: bool) -> Control {
    if x { Control::Continue } else { Control::Break }
}

pub fn break_if(x: bool) -> Control {
    if x { Control::Break } else { Control::Continue }
}

// TODO: implement visitor for &mut Visitor
pub trait Visitor<G>
    where G: WithEdge
{
    fn discover_root_vertex(&mut self, _g: &G, _v: Vertex<G>) -> Control {
        Control::Continue
    }

    fn finish_root_vertex(&mut self, _g: &G, _v: Vertex<G>) -> Control {
        Control::Continue
    }

    fn discover_vertex(&mut self, _g: &G, _v: Vertex<G>) -> Control {
        Control::Continue
    }

    fn finish_vertex(&mut self, _g: &G, _v: Vertex<G>) -> Control {
        Control::Continue
    }

    fn discover_edge(&mut self, _g: &G, _e: Edge<G>) -> Control {
        Control::Continue
    }

    fn finish_edge(&mut self, _g: &G, _e: Edge<G>) -> Control {
        Control::Continue
    }

    fn discover_tree_edge(&mut self, _g: &G, _e: Edge<G>) -> Control {
        Control::Continue
    }

    fn finish_tree_edge(&mut self, _g: &G, _e: Edge<G>) -> Control {
        Control::Continue
    }

    fn discover_back_edge(&mut self, _g: &G, _e: Edge<G>) -> Control {
        Control::Continue
    }

    fn discover_cross_or_forward_edge(&mut self, _g: &G, _e: Edge<G>) -> Control {
        Control::Continue
    }
}

impl<'a, G, V> Visitor<G> for &'a mut V
    where G: WithEdge,
          V: Visitor<G>
{
    fn discover_root_vertex(&mut self, g: &G, v: Vertex<G>) -> Control {
        V::discover_root_vertex(self, g, v)
    }

    fn finish_root_vertex(&mut self, g: &G, v: Vertex<G>) -> Control {
        V::finish_root_vertex(self, g, v)
    }

    fn discover_vertex(&mut self, g: &G, v: Vertex<G>) -> Control {
        V::discover_vertex(self, g, v)
    }

    fn finish_vertex(&mut self, g: &G, v: Vertex<G>) -> Control {
        V::finish_vertex(self, g, v)
    }

    fn discover_edge(&mut self, g: &G, e: Edge<G>) -> Control {
        V::discover_edge(self, g, e)
    }

    fn finish_edge(&mut self, g: &G, e: Edge<G>) -> Control {
        V::finish_edge(self, g, e)
    }

    fn discover_tree_edge(&mut self, g: &G, e: Edge<G>) -> Control {
        V::discover_tree_edge(self, g, e)
    }

    fn finish_tree_edge(&mut self, g: &G, e: Edge<G>) -> Control {
        V::finish_tree_edge(self, g, e)
    }

    fn discover_back_edge(&mut self, g: &G, e: Edge<G>) -> Control {
        V::discover_back_edge(self, g, e)
    }

    fn discover_cross_or_forward_edge(&mut self, g: &G, e: Edge<G>) -> Control {
        V::discover_cross_or_forward_edge(self, g, e)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TraverseEvent<G: WithVertex + WithEdge> {
    DiscoverRootVertex(Vertex<G>),
    FinishRootVertex(Vertex<G>),
    DiscoverVertex(Vertex<G>),
    FinishVertex(Vertex<G>),
    DiscoverEdge(Edge<G>),
    FinishEdge(Edge<G>),
    DiscoverTreeEdge(Edge<G>),
    FinishTreeEdge(Edge<G>),
    DiscoverBackEdge(Edge<G>),
    DiscoverCrossOrForwardEdge(Edge<G>),
}

pub struct FnTraverseEvent<F>(F);

impl<G, F, R> Visitor<G> for FnTraverseEvent<F>
    where G: WithVertex + WithEdge,
          F: FnMut(TraverseEvent<G>) -> R,
          R: Into<Control>
{
    fn discover_root_vertex(&mut self, _g: &G, v: Vertex<G>) -> Control {
        (self.0)(TraverseEvent::DiscoverRootVertex(v)).into()
    }

    fn finish_root_vertex(&mut self, _g: &G, v: Vertex<G>) -> Control {
        (self.0)(TraverseEvent::FinishRootVertex(v)).into()
    }

    fn discover_vertex(&mut self, _g: &G, v: Vertex<G>) -> Control {
        (self.0)(TraverseEvent::DiscoverVertex(v)).into()
    }

    fn finish_vertex(&mut self, _g: &G, v: Vertex<G>) -> Control {
        (self.0)(TraverseEvent::FinishVertex(v)).into()
    }

    fn discover_edge(&mut self, _g: &G, e: Edge<G>) -> Control {
        (self.0)(TraverseEvent::DiscoverEdge(e)).into()
    }

    fn finish_edge(&mut self, _g: &G, e: Edge<G>) -> Control {
        (self.0)(TraverseEvent::FinishEdge(e)).into()
    }

    fn discover_tree_edge(&mut self, _g: &G, e: Edge<G>) -> Control {
        (self.0)(TraverseEvent::DiscoverTreeEdge(e)).into()
    }

    fn finish_tree_edge(&mut self, _g: &G, e: Edge<G>) -> Control {
        (self.0)(TraverseEvent::FinishTreeEdge(e)).into()
    }

    fn discover_back_edge(&mut self, _g: &G, e: Edge<G>) -> Control {
        (self.0)(TraverseEvent::DiscoverBackEdge(e)).into()
    }

    fn discover_cross_or_forward_edge(&mut self, _g: &G, e: Edge<G>) -> Control {
        (self.0)(TraverseEvent::DiscoverCrossOrForwardEdge(e)).into()
    }
}

pub fn recursive_dfs<G, V>(g: &G, mut vis: V)
    where G: WithVertexProp<Color> + WithEdge + VertexList + Incidence,
          V: Visitor<G>
{
    let mut color = g.default_vertex_prop(Color::White);
    for v in g.vertices() {
        if color[v] == Color::White {
            vis.discover_root_vertex(g, v);
            recursive_dfs_visit(g, None, v, &mut color, &mut vis);
            vis.finish_root_vertex(g, v);
        }
    }
}

fn recursive_dfs_visit<G, V>(g: &G,
                             from: Option<Edge<G>>,
                             u: Vertex<G>,
                             color: &mut DefaultVertexPropMut<G, Color>,
                             vis: &mut V)
    where G: WithVertexProp<Color> + WithEdge + Incidence,
          V: Visitor<G>
{
    vis.discover_vertex(g, u);
    color[u] = Color::Gray;
    for e in g.out_edges(u) {
        if Some(e) == from {
            continue;
        }
        let v = g.target(e);
        let color_v = color[v];
        vis.discover_edge(g, e);
        match color_v {
            Color::White => {
                vis.discover_tree_edge(g, e);
                recursive_dfs_visit(g, Some(e), v, color, vis);
                vis.finish_tree_edge(g, e);
            }
            Color::Gray => {
                vis.discover_back_edge(g, e);
            }
            Color::Black => {
                // Test if the graph is directed
                vis.discover_cross_or_forward_edge(g, e);
            }
        }
        vis.finish_edge(g, e);
    }
    color[u] = Color::Black;
    vis.finish_vertex(g, u);
}


pub struct DiscoverRootVertex<F>(pub F);

impl<G, F, R> Visitor<G> for DiscoverRootVertex<F>
    where G: Graph,
          F: FnMut(Vertex<G>) -> R,
          R: Into<Control>
{
    fn discover_root_vertex(&mut self, _g: &G, v: Vertex<G>) -> Control {
        (self.0)(v).into()
    }
}


pub struct DiscoverTreeEdge<F>(pub F);

impl<G, F, R> Visitor<G> for DiscoverTreeEdge<F>
    where G: Graph,
          F: FnMut(Edge<G>) -> R,
          R: Into<Control>
{
    fn discover_tree_edge(&mut self, _g: &G, e: Edge<G>) -> Control {
        (self.0)(e).into()
    }
}


pub struct DiscoverBackEdge<F>(pub F);

impl<G, F, R> Visitor<G> for DiscoverBackEdge<F>
    where G: Graph,
          F: FnMut(Edge<G>) -> R,
          R: Into<Control>
{
    fn discover_back_edge(&mut self, _g: &G, e: Edge<G>) -> Control {
        (self.0)(e).into()
    }
}

macro_rules! return_unless {
    ($e:expr) => (
        if $e == Control::Break  {
            return false;
        }
    )
}

macro_rules! break_unless {
    ($e:expr) => (
        if $e == Control::Break {
            break;
        }
    )
}


pub struct DiscoverFinishTime<G: Graph> {
    time: u64,
    pub discover: DefaultVertexPropMut<G, u64>,
    pub finish: DefaultVertexPropMut<G, u64>,
}

impl<G: Graph> DiscoverFinishTime<G> {
    pub fn new(g: &G) -> Self {
        DiscoverFinishTime {
            time: 0,
            discover: g.vertex_prop(0),
            finish: g.vertex_prop(0),
        }
    }

    pub fn is_ancestor_of(&self, ancestor: Vertex<G>, u: Vertex<G>) -> bool {
        self.discover[ancestor] <= self.discover[u] && self.finish[u] <= self.finish[ancestor]
    }
}

impl<G: Graph> Visitor<G> for DiscoverFinishTime<G> {
    fn discover_vertex(&mut self, _g: &G, v: Vertex<G>) -> Control {
        self.discover[v] = self.time;
        self.time += 1;
        Control::Continue
    }

    fn finish_vertex(&mut self, _g: &G, v: Vertex<G>) -> Control {
        self.finish[v] = self.time;
        self.time += 1;
        Control::Continue
    }
}


// TODO: Add Traverser::reset
// TODO: use a enum instead of true/false to stop/continue the traverse
// TODO: Create TraverserIter

pub trait Traverser<'a, G>
    where G: 'a + Graph
{
    fn graph(&self) -> &G;

    fn is_discovered(&self, v: Vertex<G>) -> bool;

    fn traverse<V: Visitor<G>>(&mut self, v: Vertex<G>, vis: V) -> bool;

    fn traverse_all<V: Visitor<G>>(&mut self, vis: V);

    fn traverse_vertices<I, V>(&mut self, vertices: I, mut vis: V)
        where I: IntoIterator<Item = Vertex<G>>,
              V: Visitor<G>
    {
        for v in vertices {
            if !self.is_discovered(v) {
                break_unless!(vis.discover_root_vertex(self.graph(), v));
                break_unless!(continue_if(self.traverse(v, &mut vis)));
            }
        }
    }

    // TODO: find_back_edge: write test
    fn find_back_edge(&mut self) -> Option<Edge<G>> {
        let mut back = None;
        self.traverse_all(DiscoverBackEdge(|e| {
            back = Some(e);
            Control::Break
        }));
        back
    }

    // TODO: find_parent: write test
    fn find_parent(&mut self) {
        let n = self.graph().num_vertices();
        let mut m = 0;
        self.traverse_all(DiscoverTreeEdge(|_| {
            m += 1;
            break_if(m + 1 == n)
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
    pub stack: Vec<(Vertex<G>, OutEdgeIter<'a, G>)>,
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
    fn traverse<V: Visitor<G>>(&mut self, v: Vertex<G>, mut vis: V) -> bool {
        self.stack.push((v, self.g.out_edges(v)));
        self.open(v);
        return_unless!(vis.discover_vertex(self.g, v));
        'out: while let Some((u, mut inc)) = self.stack.pop() {
            while let Some(e) = inc.next() {
                let v = self.g.target(e);
                if !self.is_discovered(v) {
                    self.open(v);
                    self.mark(e);
                    self.stack.push((u, inc));
                    self.stack.push((v, self.g.out_edges(v)));
                    return_unless!(vis.discover_tree_edge(self.g, e));
                    return_unless!(vis.discover_vertex(self.g, v));
                    continue 'out;
                } else if self.is_back(e) {
                    return_unless!(vis.discover_back_edge(self.g, e));
                }
                return_unless!(vis.finish_edge(self.g, e));
            }
            self.close(u);
            return_unless!(vis.finish_vertex(self.g, u));
            if let Some(e) = self.parent[u].into_option() {
                return_unless!(vis.finish_edge(self.g, self.g.reverse(e)));
            }
        }
        true
    }

    fn traverse_all<V: Visitor<G>>(&mut self, vis: V) {
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
    fn traverse<V: Visitor<G>>(&mut self, v: Vertex<G>, mut vis: V) -> bool {
        self.queue.push_back(v);
        self.open(v);
        while let Some(u) = self.queue.pop_front() {
            for e in self.g.out_edges(u) {
                let v = self.g.target(e);
                if !self.is_discovered(v) {
                    self.open(v);
                    self.mark(e);
                    self.queue.push_back(v);
                    return_unless!(vis.discover_tree_edge(self.g, e));
                } else if self.is_back(e) {
                    return_unless!(vis.discover_back_edge(self.g, e));
                }
            }
            self.close(u);
        }
        true
    }

    fn traverse_all<V: Visitor<G>>(&mut self, vis: V) {
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

    fn edge_by_ends(g: &StaticGraph,
                    u: Vertex<StaticGraph>,
                    v: Vertex<StaticGraph>)
                    -> Edge<StaticGraph> {
        for e in g.edges() {
            let (x, y) = g.ends(e);
            if u == x && v == y {
                return e;
            } else if u == y && v == x {
                return g.reverse(e);
            }
        }
        panic!()
    }

    #[test]
    fn test_recursive_dfs() {
        use super::TraverseEvent::*;
        let g = new();
        let v = g.vertices().into_vec();
        let e = |x: usize, y: usize| edge_by_ends(&g, v[x], v[y]);
        let mut v = vec![];
        recursive_dfs(&g, FnTraverseEvent(|evt| v.push(evt)));
        let expected = vec![
            DiscoverRootVertex(0),
            DiscoverVertex(0),
            DiscoverEdge(e(0, 1)),
            DiscoverTreeEdge(e(0, 1)),
            DiscoverVertex(1),
            DiscoverEdge(e(1, 2)),
            DiscoverTreeEdge(e(1, 2)),
            DiscoverVertex(2),
            DiscoverEdge(e(2, 0)),
            DiscoverBackEdge(e(2, 0)),
            FinishEdge(e(2, 0)),
            DiscoverEdge(e(2, 3)),
            DiscoverTreeEdge(e(2, 3)),
            DiscoverVertex(3),
            DiscoverEdge(e(3, 1)),
            DiscoverBackEdge(e(3, 1)),
            FinishEdge(e(3, 1)),
            FinishVertex(3),
            FinishTreeEdge(e(2, 3)),
            FinishEdge(e(2, 3)),
            FinishVertex(2),
            FinishTreeEdge(e(1, 2)),
            FinishEdge(e(1, 2)),
            DiscoverEdge(e(1, 3)),
            DiscoverCrossOrForwardEdge(e(1, 3)),
            FinishEdge(e(1, 3)),
            FinishVertex(1),
            FinishTreeEdge(e(0, 1)),
            FinishEdge(e(0, 1)),
            DiscoverEdge(e(0, 2)),
            DiscoverCrossOrForwardEdge(e(0, 2)),
            FinishEdge(e(0, 2)),
            FinishVertex(0),
            FinishRootVertex(0),

            DiscoverRootVertex(4),
            DiscoverVertex(4),
            DiscoverEdge(e(4, 5)),
            DiscoverTreeEdge(e(4, 5)),
            DiscoverVertex(5),
            DiscoverEdge(e(5, 6)),
            DiscoverTreeEdge(e(5, 6)),
            DiscoverVertex(6),
            DiscoverEdge(e(6, 4)),
            DiscoverBackEdge(e(6, 4)),
            FinishEdge(e(6, 4)),
            FinishVertex(6),
            FinishTreeEdge(e(5, 6)),
            FinishEdge(e(5, 6)),
            FinishVertex(5),
            FinishTreeEdge(e(4, 5)),
            FinishEdge(e(4, 5)),
            DiscoverEdge(e(4, 6)),
            DiscoverCrossOrForwardEdge(e(4, 6)),
            FinishEdge(e(4, 6)),
            FinishVertex(4),
            FinishRootVertex(4),
        ];
        for (a, b) in expected.into_iter().zip(v) {
            assert_eq!(a, b);
        }
    }

    const TREE: usize = 1;
    const BACK: usize = 2;

    struct TestVisitor<G>
        where G: Graph
    {
        parent: DefaultVertexPropMut<G, OptionEdge<G>>,
        depth: DefaultVertexPropMut<G, usize>,
        edge_type: DefaultEdgePropMut<G, usize>,
    }

    fn new_test_visitor(g: &StaticGraph) -> TestVisitor<StaticGraph> {
        TestVisitor {
            parent: g.vertex_prop(StaticGraph::edge_none()),
            depth: g.vertex_prop(0),
            edge_type: g.edge_prop(0),
        }
    }

    impl<G> Visitor<G> for TestVisitor<G>
        where G: Graph
    {
        fn discover_tree_edge(&mut self, g: &G, e: Edge<G>) -> Control {
            let (u, v) = g.ends(e);
            assert_eq!(0, self.edge_type[e]);
            self.parent[v] = g.reverse(e).into();
            self.depth[v] = self.depth[u] + 1;
            self.edge_type[e] = TREE;
            Control::Continue
        }

        fn discover_back_edge(&mut self, _: &G, e: Edge<G>) -> Control {
            assert_eq!(0, self.edge_type[e]);
            self.edge_type[e] = BACK;
            Control::Continue
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
        Dfs::new(&g).traverse_all(DiscoverRootVertex(|v| start.push(v)));
        let v = g.vertices().into_vec();
        assert_eq!(vec![v[0], v[4]], start);
    }

    #[test]
    fn dfs_tree_visitor() {
        let g = new();
        let mut edges = vec![];
        Dfs::new(&g).traverse_all(DiscoverTreeEdge(|e| {
            edges.push(e);
            continue_if(edges.len() != 2)
        }));
        let e = g.edges().into_vec();
        assert_eq!(vec![e[0], e[2]], edges);
    }

    #[test]
    fn dfs_back_visitor() {
        let g = new();
        let mut edges = vec![];
        Dfs::new(&g).traverse_all(DiscoverBackEdge(|e| {
            edges.push(e);
            continue_if(edges.len() != 2)
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
        Bfs::new(&g).traverse_all(DiscoverRootVertex(|v| start.push(v)));
        let v = g.vertices().into_vec();
        assert_eq!(vec![v[0], v[4]], start);
    }

    #[test]
    fn bfs_tree_visitor() {
        let g = new();
        let mut edges = vec![];
        Bfs::new(&g).traverse_all(DiscoverTreeEdge(|e| {
            edges.push(e);
            continue_if(edges.len() != 2)
        }));
        let e = g.edges().into_vec();
        assert_eq!(vec![e[0], e[1]], edges);
    }

    #[test]
    fn bfs_back_visitor() {
        let g = new();
        let mut edges = vec![];
        Bfs::new(&g).traverse_all(DiscoverBackEdge(|e| {
            edges.push(e);
            continue_if(edges.len() != 2)
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

    fn bench_dfs<'a>(b: &mut Bencher, g: &'a StaticGraph) {
        b.iter(|| {
            Dfs::new(g).traverse_all(DiscoverTreeEdge(|_| Control::Continue));
        });
    }

    fn bench_bfs<'a>(b: &mut Bencher, g: &'a StaticGraph) {
        b.iter(|| {
            Bfs::new(g).traverse_all(DiscoverTreeEdge(|_| Control::Continue));
        });
    }

    #[bench]
    fn bench_dfs_complete(b: &mut Bencher) {
        let g = StaticGraph::complete(100);
        bench_dfs(b, &g);
    }

    #[bench]
    fn bench_dfs_tree(b: &mut Bencher) {
        let g = StaticGraph::random_tree(100, &mut StdRng::from_seed(&[123]));
        bench_dfs(b, &g);
    }

    #[bench]
    fn bench_bfs_complete(b: &mut Bencher) {
        let g = StaticGraph::complete(100);
        bench_bfs(b, &g);
    }

    #[bench]
    fn bench_bfs_tree(b: &mut Bencher) {
        let g = StaticGraph::random_tree(100, &mut StdRng::from_seed(&[123]));
        bench_bfs(b, &g);
    }
}
