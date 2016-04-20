use graph::*;
use fera::{VecExt, IteratorExt};
use fera::collections::Extend1;
use unionfind::WithUnionFind;

#[derive(PartialEq, Eq)]
pub enum Accept {
    Yes,
    No,
}

pub trait Visitor<G>
    where G: Graph
{
    fn visit(&mut self, e: Edge<G>) -> Accept;
}

impl<F, G> Visitor<G> for F
    where G: Graph,
          F: FnMut(Edge<G>) -> Accept
{
    fn visit(&mut self, e: Edge<G>) -> Accept {
        self(e)
    }
}

// TODO: Allow an UnionFind as parameter, so the kruskal algorithm can be
// executed in more than one step

pub trait Kruskal: Graph {
    fn kruskal_with_edges<I, V>(&self, edges: I, visitor: &mut V)
        where I: Iterator<Item = Edge<Self>>,
              V: Visitor<Self>
    {
        // TODO: move num_sets to UnionFind
        let mut ds = self.new_unionfind();
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

    fn kruskal_with_edges_collect_to<I, C>(&self, edges: I, mut tree: C) -> C
        where I: Iterator<Item = Edge<Self>>,
              C: Extend1<Edge<Self>>
    {
        // TODO: Create a CollectorVisitor struct an make it implement Visitor,
        // so this function can be removed
        tree.extend1_reserve(self.num_vertices() - 1);
        self.kruskal_with_edges(edges,
                                &mut |e| {
                                    tree.extend1(e);
                                    Accept::Yes
                                });
        tree
    }

    fn kruskal<T, W, V>(&self, weight: &W, visitor: &mut V)
        where Self: WithProps<T>,
              W: PropEdge<Self, T>,
              T: PartialOrd + Clone,
              V: Visitor<Self>
    {
        let edges = self.edges().into_vec().partial_ord_sorted_by_key(|e| &weight[*e]);
        self.kruskal_with_edges(edges.into_iter(), visitor);
    }

    fn kruskal_mst<T, W>(&self, weight: &W) -> VecEdge<Self>
        where Self: WithProps<T>,
              W: PropEdge<Self, T>,
              T: PartialOrd + Clone
    {
        // TODO: Use CollectorVisitor
        let mut tree = vec![];
        self.kruskal(weight,
                     &mut |e| {
                         tree.push(e);
                         Accept::Yes
                     });
        tree
    }
}

impl<G> Kruskal for G where G: Graph {}


#[cfg(test)]
mod tests {
    use graph::*;
    use static_::*;
    use fera::IteratorExt;
    use kruskal::*;

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn kruskal_mst() {
        let g = graph!(
            StaticGraph,
            5,
            (0, 4), (2, 3), (0, 1), (1, 4), (1, 2), (2, 4), (3, 4)
            // expected tree
            // 0      1       2               3
        );
        let mut weight = g.edge_prop(0usize);
        for (e, &w) in g.edges().zip(&[1, 2, 3, 4, 5, 6, 7]) {
            weight[e] = w;
        }
        let e = g.edges().into_vec();
        assert_eq!(vec![e[0], e[1], e[2], e[4]], g.kruskal_mst(&weight));
    }
}

// TODO: write benchmarks and optimize
