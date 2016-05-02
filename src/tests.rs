use graph::*;
use builder::*;
use hashprop::*;

use fera::IteratorExt;

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

macro_rules! graph_basic_tests {
    ($T: ident) => (
        delegate_tests!{
            $T,
            vertices,
            option_vertex,
            edges,
            option_edge,
            reverse,
            opposite,
            degree,
            inc_edges
        }
    )
}

macro_rules! graph_prop_tests {
    ($T: ident) => (
        delegate_tests!{$T, vertex_prop, edge_prop}
    )
}

macro_rules! graph_adj_tests {
    ($T: ident) => (
        delegate_tests!{$T, neighbors}
    )
}

pub trait GraphTests {
    type G: Graph;

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

    fn vertices() {
        let (g, vertices, _) = Self::new();
        assert_eq!(vertices.len(), g.num_vertices());
        assert_eq!(vertices, g.vertices().into_vec());
    }

    fn option_vertex() {
        let (_, vertices, _) = Self::new();
        assert_eq!(None, Self::G::vertex_none().into_option());
        if vertices.is_empty() {
            return;
        }
        let u = vertices[0];
        assert_eq!(Some(u), Self::G::vertex_some(u).into_option());
    }

    fn edges() {
        let (g, _, edges) = Self::new();
        assert_eq!(edges.len(), g.num_edges());
        assert_eq!(edges, g.edges().into_vec());
        assert_eq!(edges.iter().map(|e| g.ends(*e)).into_vec(),
                   g.edges().map(|e| g.ends(e)).into_vec());
    }

    fn option_edge() {
        let (_, _, edges) = Self::new();
        assert_eq!(None, Self::G::edge_none().into_option());
        if edges.is_empty() {
            return;
        }
        let e = edges[0];
        assert_eq!(Some(e), Self::G::edge_some(e).into_option());
    }

    fn reverse() {
        let (g, _, _) = Self::new();
        for e in g.edges() {
            let (u, v) = g.ends(e);
            assert_eq!(e, g.reverse(e));
            assert_eq!((v, u), g.ends(g.reverse(e)))
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

    fn degree()
        where Self::G: Adjacency
    {
        let (g, _, edges) = Self::new();
        let mut d = HashProp::new(0usize);
        for e in edges {
            let (u, v) = g.ends(e);
            d[u] += 1;
            d[v] += 1;
        }
        for u in g.vertices() {
            assert_eq!(d[u], g.degree(u))
        }
    }

    fn inc_edges()
        where Self::G: Incidence
    {
        let (g, _, edges) = Self::new();
        let mut inc = HashProp::new(VecEdge::<Self::G>::new());
        for e in edges {
            let (u, v) = g.ends(e);
            inc[u].push(e);
            inc[v].push(g.reverse(e));
        }
        for u in g.vertices() {
            for e in g.inc_edges(u) {
                assert_eq!(u, g.source(e));
            }
            assert_eq!(inc[u].iter().cloned().into_hash_set(),
                       g.inc_edges(u).into_hash_set());
        }
    }

    fn vertex_prop()
        where Self::G: WithVertexProp<usize>
    {
        let (g, _, _) = Self::new();
        let mut p = g.vertex_prop(0usize);
        for (i, u) in g.vertices().enumerate() {
            p[u] = 10 * i;
        }
        for (i, u) in g.vertices().enumerate() {
            assert_eq!(10 * i, p[u])
        }
    }

    fn edge_prop()
        where Self::G: WithEdgeProp<usize>
    {
        let (g, _, _) = Self::new();
        let mut p = g.edge_prop(0usize);
        for (i, e) in g.edges().enumerate() {
            p[e] = 10 * i;
        }
        for (i, e) in g.edges().enumerate() {
            assert_eq!(10 * i, p[e]);
            assert_eq!(10 * i, p[g.reverse(e)])
        }
    }

    fn neighbors()
        where Self::G: Adjacency
    {
        let (g, _, edges) = Self::new();
        let mut adj = HashProp::new(VecVertex::<Self::G>::new());
        for e in edges {
            let (u, v) = g.ends(e);
            adj[u].push(v);
            adj[v].push(u);
        }
        for u in g.vertices() {
            assert_eq!(adj[u].iter().cloned().into_hash_set(),
                       g.neighbors(u).into_hash_set());
        }
    }
}
