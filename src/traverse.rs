use super::{
    Basic,
    AdjIter,
    GraphAdj,
    WithVertexProp,
};

// Visitor

pub trait Visitor<G: Basic> {
    fn discover(&mut self, u: G::Vertex, v: G::Vertex) -> bool;
}

impl<G, F> Visitor<G> for F where G: Basic, F: FnMut(G::Vertex, G::Vertex) -> bool {
    fn discover(&mut self, u: G::Vertex, v: G::Vertex) -> bool {
        self(u, v)
    }
}


// Dfs

pub trait Dfs: GraphAdj + WithVertexProp + Sized {
    fn dfs<'a, V>(&'a self, v: Self::Vertex, visitor: &mut V) where V: Visitor<Self> {
        let mut discovered = self.vertex_prop(false);
        let mut stack : Vec<(Self::Vertex, AdjIter<'a, Self>)> = vec![(v, self.neighbors(v))];
        discovered[v] = true;
        while let Some((u, mut adj)) = stack.pop() {
            if let Some(v) = adj.find(|v| !discovered[*v]) {
                if !visitor.discover(u, v) {
                    return;
                }
                discovered[v] = true;
                stack.push((u, adj));
                stack.push((v, self.neighbors(v)));
            }
        }
    }

    fn dfs_fun<F>(&self, v: Self::Vertex, mut fun: F) where F: FnMut(Self::Vertex, Self::Vertex) -> bool {
        self.dfs(v, &mut fun)
    }
}

impl<G> Dfs for G where G: GraphAdj + WithVertexProp { }


// Bfs

pub trait Bfs: GraphAdj + WithVertexProp + Sized {
    fn bfs<V>(&self, v: Self::Vertex, visitor: &mut V) where V: Visitor<Self> {
        use std::collections::VecDeque;
        let mut discovered = self.vertex_prop(false);
        let mut queue = VecDeque::new();
        queue.push_back(v);
        discovered[v] = true;
        while let Some(u) = queue.pop_front() {
            for v in self.neighbors(u) {
                if !discovered[v] {
                    if !visitor.discover(u, v) {
                        return;
                    }
                    discovered[v] = true;
                    queue.push_back(v);
                }
            }
        }
    }

    fn bfs_fun<F>(&self, v: Self::Vertex, mut fun: F) where F: FnMut(Self::Vertex, Self::Vertex) -> bool {
        self.dfs(v, &mut fun)
    }
}

impl<G> Bfs for G where G: GraphAdj + WithVertexProp { }


// Tests

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::*;

    #[test]
    fn dfs() {
        let g = StaticGraph::new_with_edges(6, &[(0, 1), (0, 2), (2, 3), (2, 4), (4, 5)]);
        let mut d = g.vertex_prop(0usize);
        g.dfs_fun(0, |u, v| {
            d[v] = d[u] + 1;
            true
        });
        assert_eq!(vec![0, 1, 1, 2, 2, 3], d);
    }

    #[test]
    fn bfs() {
        let g = StaticGraph::new_with_edges(6, &[(0, 1), (0, 2), (2, 3), (2, 4), (4, 5)]);
        let mut d = g.vertex_prop(0usize);
        g.bfs_fun(0, |u, v| {
            d[v] = d[u] + 1;
            true
        });
        assert_eq!(vec![0, 1, 1, 2, 2, 3], d);
    }
}
