use graph::*;
use iter::IteratorExt;
use unionfind::DisjointSet;

#[derive(PartialEq, Eq)]
pub enum Accept {
    Yes,
    No,
}

pub trait Visitor<'a, G: Basic<'a>> {
    fn visit(&mut self, e: G::Edge) -> Accept;
}

impl<'a, F, G> Visitor<'a, G> for F
    where G: Basic<'a>,
          F: FnMut(G::Edge) -> Accept {
    fn visit(&mut self, e: G::Edge) -> Accept {
        self(e)
    }
}

pub trait Kruskal<'a>: Basic<'a> + WithVertexProp<'a> + Sized {
    fn kruskal_edges<I, V>(&'a self, edges: I, visitor: &mut V)
        where I: Iterator<Item = Self::Edge>,
              V: Visitor<'a, Self>
    {
        let mut ds = DisjointSet::new(self);
        let mut num_sets = self.num_vertices();
        for e in edges {
            let (u, v) = self.endvertices(e);
            if !ds.in_same_set(u, v) && visitor.visit(e) == Accept::Yes {
                ds.union(u, v);
                num_sets -= 1;
                if num_sets == 1 {
                    return;
                }
            }
        }
    }

    fn kruskal<T, V>(&'a self, weight: &'a EdgeProp<'a, Self, T>, visitor: &mut V)
        where T: Ord + Clone,
              V: Visitor<'a, Self>,
              Self: EdgeProperty<'a, T>
    {
        let mut edges = self.edges().as_vec();
        edges.sort_by(|&a, &b| weight[a].cmp(&weight[b]));
        self.kruskal_edges(edges.iter().cloned(), visitor);
    }

    fn kruskal_mst<T>(&'a self, weight: &'a EdgeProp<'a, Self, T>) -> VecEdge<Self>
        where T: Ord + Clone,
              Self: EdgeProperty<'a, T>
    {
        let mut edges = vec![];
        self.kruskal(weight, &mut |e| {
            edges.push(e);
            Accept::Yes
        });
        edges
    }
}

impl<'a, G> Kruskal<'a> for G
    where G: Basic<'a> + WithVertexProp<'a> { }


#[cfg(test)]
mod tests {
    use graph::*;
    use static_::*;
    use iter::*;
    use kruskal::*;

    #[test]
    fn kruskal_mst() {
        let g = StaticGraph::new_with_edges(
            5,
            &[(0, 4), (2, 3), (0, 1), (1, 4), (1, 2), (2, 4), (3, 4)]);
        // expected tree
        //      0       1       2               3
        let mut weight = g.edge_prop(0usize);
        for (e, &w) in g.edges().zip(&[1, 2, 3, 4, 5, 6, 7]) {
            weight[e] = w;
        }
        let e = g.edges().as_vec();
        assert_eq!(vec![e[0], e[1], e[2], e[4]], g.kruskal_mst(&weight));
    }
}
