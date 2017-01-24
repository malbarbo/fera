use prelude::*;
use props::Color;
use traverse::*;
use params::*;

use std::borrow::BorrowMut;
use std::iter;

trait_alias!(RecursiveDfsDefault = VertexList + Incidence + WithVertexProp<Color>);
trait_alias!(RecursiveDfsWithRoot = Incidence + WithVertexProp<Color>);

pub trait RecursiveDfs: WithEdge {
    fn recursive_dfs<'a, V>(&'a self, vis: V) -> Control
        where Self: RecursiveDfsDefault,
              V: Visitor<Self>
    {
        self.recursive_dfs_()
            .visitor(vis)
            .run()
    }

    fn recursive_dfs_with_root<V>(&self, root: Vertex<Self>, vis: V) -> Control
        where Self: RecursiveDfsWithRoot,
              V: Visitor<Self>
    {
        self.recursive_dfs_()
            .visitor(vis)
            .root(root)
            .run()
    }

    fn recursive_dfs_
        (&self)
         -> RecursiveDfsAlg<&Self, EmptyVisitor, AllVertices, NewVertexProp<Color>> {
        RecursiveDfsAlg(self, EmptyVisitor, AllVertices, NewVertexProp(Color::White))
    }
}

impl<G: WithEdge> RecursiveDfs for G {}


generic_struct!(RecursiveDfsAlg(graph, visitor, roots, color));

impl<'a, G, V, R, C> RecursiveDfsAlg<&'a G, V, R, C> {
    pub fn run(self) -> Control
        where G: Incidence,
              V: Visitor<G>,
              R: ParamIterator<'a, G, Item = Vertex<G>>,
              C: ParamVertexProp<G, Color>
    {
        let RecursiveDfsAlg(g, mut vis, roots, color) = self;
        return_unless!(vis.start(g));
        let mut color = color.build(g);
        let color = color.borrow_mut();
        for v in roots.build(g) {
            if color[v] == Color::White {
                color[v] = Color::Gray;
                return_unless!(vis.discover_root_vertex(g, v));
                return_unless!(recursive_dfs_visit(g, G::edge_none(), v, color, &mut vis));
                return_unless!(vis.finish_root_vertex(g, v));
            }
        }
        vis.finish(g)
    }

    pub fn root(self, root: Vertex<G>) -> RecursiveDfsAlg<&'a G, V, iter::Once<Vertex<G>>, C>
        where G: WithVertex
    {
        self.roots(iter::once(root))
    }
}

pub fn recursive_dfs_visit<G, C, V>(g: &G,
                                    from: OptionEdge<G>,
                                    u: Vertex<G>,
                                    color: &mut C,
                                    vis: &mut V)
                                    -> Control
    where G: Incidence,
          C: VertexPropMut<G, Color>,
          V: Visitor<G>
{
    color[u] = Color::Gray;
    return_unless!(vis.discover_vertex(g, u));
    for e in g.out_edges(u) {
        let v = g.target(e);
        if g.is_undirected_edge(e) && color[v] == Color::Black || G::edge_some(e) == from {
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
