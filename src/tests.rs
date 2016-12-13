use prelude::*;
use props::HashMapProp;

use itertools::{cloned, enumerate};

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
            option_edge,
            ends,
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


pub fn set<I>(iter: I) -> HashSet<I::Item>
    where I: IntoIterator,
          I::Item: Eq + ::std::hash::Hash
{
    iter.into_iter().collect()
}

pub fn vec<I>(iter: I) -> Vec<I::Item>
    where I: IntoIterator
{
    iter.into_iter().collect()
}

pub trait GraphTests {
    type G: WithVertex + WithEdge;

    fn new() -> (Self::G, VecVertex<Self::G>, VecEdge<Self::G>);

    fn new_with_builder() -> (Self::G, VecVertex<Self::G>, VecEdge<Self::G>)
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
                   set(cloned(&vertices)).len(),
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
                   set(cloned(&edges)).len(),
                   "found repeated edges");
        assert_eq!(edges.len(), g.num_edges());
        assert_eq!(edges, vec(g.edges()));
        assert_eq!(vec(edges.iter().map(|e| g.ends(*e))),
                   vec(g.edges().map(|e| g.ends(e))));
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

    fn ends() {
        let (g, _, edges) = Self::new();
        for e in edges {
            let (u, v) = g.ends(e);
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
        for e in edges {
            let (u, v) = g.ends(e);
            if g.is_undirected_edge(e) {
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
        for e in edges {
            let (u, v) = g.ends(e);
            assert_eq!(u, g.opposite(v, e));
            assert_eq!(v, g.opposite(u, e));
        }
    }

    fn out_neighbors()
        where Self::G: Adjacency
    {
        let (g, vertices, edges) = Self::new();
        let mut n = HashMapProp::new(VecVertex::<Self::G>::new());
        let mut d = HashMapProp::new(0usize);
        for e in edges {
            let (u, v) = g.ends(e);
            n[u].push(v);
            d[u] += 1;
            if g.is_undirected_edge(e) {
                n[v].push(u);
                d[v] += 1;
            }
        }
        for u in vertices {
            assert_eq!(set(cloned(&n[u])), set(g.out_neighbors(u)));
            assert_eq!(d[u], g.out_degree(u))
        }
    }

    fn out_edges()
        where Self::G: Incidence
    {
        let (g, vertices, edges) = Self::new();
        let mut inc = HashMapProp::new(VecEdge::<Self::G>::new());
        for e in edges {
            let (u, v) = g.ends(e);
            inc[u].push(e);
            if g.is_undirected_edge(e) {
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
                        g.ends(e));
            }
            assert_eq!(set(cloned(&inc[u])), out);
        }
    }

    fn vertex_prop()
        where Self::G: WithVertexProp<usize>
    {
        let (g, vertices, _) = Self::new();
        let mut p = g.default_vertex_prop(0usize);
        for (i, u) in cloned(&vertices).enumerate() {
            p[u] = 10 * i;
        }
        for (i, u) in enumerate(vertices) {
            assert_eq!(10 * i, p[u])
        }
    }

    fn edge_prop()
        where Self::G: WithEdgeProp<usize>
    {
        let (g, _, edges) = Self::new();
        let mut p = g.default_edge_prop(0usize);
        for (i, e) in cloned(&edges).enumerate() {
            p[e] = i + 1;
        }
        for (i, e) in enumerate(edges) {
            assert_eq!(i + 1, p[e]);
            if g.is_undirected_edge(e) {
                assert_eq!(i + 1, p[g.get_reverse(e).unwrap()])
            }
        }
    }
}
