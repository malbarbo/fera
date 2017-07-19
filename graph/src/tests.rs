use prelude::*;
use props::HashMapProp;

use fera_fun::{enumerate, vec, set};

use std::collections::HashSet;

macro_rules! delegate_tests {
    ($T: ident, $($names: ident),+) => (
        $(
            #[test]
            fn $names() {
                $T::$names();
            }
        )*
    )
}

macro_rules! graph_vertex_list_tests {
    ($T: ident) => (
        delegate_tests!{
            $T,
            vertices,
            option_vertex
        }
    )
}

macro_rules! graph_edge_list_tests {
    ($T: ident) => (
        delegate_tests!{
            $T,
            edges,
            get_edge_by_ends,
            option_edge,
            end_vertices,
            get_reverse,
            opposite
        }
    )
}

macro_rules! graph_incidence_tests {
    ($T: ident) => (
        delegate_tests!{
            $T,
            out_neighbors,
            out_edges
        }
    )
}

macro_rules! graph_prop_tests {
    ($T: ident) => (
        delegate_tests!{
            $T,
            vertex_prop,
            edge_prop
        }
    )
}

macro_rules! graph_tests {
    ($T: ident) => (
        graph_vertex_list_tests!{$T}
        graph_edge_list_tests!{$T}
        graph_incidence_tests!{$T}
        graph_prop_tests!{$T}
    )
}


// TODO: allows Subgraph and &'a G to be tested
pub trait GraphTests {
    type G: WithEdge;

    fn new() -> (Self::G, Vec<Vertex<Self::G>>, Vec<Edge<Self::G>>);

    fn new_with_builder() -> (Self::G, Vec<Vertex<Self::G>>, Vec<Edge<Self::G>>)
        where Self::G: WithBuilder
    {
        let mut b = <Self::G as WithBuilder>::builder(5, 4);
        for &(u, v) in &[(0, 1), (0, 2), (1, 2), (1, 3)] {
            b.add_edge(u, v);
        }
        b.finalize_()
    }

    fn vertices()
        where Self::G: VertexList
    {
        let (g, vertices, _) = Self::new();
        assert_eq!(vertices.len(),
                   set(&vertices).len(),
                   "found repeated vertices");
        assert_eq!(vertices.len(), g.num_vertices());
        assert_eq!(vertices, vec(g.vertices()));
    }

    fn option_vertex() {
        let (_, vertices, _) = Self::new();
        let mut v = Self::G::vertex_none();
        assert_eq!(None, v.to_option_ref());
        assert_eq!(None, v.to_option_mut());
        assert_eq!(None, v.into_option());
        for mut v in vertices {
            let mut vv = Self::G::vertex_some(v);
            assert_eq!(Some(&v), vv.to_option_ref());
            assert_eq!(Some(&mut v), vv.to_option_mut());
            assert_eq!(Some(v), vv.into_option());
        }
    }

    fn edges()
        where Self::G: EdgeList
    {
        let (g, _, edges) = Self::new();
        assert_eq!(edges.len(),
                   set(&edges).len(),
                   "found repeated edges");
        assert_eq!(edges.len(), g.num_edges());
        assert_eq!(edges, vec(g.edges()));
        assert_eq!(vec(g.ends(edges)),
                   vec(g.edges_ends()));
    }

    fn get_edge_by_ends()
        where Self::G: EdgeList
    {
        let (g, _, edges) = Self::new();
        for e in edges {
            let (u, v) = g.ends(e);
            // TODO: test return None
            assert_eq!(e, g.edge_by_ends(u, v));
            assert_eq!(Some(e), g.get_edge_by_ends(u, v));
            if g.orientation(e).is_undirected() {
                assert_eq!(e, g.edge_by_ends(v, u));
                assert_eq!(Some(e), g.get_edge_by_ends(v, u));
            }
        }
    }

    fn option_edge() {
        let (_, _, edges) = Self::new();
        let mut e = Self::G::edge_none();
        assert_eq!(None, e.to_option_ref());
        assert_eq!(None, e.to_option_mut());
        assert_eq!(None, e.into_option());
        for mut e in edges {
            let mut ee = Self::G::edge_some(e);
            assert_eq!(Some(&e), ee.to_option_ref());
            assert_eq!(Some(&mut e), ee.to_option_mut());
            assert_eq!(Some(e), ee.into_option());
        }
    }

    fn end_vertices() {
        let (g, _, edges) = Self::new();
        for e in edges {
            let (u, v) = g.end_vertices(e);
            assert_eq!(u, g.source(e));
            assert_eq!(v, g.target(e));
        }
    }

    fn get_reverse()
        where Self::G: WithEdge
    {
        use std::hash::{Hash, Hasher};
        fn hash<T: Hash>(t: T) -> u64 {
            let mut s = ::std::collections::hash_map::DefaultHasher::new();
            t.hash(&mut s);
            s.finish()
        }
        let (g, _, edges) = Self::new();
        for (e, u, v) in g.with_ends(edges) {
            if g.orientation(e).is_undirected() {
                // FIXME: reverse is not being directly tested
                let r = g.get_reverse(e).unwrap();
                assert_eq!(e, r);
                assert_eq!(hash(e), hash(r));
                assert_eq!(v, g.source(r));
                assert_eq!(u, g.target(r));
                assert_eq!((v, u), g.ends(r));
            }
        }
    }

    fn opposite() {
        let (g, _, edges) = Self::new();
        for (e, u, v) in g.with_ends(edges) {
            assert_eq!(u, g.opposite(v, e));
            assert_eq!(v, g.opposite(u, e));
        }
    }

    fn out_neighbors()
        where Self::G: Adjacency
    {
        let (g, vertices, edges) = Self::new();
        let mut n = HashMapProp::new(Vec::<Vertex<Self::G>>::new());
        let mut d = HashMapProp::new(0usize);
        for (e, u, v) in g.with_ends(edges) {
            n[u].push(v);
            d[u] += 1;
            if g.orientation(e).is_undirected() {
                n[v].push(u);
                d[v] += 1;
            }
        }
        for u in vertices {
            assert_eq!(set(n[u].iter().cloned()), set(g.out_neighbors(u)));
            assert_eq!(d[u], g.out_degree(u))
        }
    }

    fn out_edges()
        where Self::G: Incidence
    {
        let (g, vertices, edges) = Self::new();
        let mut inc = HashMapProp::new(Vec::<Edge<Self::G>>::new());
        for (e, u, v) in g.with_ends(edges) {
            inc[u].push(e);
            if g.orientation(e).is_undirected() {
                inc[v].push(g.get_reverse(e).unwrap());
            }
        }
        for u in vertices {
            let mut out = HashSet::new();
            for e in g.out_edges(u) {
                assert_eq!(u, g.source(e));
                assert!(out.insert(e),
                        "found repeated out edge = {:?} = {:?}",
                        e,
                        g.end_vertices(e));
            }
            assert_eq!(set(inc[u].iter().cloned()), out);
        }
    }

    fn vertex_prop()
        where Self::G: WithVertexProp<usize>
    {
        let (g, vertices, _) = Self::new();
        let mut p = g.default_vertex_prop(0usize);
        for (i, &u) in enumerate(&vertices) {
            p[u] = 10 * i;
        }
        for (i, &u) in enumerate(&vertices) {
            assert_eq!(10 * i, p[u])
        }
    }

    fn edge_prop()
        where Self::G: WithEdgeProp<usize>
    {
        let (g, _, edges) = Self::new();
        let mut p = g.default_edge_prop(0usize);
        for (i, &e) in enumerate(&edges) {
            p[e] = i + 1;
        }
        for (i, &e) in enumerate(&edges) {
            assert_eq!(i + 1, p[e]);
            if g.orientation(e).is_undirected() {
                assert_eq!(i + 1, p[g.get_reverse(e).unwrap()])
            }
        }
    }
}
