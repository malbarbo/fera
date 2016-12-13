use graph::*;
use unionfind::{UnionFind, WithUnionFind};
use sort::SortByProp;
use params::*;

use std::vec;
use std::borrow::BorrowMut;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Accept {
    Yes,
    No,
}

pub trait Visitor<G>
    where G: Graph
{
    fn visit(&mut self, e: Edge<G>) -> Accept;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AcceptAll;

impl<G: Graph> Visitor<G> for AcceptAll {
    fn visit(&mut self, _: Edge<G>) -> Accept {
        Accept::Yes
    }
}

impl<F, G> Visitor<G> for F
    where G: Graph,
          F: FnMut(Edge<G>) -> Accept
{
    fn visit(&mut self, e: Edge<G>) -> Accept {
        self(e)
    }
}

pub struct Iter<'a, G: 'a, E, V = AcceptAll, U = UnionFind<G>> {
    g: &'a G,
    edges: E,
    visitor: V,
    ds: U,
    // TODO: move num_sets to UnionFind
    num_sets: usize,
}

impl<'a, G, E, V, U> Iterator for Iter<'a, G, E, V, U>
    where G: 'a + WithUnionFind,
          E: Iterator<Item = Edge<G>>,
          V: Visitor<G>,
          U: BorrowMut<UnionFind<G>>
{
    type Item = Edge<G>;

    fn next(&mut self) -> Option<Edge<G>> {
        if self.num_sets > 1 {
            let ds = self.ds.borrow_mut();
            for e in self.edges.by_ref() {
                let (u, v) = self.g.ends(e);
                if !ds.in_same_set(u, v) && self.visitor.visit(e) == Accept::Yes {
                    ds.union(u, v);
                    self.num_sets -= 1;
                    return Some(e);
                }
            }
        }
        None
    }
}

pub trait Kruskal: WithUnionFind {
    fn kruskal_mst<T, W>(&self, weight: &W) -> Iter<Self, vec::IntoIter<Edge<Self>>>
        where W: EdgePropGet<Self, T>,
              T: Ord
    {
        self.kruskal_()
            .weight(weight)
            .run()
    }

    fn kruskal_(&self) -> KruskalAlg<&Self, AllEdges, AcceptAll, NewUnionFind> {
        KruskalAlg(self, AllEdges, AcceptAll, NewUnionFind)
    }
}

impl<G: WithUnionFind> Kruskal for G {}


generic_struct!(KruskalAlg(graph, edges, visitor, unionfind));

impl<'a, G, E, V, U> KruskalAlg<&'a G, E, V, U>
    where G: WithUnionFind
{
    pub fn weight<W, T>(self, w: &W) -> KruskalAlg<&'a G, Vec<Edge<G>>, V, U>
        where W: EdgePropGet<G, T>,
              T: Ord
    {
        let mut edges: Vec<_> = self.0.edges().collect();
        edges.sort_by_prop(w);
        self.edges(edges)
    }

    pub fn run(self) -> Iter<'a, G, E::Output, V, U::Output>
        where E: ParamIterator<'a, G, Item = Edge<G>>,
              V: Visitor<G>,
              U: Param<'a, G, UnionFind<G>>
    {
        let KruskalAlg(g, edges, visitor, ds) = self;
        Iter {
            g: g,
            edges: edges.build(g),
            visitor: visitor,
            ds: ds.build(g),
            num_sets: g.num_vertices(),
        }
    }
}

#[derive(Default)]
pub struct NewUnionFind;

impl<'a, G: 'a + WithUnionFind> Param<'a, G, UnionFind<G>> for NewUnionFind {
    type Output = UnionFind<G>;

    fn build(self, g: &'a G) -> Self::Output {
        g.new_unionfind()
    }
}

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
