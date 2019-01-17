// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[cfg(feature = "quickcheck")]
#[macro_use]
extern crate quickcheck;
extern crate fera_fun;
extern crate fera_graph;

#[cfg(feature = "quickcheck")]
mod quickchecks {
    use fera_fun::vec;
    use fera_graph::algs::{Boruvka, Kruskal, Prim, Trees};
    use fera_graph::arbitrary::GnConnectedWithEdgeProp;
    use fera_graph::prelude::*;
    use fera_graph::sum_prop;

    quickcheck! {
        fn mst(x: GnConnectedWithEdgeProp<StaticGraph, u32>) -> bool {
            let GnConnectedWithEdgeProp(g, w) = x;
            if g.num_vertices() == 0 || g.num_edges() == 0 {
                return true;
            }
            let boruvka = g.boruvka(&w).run();
            let w_boruvka: u32 = sum_prop(&w, &boruvka);
            let kruskal = vec(g.kruskal_mst(&w));
            let w_kruskal: u32 = sum_prop(&w, &kruskal);
            let prim = vec(g.prim(&w));
            let w_prim: u32 = sum_prop(&w, &prim);
            assert!(g.spanning_subgraph(&boruvka).is_tree());
            assert!(g.spanning_subgraph(&kruskal).is_tree());
            assert!(g.spanning_subgraph(&prim).is_tree());
            assert_eq!(w_boruvka, w_kruskal);
            assert_eq!(w_boruvka, w_prim);
            true
        }
    }
}
