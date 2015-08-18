use super::{
    Basic,
    IncIter,
    GraphInc,
    WithVertexProp,
    WithEdgeProp,
};


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


// Dfs

pub trait Dfs: GraphInc + WithVertexProp + WithEdgeProp + Sized {
    fn dfs<'a, V>(&'a self, v: Self::Vertex, mut visitor: V) where V: Visitor<Self> {
        let mut discovered = self.vertex_prop(false);
        let mut examined = self.edge_prop(false);
        let mut stack : Vec<(Self::Vertex, IncIter<'a, Self>)> = vec![(v, self.inc_edges(v))];
        discovered[v] = true;
        while let Some((u, mut inc)) = stack.pop() {
            while let Some(e) = inc.next() {
                let v = self.target(e);
                if !discovered[v] {
                    if !visitor.visit_tree_edge(e) { return }
                    discovered[v] = true;
                    examined[e] = true;
                    stack.push((u, inc));
                    stack.push((v, self.inc_edges(v)));
                    break;
                } else if !examined[e] {
                    examined[e] = true;
                    if !visitor.visit_back_edge(e) { return }
                }
            }
        }
    }
}

impl<G> Dfs for G where G: GraphInc + WithVertexProp + WithEdgeProp { }


// Bfs

pub trait Bfs: GraphInc + WithVertexProp + WithEdgeProp + Sized {
    fn bfs<V>(&self, v: Self::Vertex, mut visitor: V) where V: Visitor<Self> {
        use std::collections::VecDeque;
        let mut discovered = self.vertex_prop(false);
        let mut examined = self.edge_prop(false);
        let mut queue = VecDeque::new();
        queue.push_back(v);
        discovered[v] = true;
        while let Some(u) = queue.pop_front() {
            for e in self.inc_edges(u) {
                let v = self.target(e);
                if !discovered[v] {
                    if !visitor.visit_tree_edge(e) { return }
                    examined[e] = true;
                    discovered[v] = true;
                    queue.push_back(v);
                } else if !examined[e] {
                    examined[e] = true;
                    if !visitor.visit_back_edge(e) { return }
                }
            }
        }
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
            4, &[(0, 1), (0, 2), (1, 2), (1, 3), (2, 3)])
        // u -> e (u, v)
        // 0 -> 0 (0, 1) 1 (0, 2)
        // 1 -> 1 (1, 0) 2 (1, 2) 3 (1, 3)
        // 2 -> 1 (2, 0) 2 (2, 1) 4 (2, 3)
        // 3 -> 3 (3, 1) 4 (3, 2)
    }

    #[test]
    fn dfs() {
        let g = new();
        let mut d = g.vertex_prop(0usize);
        let mut p = g.vertex_prop(0usize);
        g.dfs(0, TreeEdgeVisitor(|e| {
            let (u, v) = g.endvertices(e);
            d[v] = d[u] + 1;
            p[v] = u;
            true
        }));
        assert_eq!(vec![0, 1, 2, 3], d);
        assert_eq!(vec![0, 0, 1, 2], p);

        let e = g.edges().as_vec();
        let mut back = vec![];
        g.dfs(0, BackEdgeVisitor(|e| {
            back.push(e);
            true
        }));
        assert_eq!(vec![e[1], e[3]], back);
    }

    #[test]
    fn bfs() {
        let g = new();
        let mut d = g.vertex_prop(0usize);
        let mut p = g.vertex_prop(0usize);
        g.bfs(0, TreeEdgeVisitor(|e| {
            let (u, v) = g.endvertices(e);
            d[v] = d[u] + 1;
            p[v] = u;
            true
        }));
        assert_eq!(vec![0, 1, 1, 2], d);
        assert_eq!(vec![0, 0, 0, 1], p);

        let e = g.edges().as_vec();
        let mut back = vec![];
        g.bfs(0, BackEdgeVisitor(|e| {
            back.push(e);
            true
        }));
        assert_eq!(vec![e[2], e[4]], back);
    }
}
