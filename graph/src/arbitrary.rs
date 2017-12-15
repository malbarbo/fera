// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Generate arbitrary graphs to be use in [quickcheck] tests.
//!
//! This requires enabling `quickcheck` feature.
//!
//! # Examples:
//!
//! Testing [`WithBuilder::new_gn_connected`] against [`Components::is_connected`]:
//!
//! ```
//! #[macro_use]
//! extern crate quickcheck;
//! extern crate fera_graph;
//!
//! use fera_graph::prelude::*;
//! use fera_graph::algs::Components;
//! use fera_graph::arbitrary::GnConnected;
//! use quickcheck::quickcheck;
//!
//! fn main() {
//!     fn tree(g: GnConnected<StaticGraph>) -> bool {
//!         let GnConnected(g) = g;
//!         g.is_connected()
//!     }
//!
//!     quickcheck(tree as fn(GnConnected<StaticGraph>) -> bool);
//! }
//! ```
//!
//! [quickcheck]: https://github.com/BurntSushi/quickcheck
//! [`Components::is_connected`]: ../algs/components/trait.Components.html#method.is_connected
//! [`WithBuilder::new_gn_connected`]: ../builder/trait.WithBuilder.html#method.new_gn_connected

use prelude::*;
use graphs::adjset::{AdjSetEdge, UndirectedEdge};
use props::HashMapProp;

use fera_fun::vec;
use quickcheck::{Arbitrary, Gen};

use std::cmp;
use std::collections::HashSet;
use std::fmt::Debug;

fn shrink_graph<G>(g: &G) -> Box<Iterator<Item = G>>
where
    G: EdgeList + VertexList + WithBuilder,
    G::Kind: UniformEdgeKind,
{
    let mut id: HashMapProp<Vertex<G>, usize> = g.vertex_prop(0);
    for (i, v) in g.vertices().enumerate() {
        id[v] = i;
    }
    let edges = vec(g.edges_ends().map(|(u, v)| (id[u], id[v])));
    let iter = edges.shrink().map(|edges| {
        let n = edges
            .iter()
            .map(|&(u, v)| cmp::max(u, v) + 1)
            .max()
            .unwrap_or(0);
        if G::Kind::is_undirected() {
            let set: HashSet<_> = edges
                .iter()
                .filter(|&&(u, v)| u != v)
                .map(|&(u, v)| UndirectedEdge::new(u, v))
                .collect();
            let edges = set.into_iter().map(|e| (e.source(), e.target()));
            G::new_with_edges(n, edges)
        } else {
            let set: HashSet<_> = edges.iter().filter(|&&(u, v)| u != v).cloned().collect();
            G::new_with_edges(n, set)
        }
    });
    Box::new(iter)
}

macro_rules! def_random {
    ($(#[$name_meta:meta])* $name:ident,
     $(#[$namev_meta:meta])* $namev:ident,
     $(#[$namee_meta:meta])* $namee:ident,
     $fun:ident) => (
        $(#[$name_meta])*
        #[derive(Clone, Debug)]
        pub struct $name<G>(pub G);

        impl<G> Arbitrary for $name<G>
            where G: Clone + Send + 'static + VertexList + EdgeList + WithBuilder,
                  G::Kind: UniformEdgeKind
        {
            fn arbitrary<Ge: Gen>(gen: &mut Ge) -> Self {
                let s = gen.size();
                let n = gen.gen_range(0, s);
                $name(G::$fun(n, gen))
            }

            fn shrink(&self) -> Box<Iterator<Item = Self>> {
                Box::new(shrink_graph(&self.0).map($name))
            }
        }

        $(#[$namev_meta])*
        #[derive(Clone, Debug)]
        pub struct $namev<G, T>(pub G, pub DefaultVertexPropMut<G, T>)
            where G: WithVertexProp<T>,
                  DefaultVertexPropMut<G, T>: Debug + Clone;

        impl<G, T> Arbitrary for $namev<G, T>
            where G: Clone + Send + 'static + VertexList + EdgeList + WithBuilder,
                  G::Kind: UniformEdgeKind,
                  G: WithVertexProp<T>,
                  DefaultVertexPropMut<G, T>: Debug + Clone + Send + 'static,
                  T: Arbitrary + Default + Clone
        {
            fn arbitrary<Ge: Gen>(g: &mut Ge) -> Self {
                let $name(graph) = $name::<G>::arbitrary(g);
                let prop = graph.default_vertex_prop_from_fn(|_| T::arbitrary(g));
                $namev(graph, prop)
            }

            fn shrink(&self) -> Box<Iterator<Item = Self>> {
                let g = $name(self.0.clone());
                let prop = self.1.clone();
                let iter = g.shrink()
                    .map(move |g| {
                             let prop = g.0.default_vertex_prop_from_fn(|v| prop[v].clone());
                             $namev(g.0, prop)
                         });
                Box::new(iter)
            }
        }

        $(#[$namee_meta])*
        #[derive(Clone, Debug)]
        pub struct $namee<G, T>(pub G, pub DefaultEdgePropMut<G, T>)
            where G: WithEdgeProp<T>,
                  DefaultEdgePropMut<G, T>: Debug + Clone;

        impl<G, T> Arbitrary for $namee<G, T>
            where G: Clone + Send + 'static + VertexList + EdgeList + WithBuilder,
                  G::Kind: UniformEdgeKind,
                  G: WithEdgeProp<T>,
                  DefaultEdgePropMut<G, T>: Debug + Clone + Send + 'static,
                  T: Arbitrary + Default + Clone
        {
            fn arbitrary<Ge: Gen>(g: &mut Ge) -> Self {
                let $name(graph) = $name::<G>::arbitrary(g);
                let prop = graph.default_edge_prop_from_fn(|_| T::arbitrary(g));
                $namee(graph, prop)
            }

            fn shrink(&self) -> Box<Iterator<Item = Self>> {
                let g = $name(self.0.clone());
                let prop = self.1.clone();
                let iter = g.shrink()
                    .map(move |$name(gn)| {
                        let mut propn = gn.default_edge_prop(T::default());
                        for (en, u, v) in gn.edges_with_ends() {
                            if let Some(e) = g.0.get_edge_by_ends(u, v) {
                                propn[en] = prop[e].clone();
                            }
                        }
                        $namee(gn, propn)
                    });
                Box::new(iter)
            }
        }
    )
}

def_random!{
    /// A wrapper to create arbitrary graphs using [`WithBuilder::new_gn`].
    ///
    /// [`WithBuilder::new_gn`]: ../builder/trait.WithBuilder.html#method.new_gn
    Gn,

    /// A wrapper to create arbitrary graphs with a vertex property using [`WithBuilder::new_gn`].
    ///
    /// [`WithBuilder::new_gn`]: ../builder/trait.WithBuilder.html#method.new_gn
    GnWithVertexProp,

    /// A wrapper to create arbitrary graphs with an edge property using [`WithBuilder::new_gn`].
    ///
    /// [`WithBuilder::new_gn`]: ../builder/trait.WithBuilder.html#method.new_gn
    GnWithEdgeProp,

    new_gn
}

def_random!{
    /// A wrapper to create arbitrary graphs using [`WithBuilder::new_gn_connected`].
    ///
    /// [`WithBuilder::new_gn_connected`]: trait.WithBuilder.html#method.new_gn_connected
    GnConnected,

    /// A wrapper to create arbitrary graphs with a vertex property using
    /// [`WithBuilder::new_gn_connected`].
    ///
    /// [`WithBuilder::new_gn_connected`]: ../builder/trait.WithBuilder.html#method.new_gn_connected
    GnConnectedWithVertexProp,

    /// A wrapper to create arbitrary graphs with an edge property using
    /// [`WithBuilder::new_gn_connected`].
    ///
    /// [`WithBuilder::new_gn_connected`]: ../builder/trait.WithBuilder.html#method.new_gn_connected
    GnConnectedWithEdgeProp,
    new_gn_connected
}

// TODO: add CompleteWithVertexProp and CompleteWithVertexProp
impl Arbitrary for CompleteGraph {
    fn arbitrary<G: Gen>(gen: &mut G) -> Self {
        let s = gen.size();
        let n = gen.gen_range(0, s) as u32;
        CompleteGraph::new(n)
    }

    fn shrink(&self) -> Box<Iterator<Item = Self>> {
        let n = self.num_vertices();
        Box::new(n.shrink().map(|n| CompleteGraph::new(n as u32)))
    }
}
