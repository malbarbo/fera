use graph::*;

pub trait Builder: Sized {
    type Graph: Basic;

    fn add_edge(&mut self, u: usize, v: usize);

    fn finalize(self) -> Self::Graph;

    fn finalize_(self) -> (Self::Graph, VecVertex<Self::Graph>, VecEdge<Self::Graph>);
}

pub trait WithBuilder: Basic {
    type Builder: Builder<Graph=Self>;

    fn builder(num_vertices: usize, num_edges: usize) -> Self::Builder;

    fn complete(n: usize) -> Self {
        complete::<Self>(n).finalize()
    }

    fn complete_binary_tree(height: u32) -> Self {
        complete_binary_tree::<Self>(height).finalize()
    }
}

pub fn complete<G: WithBuilder>(n: usize) -> G::Builder {
    if n == 0 {
        return G::builder(0, 0)
    }
    let mut b = G::builder(n, n * (n - 1) / 2);
    for u in 0..n {
        for v in u+1..n {
            b.add_edge(u, v);
        }
    }
    b
}

fn complete_binary_tree<G: WithBuilder>(height: u32) -> G::Builder {
    let num_vertices = 2usize.pow(height + 1) - 1;
    let mut b = G::builder(num_vertices, num_vertices - 1);
    for i in 0..2usize.pow(height) - 1 {
        b.add_edge(i, 2 * i + 1);
        b.add_edge(i, 2 * i + 2);
    }
    b
}


// Tests

#[cfg(test)]
mod tests {
    use super::*;
    use graph::*;
    use static_::*;
    use iter::*;
    use props::*;

    #[test]
    fn test_complete() {
        let (g, v, e) = complete::<StaticGraph>(3).finalize_();
        assert_eq!((v[0], v[1]), g.endvertices(e[0]));
        assert_eq!((v[0], v[2]), g.endvertices(e[1]));
        assert_eq!((v[1], v[2]), g.endvertices(e[2]));

        for (n, &e) in (0..5).zip(&[0, 0, 1, 3, 6, 10]) {
            let g = StaticGraph::complete(n);
            assert_eq!(n, g.num_vertices());
            assert_eq!(e, g.num_edges());
        }
    }
    #[test]
    fn test_complete_binary_tree() {
        let g = StaticGraph::complete_binary_tree(0);
        assert_eq!(1, g.num_vertices());
        assert_eq!(0, g.num_edges());

        let g = StaticGraph::complete_binary_tree(1);
        assert_eq!(3, g.num_vertices());
        assert_eq!(2, g.num_edges());
        assert_eq!(set![1, 2], g.neighbors(0).into_set());

        for h in 2..10 {
            let g = StaticGraph::complete_binary_tree(h);
            assert!(g.is_tree());
            assert_eq!(2, g.degree(0));
            for v in 1..g.num_vertices() / 2 - 1 {
                assert_eq!(3, g.degree(v));
            }
            for v in (g.num_vertices() / 2)..g.num_vertices() {
                assert_eq!(1, g.degree(v));
            }
        }
    }
}
