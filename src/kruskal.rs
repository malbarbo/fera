use graph::*;
use fera::{IteratorExt, VecExt};
use unionfind::{UnionFind, WithUnionFind};
use std::vec;

#[derive(Clone, Copy, PartialEq, Eq)]
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

// TODO: implemets Visitor for &mut V where V: Visitor

impl<G: Graph> Visitor<G> for Accept {
    fn visit(&mut self, _: Edge<G>) -> Accept {
        *self
    }
}

// TODO: allow reuse of KruskalIter
pub struct KruskalIter<'a, G: 'a + Graph, I, V> {
    g: &'a G,
    ds: UnionFind<G>,
    edges: I,
    visitor: V,
    // TODO: move num_sets to UnionFind
    num_sets: usize,
}

impl<'a, G, I, V> Iterator for KruskalIter<'a, G, I, V>
    where G: 'a + Graph,
          I: Iterator<Item = Edge<G>>,
          V: Visitor<G>
{
    type Item = Edge<G>;

    fn next(&mut self) -> Option<Edge<G>> {
        if self.num_sets > 1 {
            for e in &mut self.edges {
                let (u, v) = self.g.ends(e);
                if !self.ds.in_same_set(u, v) && self.visitor.visit(e) == Accept::Yes {
                    self.ds.union(u, v);
                    self.num_sets -= 1;
                    return Some(e);
                }
            }
        }
        None
    }
}


pub trait Kruskal: IncidenceGraph {
    fn kruskal_with_edges<I, V>(&self, edges: I, visitor: V) -> KruskalIter<Self, I, V>
        where I: Iterator<Item = Edge<Self>>,
              V: Visitor<Self>
    {
        KruskalIter {
            g: self,
            ds: self.new_unionfind(),
            edges: edges,
            visitor: visitor,
            num_sets: self.num_vertices(),
        }
    }

    fn kruskal<T, W, V>(&self,
                        weight: &W,
                        visitor: V)
                        -> KruskalIter<Self, vec::IntoIter<Edge<Self>>, V>
        where W: EdgePropGet<Self, T>,
              T: PartialOrd,
              V: Visitor<Self>
    {
        let edges = self.edges()
            .into_vec()
            .partial_ord_sorted_by_key(|&e| weight.get(e));

        self.kruskal_with_edges(edges.into_iter(), visitor)
    }

    fn kruskal_mst<T, W>(&self, weight: &W) -> KruskalIter<Self, vec::IntoIter<Edge<Self>>, Accept>
        where W: EdgePropGet<Self, T>,
              T: PartialOrd
    {
        self.kruskal(weight, Accept::Yes)
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
        assert_eq!(vec![e[0], e[1], e[2], e[4]], g.kruskal_mst(&weight).into_vec());
    }
}

// TODO: write benchmarks and optimize
