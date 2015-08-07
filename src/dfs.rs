use super::*;

pub trait Dfs: GraphAdj + WithVertexProp {
    fn dfs(&self, v: Self::Vertex, visitor: &mut DfsVisitor<Self>);
    fn dfs_fun<F>(&self, v: Self::Vertex, fun: F) where F: FnMut(Self::Vertex, Self::Vertex) -> bool;
}

pub trait DfsVisitor<G: Basic> {
    fn discover(&mut self, u: G::Vertex, v: G::Vertex) -> bool;
}

impl<G, F> DfsVisitor<G> for Box<F> where G: Basic, F: FnMut(G::Vertex, G::Vertex) -> bool {
    fn discover(&mut self, u: G::Vertex, v: G::Vertex) -> bool {
        self(u, v)
    }
}

impl<G> Dfs for G where
        G: GraphAdj + WithVertexProp {

    fn dfs<'a>(&'a self, v: Self::Vertex, visitor: &mut DfsVisitor<G>) {
        let mut discovered = self.vertex_prop(false);
        let mut stack : Vec<(Self::Vertex, <Self as AdjIter<'a>>::Type)> = vec![(v, self.neighbors(v))];
        discovered[v] = true;
        while !stack.is_empty() {
            let (u, mut adj) = stack.pop().unwrap();
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

    fn dfs_fun<F>(&self, v: Self::Vertex, fun: F) where F: FnMut(Self::Vertex, Self::Vertex) -> bool {
        self.dfs(v, &mut Box::new(fun))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::super::*;

    #[test]
    fn dfs() {
        let g = graphadj::StaticGraph::new_edges(6, &[(0, 1), (0, 2), (2, 3), (2, 4), (4, 5)]);
        let mut d = g.vertex_prop(0usize);
        g.dfs_fun(0, |u, v| {
            d[v] = d[u] + 1;
            true
        });
        assert_eq!(vec![0, 1, 1, 2, 2, 3], d);
    }
}
