// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use params::*;
use prelude::*;
use props::{Color, IgnoreWriteProp};
use traverse::*;

use std::iter;

pub trait RecursiveDfs: WithEdge {
    fn recursive_dfs<V>(
        &self,
        vis: V,
    ) -> RecursiveDfsAlg<&Self, V, AllVertices<Self>, NewVertexProp<Self, Color>>
    where
        V: Visitor<Self>,
    {
        RecursiveDfsAlg(
            self,
            vis,
            AllVertices(self),
            NewVertexProp(self, Color::White),
        )
    }
}

impl<G: WithEdge> RecursiveDfs for G {}

generic_struct! {
    #[must_use = "call .run() to execute the algorithm"]
    pub struct RecursiveDfsAlg(graph, visitor, roots, color)
}

impl<'a, G, V, R, C> RecursiveDfsAlg<&'a G, V, R, C> {
    pub fn run(self) -> Control
    where
        G: Incidence,
        V: Visitor<G>,
        R: IntoIterator<Item = Vertex<G>>,
        C: ParamDerefMut,
        C::Target: VertexPropMut<G, Color>,
    {
        let RecursiveDfsAlg(g, mut vis, roots, color) = self;
        return_unless!(vis.start(g));
        let mut color = color.build();
        for v in roots {
            if color[v] == Color::White {
                color[v] = Color::Gray;
                return_unless!(vis.discover_root_vertex(g, v));
                return_unless!(recursive_dfs_visit(
                    g,
                    G::edge_none(),
                    v,
                    &mut *color,
                    &mut vis
                ));
                return_unless!(vis.finish_root_vertex(g, v));
            }
        }
        vis.finish(g)
    }

    pub fn root(self, root: Vertex<G>) -> RecursiveDfsAlg<&'a G, V, iter::Once<Vertex<G>>, C>
    where
        G: WithVertex,
    {
        self.roots(iter::once(root))
    }

    pub fn ignore_color_changes(self) -> RecursiveDfsAlg<&'a G, V, R, Owned<IgnoreWriteProp<Color>>>
    where
        G: WithVertex,
    {
        let color = Owned(self.0.vertex_prop(Color::White));
        self.color(color)
    }
}

pub fn recursive_dfs_visit<G, C, V>(
    g: &G,
    from: OptionEdge<G>,
    u: Vertex<G>,
    color: &mut C,
    vis: &mut V,
) -> Control
where
    G: Incidence,
    C: VertexPropMut<G, Color>,
    V: Visitor<G>,
{
    color[u] = Color::Gray;
    return_unless!(vis.discover_vertex(g, u));
    for e in g.out_edges(u) {
        let v = g.target(e);
        if g.orientation(e).is_undirected() && color[v] == Color::Black || G::edge_some(e) == from {
            continue;
        }
        return_unless!(vis.discover_edge(g, e));
        match color[v] {
            Color::White => {
                return_unless!(vis.discover_tree_edge(g, e));
                return_unless!(recursive_dfs_visit(g, e.into(), v, color, vis));
                return_unless!(vis.finish_tree_edge(g, e));
            }
            Color::Gray => {
                return_unless!(vis.discover_back_edge(g, e));
            }
            Color::Black => {
                return_unless!(vis.discover_cross_or_forward_edge(g, e));
            }
        }
        return_unless!(vis.finish_edge(g, e));
    }
    color[u] = Color::Black;
    return_unless!(vis.finish_vertex(g, u));
    Control::Continue
}
