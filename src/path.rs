use graph::*;
use traverse::*;

pub type Path<G> = VecEdge<G>;

pub type ParentTree<'a, G> = VertexProp<'a, G, OptionEdge<G>>;

pub trait FindPath<'a>: Basic<'a> + Sized {
    fn find_path_on_parent_tree(&self,
                                tree: &ParentTree<'a, Self>,
                                u: Self::Vertex,
                                v: Self::Vertex)
                                -> Option<Path<Self>>
        where Self: WithVertexProp<'a>
    {
        if u == v {
            return None;
        }
        let mut v = v;
        let mut path = vec![];
        // TODO: detect loop
        while let Some(e) = tree[v] {
            v = self.source(e);
            path.push(e);
            if v == u {
                path.reverse();
                return Some(path);
            }
        }
        None
    }

    fn find_path(&'a self, u: Self::Vertex, v: Self::Vertex) -> Option<Path<Self>>
        where Self: GraphIncWithProps<'a>
    {
        if u == v {
            return None;
        }
        let mut found = false;
        let none: OptionEdge<Self> = None;
        let mut tree = self.vertex_prop(none);
        Dfs::run_start(self, u, &mut TreeEdgeVisitor(|e| {
            let t = self.target(e);
            tree[t] = Some(e);
            found = t == v;
            !found
        }));
        if found {
            self.find_path_on_parent_tree(&tree, u, v)
        } else {
            None
        }
    }
}

impl<'a, G> FindPath<'a> for G
    where G: Basic<'a> { }

#[cfg(test)]
mod tests {
    use graph::*;
    use static_::*;
    use iter::*;
    use path::*;

    #[test]
    fn find_path() {
        let g = StaticGraph::new_with_edges(6, &[(0, 1), (0, 2), (1, 4), (2, 3), (2, 4)]);
        let e = g.edges().as_vec();

        assert_eq!(None, g.find_path(0, 0));

        assert_eq!(None, g.find_path(0, 5));

        assert_eq!(vec![e[0]],
                   g.find_path(0, 1).unwrap());

        assert_eq!(vec![e[0], e[1], e[4]],
                   g.find_path(1, 4).unwrap());
    }
}
