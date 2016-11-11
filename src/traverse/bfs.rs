use graph::*;
use super::Traverser;
use super::visitor::*;
#[macro_use]
use super::control::*;
use std::collections::VecDeque;

// TODO: make queue generic
pub struct Bfs<'a, G, C>
    where G: 'a + WithEdge,
          C: VertexPropMut<G, Color>
{
    pub g: &'a G,
    pub color: C,
    pub queue: VecDeque<(OptionEdge<G>, Vertex<G>)>,
}

impl<'a, G> Bfs<'a, G, DefaultVertexPropMut<G, Color>>
    where G: 'a + WithEdge + WithVertexProp<Color>
{
    pub fn new(g: &'a G) -> Self {
        Bfs {
            g: g,
            color: g.vertex_prop(Color::White),
            queue: VecDeque::new(),
        }
    }
}

impl<'a, G, C> Traverser<'a, G> for Bfs<'a, G, C>
    where G: 'a + IncidenceGraph,
          C: VertexPropMut<G, Color>
{
    fn traverse<V: Visitor<G>>(&mut self, v: Vertex<G>, mut vis: V) -> bool {
        self.queue.push_back((G::edge_none(), v));
        self.color[v] = Color::Gray;
        return_unless!(vis.discover_vertex(self.g, v));
        while let Some((from, u)) = self.queue.pop_front() {
            for e in self.g.out_edges(u) {
                let v = self.g.target(e);
                if self.g.is_undirected_edge(e) && self.color[v] == Color::Black ||
                   G::edge_some(e) == from {
                    continue;
                }
                return_unless!(vis.discover_edge(self.g, e));
                match self.color[v] {
                    Color::White => {
                        self.color[v] = Color::Gray;
                        self.queue.push_back((e.into(), v));
                        return_unless!(vis.discover_tree_edge(self.g, e));
                        return_unless!(vis.discover_vertex(self.g, v));
                        continue;
                    }
                    Color::Gray => {
                        return_unless!(vis.discover_back_edge(self.g, e));
                    }
                    Color::Black => {
                        return_unless!(vis.discover_cross_or_forward_edge(self.g, e));
                    }
                }
                return_unless!(vis.finish_edge(self.g, e));
            }
            self.color[u] = Color::Black;
            return_unless!(vis.finish_vertex(self.g, u));
            if let Some(from) = from.into_option() {
                return_unless!(vis.finish_tree_edge(self.g, from));
                return_unless!(vis.finish_edge(self.g, from));
            }
        }
        true
    }

    fn traverse_all<V: Visitor<G>>(&mut self, vis: V) {
        self.traverse_vertices(self.g.vertices(), vis);
    }

    fn graph(&self) -> &G {
        self.g
    }

    fn is_discovered(&self, v: Vertex<G>) -> bool {
        self.color[v] != Color::White
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
    fn bfs() {
        use super::super::visitor::TraverseEvent::*;
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
        Bfs::new(&g).traverse_all(FnTraverseEvent(|evt| v.push(evt)));
        assert_eq!(expected, v);
    }
}

#[cfg(all(feature = "nightly", test))]
mod benchs {
    use static_::*;
    use builder::WithBuilder;
    use traverse::*;
    use rand::{SeedableRng, StdRng};
    use test::Bencher;

    fn bench_bfs<'a>(b: &mut Bencher, g: &'a StaticGraph) {
        b.iter(|| {
            Bfs::new(g).traverse_all(DiscoverTreeEdge(|_| Control::Continue));
        });
    }

    #[bench]
    fn bench_bfs_complete(b: &mut Bencher) {
        let g = StaticGraph::complete(100);
        bench_bfs(b, &g);
    }

    #[bench]
    fn bench_bfs_tree(b: &mut Bencher) {
        let g = StaticGraph::random_tree(100, &mut StdRng::from_seed(&[123]));
        bench_bfs(b, &g);
    }
}
