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

pub fn continue_if(cond: bool) -> Control {
    if cond {
        Control::Continue
    } else {
        Control::Break
    }
}

pub fn break_if(cond: bool) -> Control {
    if cond {
        Control::Break
    } else {
        Control::Continue
    }
}

// TODO: check if event names make sense for both dfs and bfs
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
            recursive_dfs_visit(g, G::edge_none(), v, &mut color, &mut vis);
            vis.finish_root_vertex(g, v);
        }
    }
}

fn recursive_dfs_visit<G, C, V>(g: &G,
                                from: OptionEdge<G>,
                                u: Vertex<G>,
                                color: &mut C,
                                vis: &mut V)
    where G: WithEdge + Incidence,
          C: VertexPropMut<G, Color>,
          V: Visitor<G>
{
    color[u] = Color::Gray;
    vis.discover_vertex(g, u);
    for e in g.out_edges(u) {
        let v = g.target(e);
        if g.is_undirected_edge(e) && color[v] == Color::Black || G::edge_some(e) == from {
            continue;
        }
        vis.discover_edge(g, e);
        match color[v] {
            Color::White => {
                vis.discover_tree_edge(g, e);
                recursive_dfs_visit(g, e.into(), v, color, vis);
                vis.finish_tree_edge(g, e);
            }
            Color::Gray => {
                vis.discover_back_edge(g, e);
            }
            Color::Black => {
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
    where G: WithEdge,
          F: FnMut(Vertex<G>) -> R,
          R: Into<Control>
{
    fn discover_root_vertex(&mut self, _g: &G, v: Vertex<G>) -> Control {
        (self.0)(v).into()
    }
}


pub struct DiscoverTreeEdge<F>(pub F);

impl<G, F, R> Visitor<G> for DiscoverTreeEdge<F>
    where G: WithEdge,
          F: FnMut(Edge<G>) -> R,
          R: Into<Control>
{
    fn discover_tree_edge(&mut self, _g: &G, e: Edge<G>) -> Control {
        (self.0)(e).into()
    }
}


pub struct DiscoverBackEdge<F>(pub F);

impl<G, F, R> Visitor<G> for DiscoverBackEdge<F>
    where G: WithEdge,
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


pub trait Traverser<'a, G>
    where G: 'a + Incidence
{
    fn graph(&self) -> &G;

    fn is_discovered(&self, v: Vertex<G>) -> bool;

    fn traverse<V: Visitor<G>>(&mut self, v: Vertex<G>, vis: V) -> bool;

    fn traverse_all<V: Visitor<G>>(&mut self, vis: V) where G: VertexList;

    fn traverse_vertices<I, V>(&mut self, vertices: I, mut vis: V)
        where I: IntoIterator<Item = Vertex<G>>,
              V: Visitor<G>
    {
        for v in vertices {
            if !self.is_discovered(v) {
                break_unless!(vis.discover_root_vertex(self.graph(), v));
                break_unless!(continue_if(self.traverse(v, &mut vis)));
                break_unless!(vis.finish_root_vertex(self.graph(), v));
            }
        }
    }
}


// Dfs

pub struct Dfs<'a, G, C>
    where G: 'a + Incidence,
          C: VertexPropMut<G, Color>
{
    pub g: &'a G,
    pub color: C,
    pub stack: Vec<(OptionEdge<G>, Vertex<G>, OutEdgeIter<'a, G>)>,
}

impl<'a, G> Dfs<'a, G, DefaultVertexPropMut<G, Color>>
    where G: 'a + Incidence + WithVertexProp<Color>
{
    pub fn new(g: &'a G) -> Self {
        Dfs {
            g: g,
            color: g.vertex_prop(Color::White),
            stack: Vec::new(),
        }
    }
}

impl<'a, G, C> Traverser<'a, G> for Dfs<'a, G, C>
    where G: 'a + Incidence,
          C: VertexPropMut<G, Color>
{
    fn traverse<V: Visitor<G>>(&mut self, v: Vertex<G>, mut vis: V) -> bool {
        self.stack.push((G::edge_none(), v, self.g.out_edges(v)));
        self.color[v] = Color::Gray;
        return_unless!(vis.discover_vertex(self.g, v));
        'out: while let Some((from, u, mut inc)) = self.stack.pop() {
            while let Some(e) = inc.next() {
                let v = self.g.target(e);
                if self.g.is_undirected_edge(e) && self.color[v] == Color::Black ||
                   G::edge_some(e) == from {
                    continue;
                }
                return_unless!(vis.discover_edge(self.g, e));
                match self.color[v] {
                    Color::White => {
                        self.color[v] = Color::Gray;
                        self.stack.push((from, u, inc));
                        self.stack.push((e.into(), v, self.g.out_edges(v)));
                        return_unless!(vis.discover_tree_edge(self.g, e));
                        return_unless!(vis.discover_vertex(self.g, v));
                        continue 'out;
                    }
                    Color::Gray => {
                        return_unless!(vis.discover_back_edge(self.g, e));
                    }
                    Color::Black => {
                        return_unless!(vis.discover_cross_or_forward_edge(self.g, e));
                    }
                }
                return_unless!(vis.finish_edge(self.g, e));
            }
            self.color[u] = Color::Black;
            return_unless!(vis.finish_vertex(self.g, u));
            if let Some(from) = from.into_option() {
                return_unless!(vis.finish_tree_edge(self.g, from));
                return_unless!(vis.finish_edge(self.g, from));
            }
        }
        true
    }

    fn traverse_all<V: Visitor<G>>(&mut self, vis: V)
        where G: VertexList
    {
        self.traverse_vertices(self.g.vertices(), vis);
    }

    fn graph(&self) -> &G {
        self.g
    }

    fn is_discovered(&self, v: Vertex<G>) -> bool {
        self.color[v] != Color::White
    }
}


// Bfs

// TODO: make queue generic
pub struct Bfs<'a, G, C>
    where G: 'a + WithEdge,
          C: VertexPropMut<G, Color>
{
    pub g: &'a G,
    pub color: C,
    pub queue: VecDeque<(OptionEdge<G>, Vertex<G>)>,
}

impl<'a, G> Bfs<'a, G, DefaultVertexPropMut<G, Color>>
    where G: 'a + WithEdge + WithVertexProp<Color>
{
    pub fn new(g: &'a G) -> Self {
        Bfs {
            g: g,
            color: g.vertex_prop(Color::White),
            queue: VecDeque::new(),
        }
    }
}

impl<'a, G, C> Traverser<'a, G> for Bfs<'a, G, C>
    where G: 'a + IncidenceGraph,
          C: VertexPropMut<G, Color>
{
    fn traverse<V: Visitor<G>>(&mut self, v: Vertex<G>, mut vis: V) -> bool {
        self.queue.push_back((G::edge_none(), v));
        self.color[v] = Color::Gray;
        return_unless!(vis.discover_vertex(self.g, v));
        while let Some((from, u)) = self.queue.pop_front() {
            for e in self.g.out_edges(u) {
                let v = self.g.target(e);
                if self.g.is_undirected_edge(e) && self.color[v] == Color::Black ||
                   G::edge_some(e) == from {
                    continue;
                }
                return_unless!(vis.discover_edge(self.g, e));
                match self.color[v] {
                    Color::White => {
                        self.color[v] = Color::Gray;
                        self.queue.push_back((e.into(), v));
                        return_unless!(vis.discover_tree_edge(self.g, e));
                        return_unless!(vis.discover_vertex(self.g, v));
                        continue;
                    }
                    Color::Gray => {
                        return_unless!(vis.discover_back_edge(self.g, e));
                    }
                    Color::Black => {
                        return_unless!(vis.discover_cross_or_forward_edge(self.g, e));
                    }
                }
                return_unless!(vis.finish_edge(self.g, e));
            }
            self.color[u] = Color::Black;
            return_unless!(vis.finish_vertex(self.g, u));
            if let Some(from) = from.into_option() {
                return_unless!(vis.finish_tree_edge(self.g, from));
                return_unless!(vis.finish_edge(self.g, from));
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
        self.color[v] != Color::White
    }
}


// Dfs parent

// TODO: write tests
// TODO: decide an appropriated place and name
pub trait DfsParent: IncidenceGraph {
    fn dfs_parent(&self) -> DefaultVertexPropMut<Self, OptionEdge<Self>> {
        unimplemented!()
        // let mut dfs = Dfs::new(self);
        // dfs.find_parent();
        // dfs.parent
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
        //    1
        //  / | \         4
        // 0  |  3      /   \
        //  \ | /      5 --- 6
        //    2
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
    fn dfs() {
        use super::TraverseEvent::*;
        let g = new();
        let v = g.vertices().into_vec();
        let e = |x: usize, y: usize| edge_by_ends(&g, v[x], v[y]);
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
            FinishVertex(1),
            FinishTreeEdge(e(0, 1)),
            FinishEdge(e(0, 1)),
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
            FinishVertex(4),
            FinishRootVertex(4),
        ];

        let mut v = vec![];
        recursive_dfs(&g, FnTraverseEvent(|evt| v.push(evt)));
        assert_eq!(expected, v);

        v.clear();
        Dfs::new(&g).traverse_all(FnTraverseEvent(|evt| v.push(evt)));
        assert_eq!(expected, v);

        // TODO: test recursive dfs vs dfs form random graphs
        // TODO: test each edge and vertex is visited exatly once
    }

    #[test]
    fn bfs() {
        use super::TraverseEvent::*;
        let g = new();
        let v = g.vertices().into_vec();
        let e = |x: usize, y: usize| edge_by_ends(&g, v[x], v[y]);
        let expected = vec![
            DiscoverRootVertex(0),
            DiscoverVertex(0),
            DiscoverEdge(e(0, 1)),
            DiscoverTreeEdge(e(0, 1)),
            DiscoverVertex(1),
            DiscoverEdge(e(0, 2)),
            DiscoverTreeEdge(e(0, 2)),
            DiscoverVertex(2),
            FinishVertex(0),
            DiscoverEdge(e(1, 2)),
            DiscoverBackEdge(e(1, 2)),
            FinishEdge(e(1, 2)),
            DiscoverEdge(e(1, 3)),
            DiscoverTreeEdge(e(1, 3)),
            DiscoverVertex(3),
            FinishVertex(1),
            FinishTreeEdge(e(0, 1)),
            FinishEdge(e(0, 1)),
            DiscoverEdge(e(2, 3)),
            DiscoverBackEdge(e(2, 3)),
            FinishEdge(e(2, 3)),
            FinishVertex(2),
            FinishTreeEdge(e(0, 2)),
            FinishEdge(e(0, 2)),
            FinishVertex(3),
            FinishTreeEdge(e(1, 3)),
            FinishEdge(e(1, 3)),
            FinishRootVertex(0),

            DiscoverRootVertex(4),
            DiscoverVertex(4),
            DiscoverEdge(e(4, 5)),
            DiscoverTreeEdge(e(4, 5)),
            DiscoverVertex(5),
            DiscoverEdge(e(4, 6)),
            DiscoverTreeEdge(e(4, 6)),
            DiscoverVertex(6),
            FinishVertex(4),
            DiscoverEdge(e(5, 6)),
            DiscoverBackEdge(e(5, 6)),
            FinishEdge(e(5, 6)),
            FinishVertex(5),
            FinishTreeEdge(e(4, 5)),
            FinishEdge(e(4, 5)),
            FinishVertex(6),
            FinishTreeEdge(e(4, 6)),
            FinishEdge(e(4, 6)),
            FinishRootVertex(4),
        ];

        let mut v = vec![];
        Bfs::new(&g).traverse_all(FnTraverseEvent(|evt| v.push(evt)));
        assert_eq!(expected, v);
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
