use super::{Basic, VertexProp, GraphInc, WithVertexProp, WithEdgeProp};

use super::traverse::*;

pub type Path<G> = Vec<<G as Basic>::Edge>;

pub type ParentTree<'a, G> = VertexProp<'a, G, Option<<G as Basic>::Edge>>;

pub trait FindPath: GraphInc + Sized {
    fn find_path_on_parent_tree(&self,
                                tree: &ParentTree<Self>,
                                u: Self::Vertex,
                                v: Self::Vertex)
                                -> Option<Path<Self>>
        where Self: WithVertexProp
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

    fn find_path(&self, u: Self::Vertex, v: Self::Vertex) -> Option<Path<Self>>
        where Self: WithVertexProp + WithEdgeProp
    {
        if u == v {
            return None;
        }
        let mut found = false;
        let mut tree = self.vertex_prop::<Option<Self::Edge>>(None);
        self.dfs_visit(u, &mut TreeEdgeVisitor(|e| {
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

impl<G: GraphInc> FindPath for G { }

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::*;
    use super::super::iter::IteratorExt;

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
