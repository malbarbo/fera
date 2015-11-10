pub use ds::{Map1, IteratorExt, VecExt};
pub use iter::IteratorGraphExt;

// TODO: Rewrite tests using Builder

macro_rules! create_case {
    ($B:ident) => {
        $B::new(5, &[(0, 1), (0, 2), (1, 2), (1, 3)])
    }
}

#[macro_export]
macro_rules! test_basic {
    ($B:ident) => {
        #[test] fn vertices(){
            let (g, vertices, _) = create_case!{$B};
            assert_eq!(5, vertices.len());
            assert_eq!(5, g.num_vertices());
            assert_eq!(vertices, g.vertices().into_vec());
        }

        #[test] fn edges() {
            let (g, v, edges) = create_case!{$B};
            assert_eq!(4, edges.len());
            assert_eq!(4, g.num_edges());
            assert_eq!(edges, g.edges().into_vec());
            assert_eq!(vec![(v[0], v[1]), (v[0], v[2]), (v[1], v[2]), (v[1], v[3])],
                       g.edges().endvertices(&g).into_vec());
        }

        #[test] fn reverse() {
            let (g, _, edges) = create_case!{$B};
            for e in edges {
                let (u, v) = g.endvertices(e);
                assert_eq!(e, g.reverse(e));
                assert_eq!((v, u), g.endvertices(g.reverse(e)))
            }
        }

        #[test] fn opposite() {
            let (g, _, edges) = create_case!{$B};
            for e in edges {
                let (u, v) = g.endvertices(e);
                assert_eq!(u, g.opposite(v, e));
                assert_eq!(v, g.opposite(u, e));
            }
        }
    }
}

#[macro_export]
macro_rules! test_degree {
    ($B:ident) => {
        #[test] fn degree() {
            let (g, v, edges) = create_case!{$B};
            assert_eq!(4, edges.len());
            assert_eq!(4, g.num_edges());
            assert_eq!(edges, g.edges().into_vec());
            assert_eq!(vec![(v[0], v[1]), (v[0], v[2]), (v[1], v[2]), (v[1], v[3])],
                       g.edges().endvertices(&g).into_vec());
        }
    }
}

#[macro_export]
macro_rules! test_inc {
    ($B:ident) => {
        #[test] fn inc_edges_one_edge() {
            let (g, v, _) = $B::new(2, &[(0, 1)]);
            let e = g.edges().next().unwrap();
            let ab = g.inc_edges(v[0]).next().unwrap();
            let ba = g.inc_edges(v[1]).next().unwrap();
            assert_eq!(e, ab);
            assert_eq!(e, ba);
            assert_eq!(ab, ba);
            assert_eq!(v[0], g.source(ab));
            assert_eq!(v[1], g.target(ab));
            assert_eq!(v[1], g.source(ba));
            assert_eq!(v[0], g.target(ba));
        }

        #[test] fn inc_edges() {
            let (g, v, e) = create_case!{$B};
            assert_eq!(set![e[0], e[1]],
                       g.inc_edges(v[0]).into_set());
            assert_eq!(set![e[0], e[2], e[3]],
                       g.inc_edges(v[1]).into_set());
            assert_eq!(set![e[1], e[2]],
                       g.inc_edges(v[2]).into_set());
            assert_eq!(set![e[3]],
                       g.inc_edges(v[3]).into_set());
            assert_eq!(set![],
                       g.inc_edges(v[4]).into_set());
        }
    }
}

#[macro_export]
macro_rules! test_adj {
    ($B:ident) => {
        #[test] fn neighbors() {
            let (g, v, _) = create_case!{$B};
            assert_eq!(set![v[1], v[2]],
                       g.neighbors(v[0]).into_set());
            assert_eq!(set![v[0], v[2], v[3]],
                       g.neighbors(v[1]).into_set());
            assert_eq!(set![v[0], v[1]],
                       g.neighbors(v[2]).into_set());
            assert_eq!(set![v[1]],
                       g.neighbors(v[3]).into_set());
            assert_eq!(set![],
                       g.neighbors(v[4]).into_set());
        }
    }
}

#[macro_export]
macro_rules! test_vertex_prop {
    ($B:ident) => {
        #[test] fn vertex_prop() {
            let (g, v, _) = create_case!{$B};
            let mut x = g.vertex_prop(0usize);
            let mut y = g.vertex_prop("a");
            let (a, b, c, d, e) = (v[0], v[1], v[2], v[3], v[4]);
            x[c] = 8;
            y[d] = "b";
            assert_eq!(0, x[a]);
            assert_eq!(0, x[b]);
            assert_eq!(8, x[c]);
            assert_eq!(0, x[d]);
            assert_eq!(0, x[e]);
            assert_eq!("a", y[a]);
            assert_eq!("a", y[b]);
            assert_eq!("a", y[c]);
            assert_eq!("b", y[d]);
            assert_eq!("a", y[e]);
        }
    }
}

#[macro_export]
macro_rules! test_edge_prop {
    ($B:ident) => {
        #[test] fn edge_prop() {
            let (g, _, e) = create_case!{$B};
            let mut x = g.edge_prop(0usize);
            let mut y = g.edge_prop("a");
            let (a, b, c, d) = (e[0], e[1], e[2], e[3]);
            x[c] = 8;
            y[d] = "b";
            assert_eq!(0, x[a]);
            assert_eq!(0, x[b]);
            assert_eq!(8, x[c]);
            assert_eq!(0, x[d]);
            assert_eq!("a", y[a]);
            assert_eq!("a", y[b]);
            assert_eq!("a", y[c]);
            assert_eq!("b", y[d]);
        }
    }
}
