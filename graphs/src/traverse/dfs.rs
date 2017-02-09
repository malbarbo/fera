use prelude::*;
use props::Color;
use traverse::*;
use params::*;

use std::borrow::BorrowMut;
use std::iter;

trait_alias!(DfsWithRoot = Incidence + WithVertexProp<Color>);
trait_alias!(DfsDefault = Incidence + VertexList + WithVertexProp<Color>);

pub trait Dfs: WithEdge {
    fn dfs<V>(&self, vis: V) -> Control
        where Self: DfsDefault,
              V: Visitor<Self>
    {
        self.dfs_()
            .visitor(vis)
            .run()
    }

    fn dfs_with_root<V>(&self, root: Vertex<Self>, vis: V) -> Control
        where Self: DfsWithRoot,
              V: Visitor<Self>
    {
        self.dfs_()
            .visitor(vis)
            .root(root)
            .run()
    }

    fn dfs_(&self) -> DfsAlg<&Self, EmptyVisitor, AllVertices, NewVertexProp<Color>, NewDfsStack> {
        DfsAlg(self,
               EmptyVisitor,
               AllVertices,
               NewVertexProp(Color::White),
               NewDfsStack)
    }
}

impl<G: WithEdge> Dfs for G {}


generic_struct!(DfsAlg(graph, visitor, roots, color, stack));

impl<'a, G, V, R, C, S> DfsAlg<&'a G, V, R, C, S> {
    pub fn run(self) -> Control
        where G: Incidence,
              V: Visitor<G>,
              R: ParamIterator<'a, G, Item = Vertex<G>>,
              C: ParamVertexProp<G, Color>,
              S: Param<'a, G, DfsStack<'a, G>>
    {
        let DfsAlg(g, mut vis, roots, color, stack) = self;
        return_unless!(vis.start(g));
        let mut color = color.build(g);
        let color = color.borrow_mut();
        let mut stack = stack.build(g);
        let stack = stack.borrow_mut();
        for v in roots.build(g) {
            if color[v] == Color::White {
                color[v] = Color::Gray;
                stack.push((G::edge_none(), v, g.out_edges(v)));
                return_unless!(vis.discover_root_vertex(g, v));
                return_unless!(vis.discover_vertex(g, v));
                return_unless!(dfs_visit(g, color, stack, &mut vis));
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
            if g.is_undirected_edge(e) && color[v] == Color::Black || G::edge_some(e) == from {
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

#[derive(Default)]
pub struct NewDfsStack;

impl<'a, G: 'a + WithEdge> Param<'a, G, DfsStack<'a, G>> for NewDfsStack {
    type Output = DfsStack<'a, G>;

    fn build(self, _g: &'a G) -> Self::Output {
        DfsStack::<G>::new()
    }
}


// Tests

#[cfg(test)]
mod tests {
    use prelude::*;
    use traverse::*;
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

    fn edge_by_ends(g: &StaticGraph,
                    u: Vertex<StaticGraph>,
                    v: Vertex<StaticGraph>)
                    -> Edge<StaticGraph> {
        for e in g.edges() {
            let (x, y) = g.ends(e);
            if u == x && v == y {
                return e;
            } else if u == y && v == x {
                return g.reverse(e);
            }
        }
        panic!()
    }

    #[test]
    fn events() {
        use traverse::TraverseEvent::*;
        use traverse::RecursiveDfs;
        let g = new();
        let v = vec(g.vertices());
        let e = |x: usize, y: usize| edge_by_ends(&g, v[x], v[y]);
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

            Finish,
        ];

        let mut v = vec![];
        g.recursive_dfs(OnTraverseEvent(|evt| v.push(evt)));
        assert_eq!(expected, v);

        v.clear();
        g.dfs(OnTraverseEvent(|evt| v.push(evt)));
        assert_eq!(expected, v);

        // TODO: test recursive dfs vs dfs for random graphs
        // TODO: test each edge and vertex is visited exatly once
    }
}
