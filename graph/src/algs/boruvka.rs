// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! [Borůvka]'s minimum spannin tree algoritm.
//!
//! [Borůvka]: https://en.wikipedia.org/wiki/Borůvka's_algorithm

use params::*;
use prelude::*;
use unionfind::{NewUnionFind, UnionFind, WithUnionFind};

pub trait Boruvka: WithUnionFind {
    fn boruvka<W>(
        &self,
        weight: W,
    ) -> BoruvkaAlg<&Self, W, Owned<Vec<OptionEdge<Self>>>, NewUnionFind<Self>> {
        BoruvkaAlg(self, weight, Owned(vec![]), NewUnionFind(self))
    }
}

impl<G: WithUnionFind> Boruvka for G {}

generic_struct! {
    #[must_use]
    pub struct BoruvkaAlg(graph, weight, safe, unionfind)
}

impl<'a, G, W, S, U> BoruvkaAlg<&'a G, W, S, U> {
    // FIXME: return a iterator, like kruskal and prim
    pub fn run<T>(self) -> Vec<Edge<G>>
    where
        G: WithUnionFind + WithVertexIndexProp,
        W: EdgePropGet<G, T>,
        T: PartialOrd,
        S: ParamDerefMut<Target = Vec<OptionEdge<G>>>,
        U: ParamDerefMut<Target = UnionFind<G>>,
    {
        let BoruvkaAlg(g, w, safe, ds) = self;
        let mut safe = safe.build();
        let mut ds = ds.build();
        let mut edges = vec![];
        let mut changed = true;
        let index = g.vertex_index();
        safe.resize(g.num_vertices(), None.into());
        while changed && ds.num_sets() > 1 {
            changed = false;
            for (e, u, v) in g.edges_with_ends() {
                let u_comp = index.get(ds.find_set(u));
                let v_comp = index.get(ds.find_set(v));
                if u_comp != v_comp {
                    let we = w.get(e);
                    safe[u_comp] = match safe[u_comp].into_option() {
                        None => e.into(),
                        Some(f) if we < w.get(f) => e.into(),
                        _ => safe[u_comp],
                    };
                    safe[v_comp] = match safe[v_comp].into_option() {
                        None => e.into(),
                        Some(f) if we < w.get(f) => e.into(),
                        _ => safe[v_comp],
                    };
                }
            }
            // FIXME: this should not be linear
            for e in safe.iter_mut() {
                if let Some(ee) = e.into_option() {
                    let (u, v) = g.ends(ee);
                    if !ds.in_same_set(u, v) {
                        ds.union(u, v);
                        edges.push(ee);
                        changed = true;
                    }
                    *e = None.into();
                }
            }
        }
        edges
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fera_fun::vec;
    use fun::sum_prop;

    #[test]
    fn basic() {
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
        let tree = g.boruvka(&weight).run();
        assert_eq!(11usize, sum_prop(&weight, &tree));
        assert_eq!(vec![e[0], e[2], e[1], e[4]], tree);
    }
}
