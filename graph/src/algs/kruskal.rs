// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! [Kruskal]'s minimum spanning tree algorithm.
//!
//! [Kruskal]: https://en.wikipedia.org/wiki/Kruskal's_algorithm

use params::*;
use prelude::*;
use unionfind::{NewUnionFind, UnionFind, WithUnionFind};

use fera_fun::vec;

use std::ops::DerefMut;

pub trait Visitor<G>
where
    G: WithEdge + WithUnionFind,
{
    fn accept(&mut self, g: &G, e: Edge<G>, ds: &mut UnionFind<G>) -> bool;

    #[allow(unused_variables)]
    fn after_union(&mut self, g: &G, e: Edge<G>, ds: &mut UnionFind<G>) {}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AcceptAll;

impl<G> Visitor<G> for AcceptAll
where
    G: WithEdge + WithUnionFind,
{
    fn accept(&mut self, _g: &G, _e: Edge<G>, _ds: &mut UnionFind<G>) -> bool {
        true
    }
}

impl<F, G> Visitor<G> for F
where
    G: WithEdge + WithUnionFind,
    F: FnMut(&G, Edge<G>, &mut UnionFind<G>) -> bool,
{
    fn accept(&mut self, g: &G, e: Edge<G>, ds: &mut UnionFind<G>) -> bool {
        self(g, e, ds)
    }
}

pub struct Iter<'a, G: 'a, E, V, U> {
    g: &'a G,
    edges: E,
    visitor: V,
    ds: U,
}

impl<'a, G, E, V, U> Iterator for Iter<'a, G, E, V, U>
where
    G: 'a + WithUnionFind,
    E: Iterator,
    E::Item: IntoOwned<Edge<G>>,
    V: Visitor<G>,
    U: DerefMut<Target = UnionFind<G>>,
{
    type Item = Edge<G>;

    fn next(&mut self) -> Option<Edge<G>> {
        if self.ds.num_sets() > 1 {
            for e in self.edges.by_ref().map(|e| e.into_owned()) {
                let (u, v) = self.g.ends(e);
                if !self.ds.in_same_set(u, v) && self.visitor.accept(self.g, e, &mut self.ds) {
                    self.ds.union(u, v);
                    self.visitor.after_union(self.g, e, &mut self.ds);
                    return Some(e);
                }
            }
        }
        None
    }
}

pub trait Kruskal: WithUnionFind {
    fn kruskal_mst<T, W>(
        &self,
        weight: W,
    ) -> KruskalAlg<&Self, Vec<Edge<Self>>, AcceptAll, NewUnionFind<Self>>
    where
        W: EdgePropGet<Self, T>,
        T: Ord,
    {
        self.kruskal().weight(weight)
    }

    fn kruskal(&self) -> KruskalAlg<&Self, AllEdges<Self>, AcceptAll, NewUnionFind<Self>> {
        KruskalAlg(self, AllEdges(self), AcceptAll, NewUnionFind(self))
    }
}

impl<G: WithUnionFind> Kruskal for G {}

generic_struct! {
    #[must_use]
    pub struct KruskalAlg(graph, edges, visitor, unionfind)
}

impl<'a, G, E, V, U> KruskalAlg<&'a G, E, V, U>
where
    G: WithUnionFind,
{
    pub fn weight<W, T>(self, w: W) -> KruskalAlg<&'a G, Vec<Edge<G>>, V, U>
    where
        W: EdgePropGet<G, T>,
        T: Ord,
    {
        let edges = vec(self.0.edges()).sorted_by_prop(&w);
        self.edges(edges)
    }
}

impl<'a, G, E, V, U> IntoIterator for KruskalAlg<&'a G, E, V, U>
where
    G: WithUnionFind,
    E: IntoIterator,
    E::Item: IntoOwned<Edge<G>>,
    V: Visitor<G>,
    U: ParamDerefMut<Target = UnionFind<G>>,
{
    type Item = Edge<G>;
    type IntoIter = Iter<'a, G, E::IntoIter, V, U::Output>;

    fn into_iter(self) -> Self::IntoIter {
        let KruskalAlg(g, edges, visitor, ds) = self;
        Iter {
            g,
            visitor,
            edges: edges.into_iter(),
            ds: ds.build(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Kruskal;
    use fera_fun::vec;
    use prelude::*;

    #[test]
    fn kruskal_mst() {
        let g: StaticGraph = graph!(
            5,
            (0, 4), // 0
            (2, 3), // 1
            (0, 1), // 2
            (1, 4),
            (1, 2), // 4
            (2, 4),
            (3, 4),
        );
        let mut weight = g.default_edge_prop(0usize);
        for (e, &w) in g.edges().zip(&[1, 2, 3, 4, 5, 6, 7]) {
            weight[e] = w;
        }
        let e = vec(g.edges());
        assert_eq!(vec![e[0], e[1], e[2], e[4]], vec(g.kruskal_mst(&weight)));
    }
}

// TODO: write benchmarks and optimize
