// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use prelude::*;
use props::{Color, IgnoreWriteProp};
use traverse::*;
use params::*;

use std::iter;

pub trait Dfs: WithEdge {
    fn dfs<V>
        (&self,
         vis: V)
         -> DfsAlg<&Self, V, AllVertices<Self>, NewVertexProp<Self, Color>, Owned<DfsStack<Self>>>
        where V: Visitor<Self>
    {
        DfsAlg(self,
               vis,
               AllVertices(self),
               NewVertexProp(self, Color::White),
               Owned(DfsStack::<Self>::new()))
    }
}

impl<G: WithEdge> Dfs for G {}

generic_struct! {
    #[must_use = "call .run() to execute the algorithm"]
    pub struct DfsAlg(graph, visitor, roots, color, stack)
}

impl<'a, G, V, R, C, S> DfsAlg<&'a G, V, R, C, S> {
    pub fn run(self) -> Control
        where G: Incidence,
              V: Visitor<G>,
              R: IntoIterator<Item = Vertex<G>>,
              C: ParamDerefMut,
              C::Target: VertexPropMut<G, Color>,
              S: ParamDerefMut<Target = DfsStack<'a, G>>
    {
        let DfsAlg(g, mut vis, roots, color, stack) = self;
        return_unless!(vis.start(g));
        let mut color = color.build();
        let mut stack = stack.build();
        for v in roots {
            if color[v] == Color::White {
                color[v] = Color::Gray;
                stack.push((G::edge_none(), v, g.out_edges(v)));
                return_unless!(vis.discover_root_vertex(g, v));
                return_unless!(vis.discover_vertex(g, v));
                return_unless!(dfs_visit(g, &mut *color, &mut *stack, &mut vis));
                return_unless!(vis.finish_root_vertex(g, v));
            }
        }
        vis.finish(g)
    }

    pub fn root(self, root: Vertex<G>) -> DfsAlg<&'a G, V, iter::Once<Vertex<G>>, C, S>
        where G: WithVertex
    {
        self.roots(iter::once(root))
    }

    pub fn ignore_color_changes(self) -> DfsAlg<&'a G, V, R, Owned<IgnoreWriteProp<Color>>, S>
        where G: WithVertex
    {
        let color = Owned(self.0.vertex_prop(Color::White));
        self.color(color)
    }
}

pub fn dfs_visit<'a, G, C, V>(g: &'a G,
                              color: &mut C,
                              stack: &mut DfsStack<'a, G>,
                              vis: &mut V)
                              -> Control
    where G: Incidence,
          C: VertexPropMut<G, Color>,
          V: Visitor<G>
{
    'out: while let Some((from, u, mut inc)) = stack.pop() {
        while let Some(e) = inc.next() {
            let v = g.target(e);
            if g.orientation(e).is_undirected() && color[v] == Color::Black || G::edge_some(e) == from {
                continue;
            }
            return_unless!(vis.discover_edge(g, e));
            match color[v] {
                Color::White => {
                    color[v] = Color::Gray;
                    stack.push((from, u, inc));
                    stack.push((e.into(), v, g.out_edges(v)));
                    return_unless!(vis.discover_tree_edge(g, e));
                    return_unless!(vis.discover_vertex(g, v));
                    continue 'out;
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
        if let Some(from) = from.into_option() {
            return_unless!(vis.finish_tree_edge(g, from));
            return_unless!(vis.finish_edge(g, from));
        }
    }
    Control::Continue
}


pub type DfsStack<'a, G> = Vec<(OptionEdge<G>, Vertex<G>, OutEdgeIter<'a, G>)>;


// Tests

#[cfg(test)]
mod tests {
    use prelude::*;
    use traverse::*;
    use traverse::TraverseEvent::*;
    use fera_fun::vec;

    fn new() -> StaticGraph {
        //    1
        //  / | \         4
        // 0  |  3      /   \
        //  \ | /      5 --- 6
        //    2
        graph!(7,
               (0, 1),
               (0, 2),
               (1, 2),
               (1, 3),
               (2, 3),

               (4, 5),
               (4, 6),
               (5, 6))
    }

    #[test]
    fn events() {
        let g = new();
        let v = vec(g.vertices());
        let e = |x: usize, y: usize| g.edge_by_ends(v[x], v[y]);
        let expected = vec![
            Start,

            DiscoverRootVertex(0),
            DiscoverVertex(0),
            DiscoverEdge(e(0, 1)),
            DiscoverTreeEdge(e(0, 1)),
            DiscoverVertex(1),
            DiscoverEdge(e(1, 2)),
            DiscoverTreeEdge(e(1, 2)),
            DiscoverVertex(2),
            DiscoverEdge(e(2, 0)),
            DiscoverBackEdge(e(2, 0)),
            FinishEdge(e(2, 0)),
            DiscoverEdge(e(2, 3)),
            DiscoverTreeEdge(e(2, 3)),
            DiscoverVertex(3),
            DiscoverEdge(e(3, 1)),
            DiscoverBackEdge(e(3, 1)),
            FinishEdge(e(3, 1)),
            FinishVertex(3),
            FinishTreeEdge(e(2, 3)),
            FinishEdge(e(2, 3)),
            FinishVertex(2),
            FinishTreeEdge(e(1, 2)),
            FinishEdge(e(1, 2)),
            FinishVertex(1),
            FinishTreeEdge(e(0, 1)),
            FinishEdge(e(0, 1)),
            FinishVertex(0),
            FinishRootVertex(0),

            DiscoverRootVertex(4),
            DiscoverVertex(4),
            DiscoverEdge(e(4, 5)),
            DiscoverTreeEdge(e(4, 5)),
            DiscoverVertex(5),
            DiscoverEdge(e(5, 6)),
            DiscoverTreeEdge(e(5, 6)),
            DiscoverVertex(6),
            DiscoverEdge(e(6, 4)),
            DiscoverBackEdge(e(6, 4)),
            FinishEdge(e(6, 4)),
            FinishVertex(6),
            FinishTreeEdge(e(5, 6)),
            FinishEdge(e(5, 6)),
            FinishVertex(5),
            FinishTreeEdge(e(4, 5)),
            FinishEdge(e(4, 5)),
            FinishVertex(4),
            FinishRootVertex(4),

            Finish
        ];

        let mut v = vec![];
        g.recursive_dfs(OnTraverseEvent(|evt| v.push(evt))).run();
        assert_eq!(expected, v);

        v.clear();
        g.dfs(OnTraverseEvent(|evt| v.push(evt))).run();
        assert_eq!(expected, v);
    }
}
