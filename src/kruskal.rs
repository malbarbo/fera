use graph::*;
use fera::{IteratorExt, VecExt};
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

// TODO: Turn Kruskal in an Iterator

pub trait Kruskal: IncidenceGraph {
    fn kruskal_with_edges<I, V>(&self, edges: I, mut visitor: V)
        where I: Iterator<Item = Edge<Self>>,
              V: Visitor<Self>
    {
        // TODO: move num_sets to UnionFind
        let mut ds = self.new_unionfind();
        let mut num_sets = self.num_vertices();
        for e in edges {
            let (u, v) = self.ends(e);
            if !ds.in_same_set(u, v) && visitor.visit(e) == Accept::Yes {
                ds.union(u, v);
                num_sets -= 1;
                if num_sets == 1 {
                    return;
                }
            }
        }
    }

    fn kruskal<T, W, V>(&self, weight: &W, visitor: V)
        where W: EdgeProp<Self, T>,
              T: PartialOrd + Clone,
              V: Visitor<Self>
    {
        let edges = self.edges()
            .into_vec()
            .partial_ord_sorted_by_key(|e| &weight[*e]);

        self.kruskal_with_edges(edges.into_iter(), visitor);
    }

    fn kruskal_mst<T, W>(&self, weight: &W) -> VecEdge<Self>
        where W: EdgeProp<Self, T>,
              T: PartialOrd + Clone
    {
        let mut tree = vec![];
        self.kruskal(weight, |e| {
            tree.push(e);
            Accept::Yes
        });
        tree
    }
}

impl<G> Kruskal for G where G: IncidenceGraph {}


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
        let mut weight = g.default_edge_prop(0usize);
        for (e, &w) in g.edges().zip(&[1, 2, 3, 4, 5, 6, 7]) {
            weight[e] = w;
        }
        let e = g.edges().into_vec();
        assert_eq!(vec![e[0], e[1], e[2], e[4]], g.kruskal_mst(&weight));
    }
}

// TODO: write benchmarks and optimize
