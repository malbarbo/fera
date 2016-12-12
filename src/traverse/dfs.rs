use super::control::*;
use super::visitor::*;

use graph::*;
use params::*;

pub trait Dfs<V: Visitor<Self>>: Incidence {
    fn dfs_with_params<'a, P>(&'a self, params: P, mut vis: V) -> Control
        where Self: DfsWithParams<'a, P>
    {
        return_unless!(vis.start(self));
        use std::borrow::BorrowMut;
        let (mut color, mut stack, roots) = self.dfs_params(params);
        let color = color.borrow_mut();
        let stack = stack.borrow_mut();
        for v in roots {
            if color[v] == Color::White {
                color[v] = Color::Gray;
                stack.push((Self::edge_none(), v, self.out_edges(v)));
                return_unless!(vis.discover_root_vertex(self, v));
                return_unless!(vis.discover_vertex(self, v));
                return_unless!(dfs_visit(self, color, stack, &mut vis));
                return_unless!(vis.finish_root_vertex(self, v));
            }
        }
        vis.finish(self)
    }

    fn dfs(&self, vis: V) -> Control
        where Self: DfsWithDefaultParams
    {
        self.dfs_with_params(DfsParams::new(), vis)
    }

    fn dfs_with_root(&self, root: Vertex<Self>, vis: V) -> Control
        where Self: DfsWithRoot
    {
        use std::iter::once;
        self.dfs_with_params(DfsParams::new().roots(once(root)), vis)
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

impl<G, V> Dfs<V> for G
    where G: Incidence,
          V: Visitor<G>
{
}


// Params

define_param!(DfsParams(color, stack, roots));

impl DfsParams<NewVertexProp<Color>, NewDfsStack, AllVertices> {
    pub fn new() -> Self {
        Default::default()
    }
}

trait_alias!(DfsWithDefaultParams = VertexList + Incidence + WithVertexProp<Color>);

trait_alias!(DfsWithRoot = Incidence + WithVertexProp<Color>);

pub trait DfsWithParams<'a, P>: 'a + WithEdge {
    type Color: ParamVertexProp<Self, Color>;
    type Stack: Param<'a, Self, DfsStack<'a, Self>>;
    type Roots: ParamIterator<'a, Self, Item = Vertex<Self>>;

    fn dfs_params(&'a self, params: P) -> (<Self::Color as ParamVertexProp<Self, Color>>::Output,
                                           <Self::Stack as Param<'a, Self, DfsStack<'a, Self>>>::Output,
                                           <Self::Roots as ParamIterator<'a, Self>>::Output);
}

impl<'a, G, C, S, R> DfsWithParams<'a, DfsParams<C, S, R>> for G
    where G: 'a + WithEdge,
          C: ParamVertexProp<G, Color>,
          S: Param<'a, G, DfsStack<'a, G>>,
          R: ParamIterator<'a, G, Item = Vertex<G>>
{
    type Color = C;
    type Stack = S;
    type Roots = R;

    fn dfs_params(&'a self, p: DfsParams<C, S, R>) -> (C::Output, S::Output, R::Output) {
        (p.0.build(self), p.1.build(self), p.2.build(self))
    }
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
    use graph::*;
    use static_::*;
    use fera::IteratorExt;
    use traverse::*;

    fn new() -> StaticGraph {
        //    1
        //  / | \         4
        // 0  |  3      /   \
        //  \ | /      5 --- 6
        //    2
        graph!(StaticGraph,
               7,
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
        use traverse::visitor::TraverseEvent::*;
        use traverse::recursive_dfs;
        let g = new();
        let v = g.vertices().into_vec();
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
        recursive_dfs(&g, OnTraverseEvent(|evt| v.push(evt)));
        assert_eq!(expected, v);

        v.clear();
        g.dfs(OnTraverseEvent(|evt| v.push(evt)));
        assert_eq!(expected, v);

        // TODO: test recursive dfs vs dfs form random graphs
        // TODO: test each edge and vertex is visited exatly once
    }
}

#[cfg(all(feature = "nightly", test))]
mod benchs {
    use static_::*;
    use builder::WithBuilder;
    use traverse::*;
    use rand::XorShiftRng;
    use test::Bencher;

    fn bench_dfs<'a>(b: &mut Bencher, g: &'a StaticGraph) {
        b.iter(|| {
            g.dfs(OnDiscoverTreeEdge(|_| Control::Continue));
        });
    }

    #[bench]
    fn complete_graph(b: &mut Bencher) {
        let g = StaticGraph::complete(100);
        bench_dfs(b, &g);
    }

    #[bench]
    fn tree(b: &mut Bencher) {
        let g = StaticGraph::random_tree(100, XorShiftRng::new_unseeded());
        bench_dfs(b, &g);
    }
}
