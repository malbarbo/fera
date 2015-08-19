use super::{
    Basic,
    IncIter,
    GraphInc,
    VertexProp,
    WithVertexProp,
    EdgeProp,
    WithEdgeProp,
};
use std::collections::VecDeque;


// Visitor

pub trait Visitor<G: Basic> {
    fn visit_tree_edge(&mut self, _e: G::Edge) -> bool { true }
    fn visit_back_edge(&mut self, _e: G::Edge) -> bool { true }
}

pub struct TreeEdgeVisitor<F>(pub F);
pub struct BackEdgeVisitor<F>(pub F);

impl<G, F> Visitor<G> for TreeEdgeVisitor<F>
    where G: Basic,
          F: FnMut(G::Edge) -> bool {
    fn visit_tree_edge(&mut self, e: G::Edge) -> bool {
        self.0(e)
    }
}

impl<G, F> Visitor<G> for BackEdgeVisitor<F>
    where G: Basic,
          F: FnMut(G::Edge) -> bool {
    fn visit_back_edge(&mut self, e: G::Edge) -> bool {
        self.0(e)
    }
}

// FIXME: To initialize State is necessary O(V + E). Some uses of dfs and bfs stop traversing
// before visiting all vertices and edges. Ideally the running time and space should be
// proportional to the number of visited vertices and edges.

pub struct State<'a, G> where G: Basic + WithVertexProp + WithEdgeProp {
    discovered: VertexProp<'a, G, bool>,
    examined: EdgeProp<'a, G, bool>,
}

impl<'a, G> State<'a, G> where G: Basic + WithVertexProp + WithEdgeProp {
    pub fn new(g: &G) -> State<G> {
        State {
            discovered: g.vertex_prop(false),
            examined: g.edge_prop(false),
        }
    }
}


// Dfs

pub trait Dfs: GraphInc + WithVertexProp + WithEdgeProp + Sized {
    fn dfs<'a, V>(&'a self, visitor: &mut V) where V: Visitor<Self> {
        let mut state = State::new(self);
        for v in self.vertices() {
            if !state.discovered[v] {
                if !self.dfs_visit_state(&mut state, v, visitor) { return }
            }
        }
    }

    fn dfs_visit<V>(&self, v: Self::Vertex, visitor: &mut V) where V: Visitor<Self> {
        self.dfs_visit_state(&mut State::new(self), v, visitor);
    }

    fn dfs_visit_state<'a, V>(&'a self,
                              state: &mut State<Self>,
                              v: Self::Vertex,
                              visitor: &mut V)
        -> bool where V: Visitor<Self> {
        let mut stack: Vec<(Self::Vertex, IncIter<'a, Self>)> = vec![(v, self.inc_edges(v))];
        state.discovered[v] = true;
        while let Some((u, mut inc)) = stack.pop() {
            while let Some(e) = inc.next() {
                let v = self.target(e);
                if !state.discovered[v] {
                    if !visitor.visit_tree_edge(e) { return false }
                    state.discovered[v] = true;
                    state.examined[e] = true;
                    stack.push((u, inc));
                    stack.push((v, self.inc_edges(v)));
                    break;
                } else if !state.examined[e] {
                    state.examined[e] = true;
                    if !visitor.visit_back_edge(e) { return false }
                }
            }
        }
        true
    }
}

impl<G> Dfs for G where G: GraphInc + WithVertexProp + WithEdgeProp { }


// Bfs

pub trait Bfs: GraphInc + WithVertexProp + WithEdgeProp + Sized {
    fn bfs<V>(&self, visitor: &mut V) where V: Visitor<Self> {
        let mut state = State::new(self);
        for v in self.vertices() {
            if !state.discovered[v] {
                if !self.bfs_visit_state(&mut state, v, visitor) { return }
            }
        }
    }

    fn bfs_visit<V>(&self, v: Self::Vertex, visitor: &mut V) where V: Visitor<Self> {
        self.bfs_visit_state(&mut State::new(self), v, visitor);
    }

    fn bfs_visit_state<V>(&self, state: &mut State<Self>, v: Self::Vertex, visitor: &mut V) -> bool
        where V: Visitor<Self> {
        let mut queue = VecDeque::new();
        queue.push_back(v);
        state.discovered[v] = true;
        while let Some(u) = queue.pop_front() {
            for e in self.inc_edges(u) {
                let v = self.target(e);
                if !state.discovered[v] {
                    if !visitor.visit_tree_edge(e) { return false }
                    state.examined[e] = true;
                    state.discovered[v] = true;
                    queue.push_back(v);
                } else if !state.examined[e] {
                    state.examined[e] = true;
                    if !visitor.visit_back_edge(e) { return false }
                }
            }
        }
        true
    }
}

impl<G> Bfs for G where G: GraphInc + WithVertexProp + WithEdgeProp { }


// Tests

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::*;
    use super::super::iter::IteratorExt;

    fn new() -> StaticGraph {
        StaticGraph::new_with_edges(
            7, &[(0, 1), (0, 2), (1, 2), (1, 3), (2, 3),
                 (4, 5), (4, 6), (5, 6)])
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

    struct TestVisitor<'a, G: 'a + Basic + WithVertexProp + WithEdgeProp> {
        g: &'a G,
        parent: VertexProp<'a, G, Option<G::Vertex>>,
        d: VertexProp<'a, G, usize>,
        edge_type: EdgeProp<'a, G, usize>,
    }

    fn new_test_visitor(g: &StaticGraph) -> TestVisitor<StaticGraph> {
        TestVisitor {
            g: g,
            parent: g.vertex_prop(None),
            d: g.vertex_prop(0),
            edge_type: g.edge_prop(0),
        }
    }

    impl<'a, G> Visitor<G> for TestVisitor<'a, G> where G: Basic + WithVertexProp + WithEdgeProp {
        fn visit_tree_edge(&mut self, e: G::Edge) -> bool {
            assert_eq!(0, self.edge_type[e]);
            self.parent[self.g.target(e)] = Some(self.g.source(e));
            self.d[self.g.target(e)] = self.d[self.g.source(e)] + 1;
            self.edge_type[e] = TREE;
            true
        }

        fn visit_back_edge(&mut self, e: G::Edge) -> bool {
            assert_eq!(0, self.edge_type[e]);
            self.edge_type[e] = BACK;
            true
        }
    }

    #[test]
    fn dfs() {
        let g = new();
        let mut vis = new_test_visitor(&g);
        g.dfs(&mut vis);

        assert_eq!(vec![None, Some(0), Some(1), Some(2), None, Some(4), Some(5)],
                   vis.parent);

        assert_eq!(vec![0, 1, 2, 3, 0, 1, 2],
                   vis.d);

        assert_eq!(vec![TREE, BACK, TREE, BACK, TREE, TREE, BACK, TREE],
                   vis.edge_type.0);
    }

    #[test]
    fn dfs_tree_visitor() {
        let g = new();
        let mut edges = vec![];
        g.dfs(&mut TreeEdgeVisitor(|e| {
            edges.push(e);
            edges.len() != 2
        }));
        let e = g.edges().as_vec();
        assert_eq!(vec![e[0], e[2]], edges);
    }

    #[test]
    fn dfs_back_visitor() {
        let g = new();
        let mut edges = vec![];
        g.dfs(&mut BackEdgeVisitor(|e| {
            edges.push(e);
            edges.len() != 2
        }));
        let e = g.edges().as_vec();
        assert_eq!(vec![e[1], e[3]], edges);
    }

    #[test]
    fn bfs() {
        let g = new();
        let mut vis = new_test_visitor(&g);
        g.bfs(&mut vis);

        assert_eq!(vec![None, Some(0), Some(0), Some(1), None, Some(4), Some(4)],
                   vis.parent);

        assert_eq!(vec![0, 1, 1, 2, 0, 1, 1],
                   vis.d);

        assert_eq!(vec![TREE, TREE, BACK, TREE, BACK, TREE, TREE, BACK],
                   vis.edge_type.0);
    }

    #[test]
    fn bfs_tree_visitor() {
        let g = new();
        let mut edges = vec![];
        g.bfs(&mut TreeEdgeVisitor(|e| {
            edges.push(e);
            edges.len() != 2
        }));
        let e = g.edges().as_vec();
        assert_eq!(vec![e[0], e[1]], edges);
    }

    #[test]
    fn bfs_back_visitor() {
        let g = new();
        let mut edges = vec![];
        g.bfs(&mut BackEdgeVisitor(|e| {
            edges.push(e);
            edges.len() != 2
        }));
        let e = g.edges().as_vec();
        assert_eq!(vec![e[2], e[4]], edges);
    }
}
