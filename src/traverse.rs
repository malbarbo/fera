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


// Traversers

pub trait Traverser<'a, G>: Sized
    where G: 'a + Graph,
          &'a G: Types<G>,
{
    fn new(g: &'a G) -> Self;

    fn is_discovered(&mut self, v: Vertex<G>) -> bool;

    fn traverse<V: Visitor<G>>(&mut self, v: Vertex<G>, vis: &mut V) -> bool;

    fn run<V: Visitor<G>>(g: &'a G, vis: &mut V) {
        let mut t = Self::new(g);
        for v in g.vertices() {
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


// FIXME: To initialize Dfs and Bfs is necessary O(V + E). Some uses of dfs and bfs stop
// traversing before visiting all vertices and edges. Ideally the running time and space
// should be proportional to the number of visited vertices and edges.


// Dfs

pub struct Dfs<'a, G>
    where G: 'a + Graph,
          &'a G: Types<G>
{
    g: &'a G,
    discovered: PropVertex<G, bool>,
    examined: PropEdge<G, bool>,
}

impl<'a, G> Traverser<'a, G> for Dfs<'a, G>
    where G: 'a + Graph,
          &'a G: Types<G>,
{
    fn new(g: &'a G) -> Self {
        Dfs {
            g: g,
            discovered: g.vertex_prop(false),
            examined: g.edge_prop(false),
        }
    }

    fn is_discovered(&mut self, v: Vertex<G>) -> bool {
        self.discovered[v]
    }

    fn traverse<V: Visitor<G>>(&mut self, v: Vertex<G>, vis: &mut V) -> bool {
        let mut stack: Vec<(_, IterInc<'a, _>)> = vec![(v, self.g.inc_edges(v))];
        self.discovered[v] = true;
        while let Some((u, mut inc)) = stack.pop() {
            while let Some(e) = inc.next() {
                let v = self.g.target(e);
                if !self.discovered[v] {
                    return_if_false!(vis.visit_tree_edge(e));
                    self.discovered[v] = true;
                    self.examined[e] = true;
                    stack.push((u, inc));
                    stack.push((v, self.g.inc_edges(v)));
                    break;
                } else if !self.examined[e] {
                    self.examined[e] = true;
                    return_if_false!(vis.visit_back_edge(e));
                }
            }
        }
        true
    }
}


// Bfs

pub struct Bfs<'a, G>
    where G: 'a + Graph,
          &'a G: Types<G>
{
    g: &'a G,
    discovered: PropVertex<G, bool>,
    examined: PropEdge<G, bool>,
}

impl<'a, G> Traverser<'a, G> for Bfs<'a, G>
    where G: 'a + Graph,
          &'a G: Types<G>,
{
    fn new(g: &'a G) -> Self {
        Bfs {
            g: g,
            discovered: g.vertex_prop(false),
            examined: g.edge_prop(false),
        }
    }

    fn is_discovered(&mut self, v: Vertex<G>) -> bool {
        self.discovered[v]
    }

    fn traverse<V: Visitor<G>>(&mut self, v: Vertex<G>, vis: &mut V) -> bool {
        let mut queue = VecDeque::new();
        queue.push_back(v);
        self.discovered[v] = true;
        while let Some(u) = queue.pop_front() {
            for e in self.g.inc_edges(u) {
                let v = self.g.target(e);
                if !self.discovered[v] {
                    return_if_false!(vis.visit_tree_edge(e));
                    self.examined[e] = true;
                    self.discovered[v] = true;
                    queue.push_back(v);
                } else if !self.examined[e] {
                    self.examined[e] = true;
                    return_if_false!(vis.visit_back_edge(e));
                }
            }
        }
        true
    }
}


// Tests

#[cfg(test)]
mod tests {
    use graph::*;
    use static_::*;
    use ds::IteratorExt;
    use traverse::*;

    fn new() -> StaticGraph {
        StaticGraph::new_with_edges(7,
                                    &[(0, 1), (0, 2), (1, 2), (1, 3), (2, 3), (4, 5), (4, 6),
                                      (5, 6)])
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
              &'a G: Types<G>    
{
        g: &'a G,
        parent: PropVertex<G, OptionVertex<G>>,
        d: PropVertex<G, usize>,
        edge_type: PropEdge<G, usize>,
    }

    fn new_test_visitor(g: &StaticGraph) -> TestVisitor<StaticGraph> {
        TestVisitor {
            g: g,
            parent: g.vertex_prop(None),
            d: g.vertex_prop(0),
            edge_type: g.edge_prop(0),
        }
    }

    impl<'a, G> Visitor<G> for TestVisitor<'a, G>
        where G: 'a + Graph,
              &'a G: Types<G>,
    {
        fn visit_tree_edge(&mut self, e: Edge<G>) -> bool {
            assert_eq!(0, self.edge_type[e]);
            self.parent[self.g.target(e)] = Some(self.g.source(e));
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
                   vis.parent);

        assert_eq!(vec![0, 1, 2, 3, 0, 1, 2], vis.d);

        assert_eq!(vec![TREE, BACK, TREE, BACK, TREE, TREE, BACK, TREE],
                   vis.edge_type.0);
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
                   vis.parent);

        assert_eq!(vec![0, 1, 1, 2, 0, 1, 1], vis.d);

        assert_eq!(vec![TREE, TREE, BACK, TREE, BACK, TREE, TREE, BACK],
                   vis.edge_type.0);
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
