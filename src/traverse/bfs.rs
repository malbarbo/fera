use super::control::*;
use super::visitor::*;

use graph::*;
use params::*;

pub trait Bfs<V: Visitor<Self>>: Incidence {
    fn bfs_with_params<'a, P>(&'a self, params: P, mut vis: V)
        where Self: BfsWithParams<'a, P>
    {
        use std::borrow::BorrowMut;
        let (mut color, mut queue, roots) = self.bfs_params(params);
        let color = color.borrow_mut();
        let queue = queue.borrow_mut();
        for v in roots {
            if color[v] == Color::White {
                color[v] = Color::Gray;
                queue.push_back((Self::edge_none(), v));
                break_unless!(vis.discover_root_vertex(self, v));
                break_unless!(vis.discover_vertex(self, v));
                if !bfs_visit(self, color, queue, &mut vis) {
                    break;
                }
                break_unless!(vis.finish_root_vertex(self, v));
            }
        }
    }

    fn bfs(&self, vis: V)
        where Self: BfsWithDefaultParams
    {
        self.bfs_with_params(BfsParams::new(), vis);
    }

    fn bfs_with_root(&self, root: Vertex<Self>, vis: V)
        where Self: BfsWithRoot
    {
        use std::iter::once;
        self.bfs_with_params(BfsParams::new().roots(once(root)), vis);
    }
}

pub fn bfs_visit<G, C, V>(g: &G, color: &mut C, queue: &mut BfsQueue<G>, vis: &mut V) -> bool
    where G: Incidence,
          C: VertexPropMut<G, Color>,
          V: Visitor<G>
{
    while let Some((from, u)) = queue.pop_front() {
        for e in g.out_edges(u) {
            let v = g.target(e);
            if g.is_undirected_edge(e) && color[v] == Color::Black || G::edge_some(e) == from {
                continue;
            }
            return_unless!(vis.discover_edge(g, e));
            match color[v] {
                Color::White => {
                    color[v] = Color::Gray;
                    queue.push_back((e.into(), v));
                    return_unless!(vis.discover_tree_edge(g, e));
                    return_unless!(vis.discover_vertex(g, v));
                    continue;
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
    true
}

impl<G, V> Bfs<V> for G
    where G: Incidence,
          V: Visitor<G>
{
}


// Params

define_param!(BfsParams(color, queue, roots));

impl BfsParams<NewVertexProp<Color>, NewBfsQueue, AllVertices> {
    pub fn new() -> Self {
        Default::default()
    }
}

trait_alias!(BfsWithDefaultParams = VertexList + Incidence + WithVertexProp<Color>);

trait_alias!(BfsWithRoot = Incidence + WithVertexProp<Color>);

pub trait BfsWithParams<'a, P>: 'a + WithEdge {
    type Color: ParamVertexProp<Self, Color>;
    type Queue: Param<'a, Self, BfsQueue<Self>>;
    type Roots: ParamVertexIter<'a, Self>;

    fn bfs_params(&'a self, params: P) -> (<Self::Color as ParamVertexProp<Self, Color>>::Output,
                                           <Self::Queue as Param<'a, Self, BfsQueue<Self>>>::Output,
                                           <Self::Roots as ParamVertexIter<'a, Self>>::Output);
}

impl<'a, G, C, S, R> BfsWithParams<'a, BfsParams<C, S, R>> for G
    where G: 'a + WithEdge,
          C: ParamVertexProp<G, Color>,
          S: Param<'a, G, BfsQueue<G>>,
          R: ParamVertexIter<'a, G>
{
    type Color = C;
    type Queue = S;
    type Roots = R;

    fn bfs_params(&'a self, p: BfsParams<C, S, R>) -> (C::Output, S::Output, R::Output) {
        (p.0.build(self), p.1.build(self), p.2.build(self))
    }
}

pub type BfsQueue<G> = ::std::collections::VecDeque<(OptionEdge<G>, Vertex<G>)>;

#[derive(Default)]
pub struct NewBfsQueue;

impl<'a, G: 'a + WithEdge> Param<'a, G, BfsQueue<G>> for NewBfsQueue {
    type Output = BfsQueue<G>;

    fn build(self, _g: &'a G) -> Self::Output {
        BfsQueue::<G>::new()
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
        use traverse::TraverseEvent::*;
        let g = new();
        let v = g.vertices().into_vec();
        let e = |x: usize, y: usize| edge_by_ends(&g, v[x], v[y]);
        let expected = vec![
            DiscoverRootVertex(0),
            DiscoverVertex(0),
            DiscoverEdge(e(0, 1)),
            DiscoverTreeEdge(e(0, 1)),
            DiscoverVertex(1),
            DiscoverEdge(e(0, 2)),
            DiscoverTreeEdge(e(0, 2)),
            DiscoverVertex(2),
            FinishVertex(0),
            DiscoverEdge(e(1, 2)),
            DiscoverBackEdge(e(1, 2)),
            FinishEdge(e(1, 2)),
            DiscoverEdge(e(1, 3)),
            DiscoverTreeEdge(e(1, 3)),
            DiscoverVertex(3),
            FinishVertex(1),
            FinishTreeEdge(e(0, 1)),
            FinishEdge(e(0, 1)),
            DiscoverEdge(e(2, 3)),
            DiscoverBackEdge(e(2, 3)),
            FinishEdge(e(2, 3)),
            FinishVertex(2),
            FinishTreeEdge(e(0, 2)),
            FinishEdge(e(0, 2)),
            FinishVertex(3),
            FinishTreeEdge(e(1, 3)),
            FinishEdge(e(1, 3)),
            FinishRootVertex(0),

            DiscoverRootVertex(4),
            DiscoverVertex(4),
            DiscoverEdge(e(4, 5)),
            DiscoverTreeEdge(e(4, 5)),
            DiscoverVertex(5),
            DiscoverEdge(e(4, 6)),
            DiscoverTreeEdge(e(4, 6)),
            DiscoverVertex(6),
            FinishVertex(4),
            DiscoverEdge(e(5, 6)),
            DiscoverBackEdge(e(5, 6)),
            FinishEdge(e(5, 6)),
            FinishVertex(5),
            FinishTreeEdge(e(4, 5)),
            FinishEdge(e(4, 5)),
            FinishVertex(6),
            FinishTreeEdge(e(4, 6)),
            FinishEdge(e(4, 6)),
            FinishRootVertex(4),
        ];

        let mut v = vec![];
        g.bfs(OnTraverseEvent(|evt| v.push(evt)));
        assert_eq!(expected, v);
    }
}

#[cfg(all(feature = "nightly", test))]
mod benchs {
    use static_::*;
    use builder::WithBuilder;
    use traverse::*;
    use rand::XorShiftRng;
    use test::Bencher;

    fn bench_bfs<'a>(b: &mut Bencher, g: &'a StaticGraph) {
        b.iter(|| {
            g.bfs(OnDiscoverTreeEdge(|_| Control::Continue));
        });
    }

    #[bench]
    fn complete_graph(b: &mut Bencher) {
        let g = StaticGraph::complete(100);
        bench_bfs(b, &g);
    }

    #[bench]
    fn tree(b: &mut Bencher) {
        let g = StaticGraph::random_tree(100, XorShiftRng::new_unseeded());
        bench_bfs(b, &g);
    }
}
