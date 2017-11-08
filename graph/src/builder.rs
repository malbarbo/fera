// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Builder to create user defined, standard and random graphs.
//!
//! This module offers an abstract way (independent of the graph type) to create graphs. To support
//! graph building a graph type must implement the [`WithBuilder`] trait, which requires defining a
//! type that implements the [`Builder`] trait. Each graph type has its own vertex and edge
//! representation, but the [`Builder`] trait works with numeric vertices.
//!
//! # Examples
//!
//! Creating a literal graph:
//!
//! ```
//! use fera_graph::prelude::*;
//!
//! // Creates a graph builder for a graph with 4 vertices and initial capacity for 5 edges
//! let mut builder = StaticGraph::builder(4, 5);
//! builder.add_edge(0, 1);
//! builder.add_edge(1, 2);
//! builder.add_edge(1, 3);
//! builder.add_edge(2, 3);
//! builder.add_edge(3, 0);
//! // Note that we can add more than 5 edges
//! builder.add_edge(2, 0);
//!
//! let g = builder.finalize();
//! assert_eq!(4, g.num_vertices());
//! assert_eq!(6, g.num_edges());
//! ```
//!
//! The [`graph!`] macro can be used to simplify the creation of literal graphs.
//!
//! Creating standard graphs (see also [`Complete`]):
//!
//! ```
//! use fera_graph::prelude::*;
//!
//! // Creates a complete graph with 4 vertices
//! let g = StaticGraph::new_complete(4);
//! assert_eq!(4, g.num_vertices());
//! assert_eq!(6, g.num_edges());
//! let edges = [(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)];
//! assert!(edges.iter().all(|&(u, v)| g.get_edge_by_ends(u, v).is_some()));
//! ```
//!
//! Creating random graphs:
//!
//! ```
//! extern crate rand;
//! extern crate fera_graph;
//!
//! use fera_graph::prelude::*;
//! use fera_graph::algs::{Components, Trees};
//!
//! # fn main() {
//! let mut rng = rand::weak_rng();
//!
//! // Creates a connected graph with 10 vertices
//! let g = StaticGraph::new_gn_connected(10, &mut rng);
//! assert!(g.is_connected());
//!
//! // Creates a graph with 7 vertices that is a tree.
//! let g = StaticGraph::new_random_tree(7, &mut rng);
//! assert!(g.is_tree());
//! # }
//! ```
//!
//! See [`WithBuilder`] for other methods to create standard and random graphs.
//!
//! [`graph!`]: ../macro.graph.html
//! [`Builder`]: trait.Builder.html
//! [`Complete`]: ../graphs/complete/index.html
//! [`WithBuilder`]: trait.WithBuilder.html

use prelude::*;
use algs::{Components, Trees};
use props::Color;
use sets::FastVecSet;

use std::cmp;
use std::mem;

use fera_fun::set;
use rand::{Rng, XorShiftRng};
use rand::distributions::{IndependentSample, Range};

/// Creates a new graph with `n` vertices and the specified edges.
///
/// The type of the graph that will be created needs to be specified. Any graph that implements
/// [`Builder`] can be used. There are two forms of this macro:
///
/// - Creates a graph with a list of edges:
///
/// ```
/// #[macro_use] extern crate fera_graph;
/// use fera_graph::prelude::*;
///
/// # fn main() {
/// let g: StaticGraph = graph!{
///     4,
///     (0, 2),
///     (0, 3),
///     (1, 2)
/// };
///
/// assert_eq!(4, g.num_vertices());
/// assert_eq!(3, g.num_edges());
///
/// for u in 0..4 {
///     for v in 0..4 {
///         if [(0, 2), (2, 0), (0, 3), (3, 0), (1, 2), (2, 1)].contains(&(u, v)) {
///             assert_eq!(g.end_vertices(g.edge_by_ends(u, v)), (u, v));
///         } else {
///             assert_eq!(None, g.get_edge_by_ends(u, v));
///         }
///     }
/// }
/// # }
/// ```
///
/// - Creates a graph with a list of edges and an associated property:
///
/// ```
/// #[macro_use] extern crate fera_graph;
/// use fera_graph::prelude::*;
/// use fera_graph::sum_prop;
///
/// # fn main() {
/// let (g, w): (StaticGraph, _) = graph!{
///     4,
///     (0, 2) -> 4,
///     (0, 3) -> 2,
///     (1, 2) -> 3
/// };
///
/// assert_eq!(9, sum_prop(&w, g.edges()));
/// # }
/// ```
///
/// In this case, a [`DefaultEdgePropMut`] is created.
///
/// [`Builder`]: builder/trait.Builder.html
/// [`DefaultEdgePropMut`]: graphs/type.DefaultEdgePropMut.html
// TODO: move to macros.rs
// TODO: create a vertex_prop macro
#[macro_export]
macro_rules! graph {
    () => (
        {
            use $crate::builder::WithBuilder;
            WithBuilder::new_empty(0)
        }
    );

    ($n:expr) => (
        {
            use $crate::builder::WithBuilder;
            WithBuilder::new_empty($n)
        }
    );

    ($n:expr, $(($u:expr, $v:expr)),+) => (
        {
            let edges = [$(($u, $v)),*];
            $crate::builder::WithBuilder::new_with_edges($n, edges.iter().cloned())
        }
    );

    ($n:expr, $(($u:expr, $v:expr)),+,) => (
        graph!($n, $(($u, $v)),+)
    );

    ($n:expr, $(($u:expr, $v:expr) -> $p:expr),+) => (
        {
            let edges = [$(($u, $v, $p)),*];
            $crate::builder::WithBuilder::new_with_edges_prop($n, &edges)
        }
    );

    ($n:expr, $(($u:expr, $v:expr) -> $p:expr),+,) => (
        graph!($n, $(($u, $v) -> $p),+)
    );
}


// TODO: rename to GraphBuilder
/// A builder used to build graphs.
///
/// See the [module documentation] for examples.
///
/// [module documentation]: index.html
pub trait Builder {
    /// The graph type produced by this builder.
    type Graph: WithEdge;

    /// Creates a new builder for a graph with exactly `n` vertices and initial capacity for `m`
    /// edges.
    ///
    /// This method is generally called through [`WithBuilder::builder`], for example,
    /// `StaticGraph::builder(10, 26)`.
    ///
    /// [`WithBuilder::builder`]: trait.WithBuilder.html#method.builder
    fn new(n: usize, m: usize) -> Self;

    /// Add `(u, v)` edge to the graph. Support for multiple edges and loops are graph dependent.
    ///
    /// # Panics
    ///
    /// If `u` or `v` is not a valid vertex, that is `>= num_vertices`.
    fn add_edge(&mut self, u: usize, v: usize);

    /// Builds the graph.
    fn finalize(self) -> Self::Graph;

    #[doc(hidden)]
    fn finalize_(self) -> (Self::Graph, Vec<Vertex<Self::Graph>>, Vec<Edge<Self::Graph>>);
}

/// A graph that has a [`Builder`].
///
/// See the [module documentation] for examples.
///
/// [`Builder`]: trait.Builder.html
/// [module documentation]: index.html
// TODO: ex: G::new().complete(5), G::new_with_rng(rng).random_tree(10)
pub trait WithBuilder: WithEdge {
    /// The builder for this graph type.
    type Builder: Builder<Graph = Self>;

    /// Creates a new builder for a graph of this type with `n` vertices and initial capacity for
    /// `m` edges.
    fn builder(num_vertices: usize, num_edges: usize) -> Self::Builder {
        Self::Builder::new(num_vertices, num_edges)
    }

    /// Creates a new graph with `n` vertices from `edges` iterator.
    ///
    /// # Panics
    ///
    /// If some edges is not valid.
    fn new_with_edges<I>(n: usize, edges: I) -> Self
        where I: IntoIterator<Item = (usize, usize)>
    {
        let edges = edges.into_iter();
        let mut b = Self::Builder::new(n, edges.size_hint().1.unwrap_or(0));
        for (u, v) in edges {
            b.add_edge(u, v);
        }
        b.finalize()
    }

    #[doc(hidden)]
    fn new_with_edges_prop<T>(n: usize,
                              edges: &[(usize, usize, T)])
                              -> (Self, DefaultEdgePropMut<Self, T>)
        where T: Copy + Default,
              Self: WithEdgeProp<T>
    {
        // TODO: Should this be optimized?
        let mut b = Self::Builder::new(n, edges.len());
        for &(ref u, ref v, _) in edges {
            b.add_edge(*u, *v);
        }
        let (g, _, ee) = b.finalize_();
        let mut p = g.default_edge_prop(T::default());
        for (e, val) in ee.into_iter().zip(edges.iter().map(|x| x.2)) {
            p[e] = val;
        }
        (g, p)
    }

    /// Creates a graph with `n` vertices and no edges.
    fn new_empty(n: usize) -> Self {
        Self::Builder::new(n, 0).finalize()
    }

    /// Creates a complete graph with `n` vertices.
    ///
    /// A complete graph has an edge between each pair of vertices.
    fn new_complete(n: usize) -> Self
        where Self: WithEdge<Kind = Undirected>
    {
        complete::<Self>(n).finalize()
    }

    /// Creates a graph that is a complete binary tree with height `h`.
    ///
    /// In complete binary tree all interior vertices have two children an all leaves have the
    /// same depth.
    fn new_complete_binary_tree(h: u32) -> Self
        where Self: WithEdge<Kind = Undirected>
    {
        complete_binary_tree::<Self>(h).finalize()
    }

    /// Creates a graph with `n` vertices that is a tree, that is, is connected and acyclic.
    ///
    /// The graph has `n - 1` edges if `n > 0` or zero edges if `n = 0`.
    ///
    /// See <https://doi.org/10.1109/SFCS.1989.63516>.
    fn new_random_tree<R: Rng>(n: usize, rng: R) -> Self {
        random_tree::<Self, _>(n, rng).finalize()
    }

    /// Similar to [`new_random_tree`] but creates a tree with diameter `d`. Returns `None`
    /// if the diameter is invalid.
    // TODO: describe what is a invalid diameter.
    fn new_random_tree_with_diameter<R: Rng>(n: u32, d: u32, rng: R) -> Option<Self> {
        random_tree_with_diameter::<Self, _>(n, d, rng).map(Builder::finalize)
    }

    /// Creates a random graph with `n` vertices.
    fn new_gn<R>(n: usize, mut rng: R) -> Self
        where Self::Kind: UniformEdgeKind,
              R: Rng
    {
        let m = if n > 1 {
            rng.gen_range(0, max_num_edges::<Self>(n))
        } else {
            0
        };
        Self::new_gnm(n, m, rng).unwrap()
    }

    /// Creates a random connected graph with `n` vertices.
    fn new_gn_connected<R: Rng>(n: usize, mut rng: R) -> Self
        where Self::Kind: UniformEdgeKind
    {
        let m = max_num_edges::<Self>(n);
        let m = if m > n {
            rng.gen_range(n, m)
        } else {
            cmp::min(n, m)
        };
        Self::new_gnm_connected(n, m, rng).unwrap()
    }

    /// Creates a random graph with `n` vertices and `m` edges.
    ///
    /// Returns `None` with `m` exceeds the maximum number of edges.
    fn new_gnm<R>(n: usize, m: usize, rng: R) -> Option<Self>
        where Self::Kind: UniformEdgeKind,
              R: Rng
    {
        gnm::<Self, _>(n, m, rng).map(Builder::finalize)
    }

    /// Creates a random connected graph (weakly connected if `Self` is a digraph) with `n`
    /// vertices and `m` edges.
    ///
    /// Returns `None` if `m` exceeds the maximum number of edges or if `m` is less than `n - 1`.
    fn new_gnm_connected<R: Rng>(n: usize, m: usize, rng: R) -> Option<Self>
        where Self::Kind: UniformEdgeKind
    {
        gnm_connected::<Self, _>(n, m, rng).map(Builder::finalize)
    }
}

fn complete<G: WithBuilder>(n: usize) -> G::Builder {
    let mut b = G::builder(n, (n * n - n) / 2);
    for u in 0..n {
        for v in u + 1..n {
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

fn random_tree<G, R>(n: usize, rng: R) -> G::Builder
    where G: WithBuilder,
          R: Rng
{
    if n == 0 {
        return G::builder(0, 0);
    }
    let mut b = G::builder(n, n - 1);
    for (u, v) in RandomTreeIter::new(n, rng) {
        b.add_edge(u, v);
    }
    b
}

fn random_tree_with_diameter<G, R>(n: u32, d: u32, mut rng: R) -> Option<G::Builder>
    where G: WithBuilder,
          R: Rng
{
    if n == 0 {
        return if d == 0 { Some(G::builder(0, 0)) } else { None };
    }

    if d > n - 1 || n > 2 && d < 2 {
        return None;
    }

    let mut b = G::builder(n as usize, (n - 1) as usize);
    let mut vertices: Vec<_> = (0..n).collect();
    let mut visited = vec![false; n as usize];
    let mut maxd = vec![0; n as usize];
    // create a complete graph so we can use graph properties
    let g = CompleteGraph::new(n);
    let mut dist = g.default_edge_prop(0);
    let mut num_edges = 0;

    // create the initial path
    rng.shuffle(&mut vertices);
    vertices.truncate(d as usize + 1);
    for i in 0..vertices.len() - 1 {
        for j in (i + 1)..vertices.len() {
            let (u, v) = (vertices[i], vertices[j]);
            let duv = (j - i) as u32;
            dist[g.edge_by_ends(u, v)] = duv;
            maxd[u as usize] = maxd[u as usize].max(duv);
            maxd[v as usize] = maxd[v as usize].max(duv);
        }
        b.add_edge(vertices[i] as usize, vertices[i + 1] as usize);
        num_edges += 1;
    }

    if num_edges == n - 1 {
        return Some(b);
    }

    // compute good
    let mut good = FastVecSet::new_vertex_set(&g);
    for &v in &vertices {
        visited[v as usize] = true;
    }
    for v in 0..n {
        if maxd[v as usize] != d {
            good.insert(v);
        }
    }

    // complete the tree
    let mut cur = *good.choose(&mut rng).unwrap();
    while !visited[cur as usize] {
        cur = *good.choose(&mut rng).unwrap();
    }
    while num_edges != n - 1 {
        // choose a new edge
        if !good.contains(cur) {
            cur = *good.choose(&mut rng).unwrap();
            while !visited[cur as usize] {
                cur = *good.choose(&mut rng).unwrap();
            }
        }
        let v = *good.choose(&mut rng).unwrap();
        if visited[v as usize] || cur == v {
            cur = v;
            continue;
        }
        assert!(maxd[cur as usize] < d);

        // update dist
        for i in 0..n {
            if i == cur {
                continue;
            }
            let dci = dist[g.edge_by_ends(cur, i)];
            if dci != 0 && v != i {
                dist[g.edge_by_ends(v, i)] = dci + 1;
            }
        }

        // update maxd and good
        maxd[v as usize] = maxd[cur as usize] + 1;
        if maxd[v as usize] == d {
            good.remove(v);
        }
        for i in 0..n {
            if v == i {
                continue;
            }
            maxd[i as usize] = maxd[i as usize].max(dist[g.edge_by_ends(v, i)]);
            if maxd[i as usize] == d && good.contains(i) {
                good.remove(i);
            }
            assert!(maxd[i as usize] <= d);
        }

        // update tree
        b.add_edge(cur as usize, v as usize);
        num_edges += 1;
        visited[v as usize] = true;

        // iterate
        cur = v;
    }

    Some(b)
}

fn max_num_edges<G>(n: usize) -> usize
    where G: WithEdge,
          G::Kind: UniformEdgeKind
{
    if G::Kind::is_directed() {
        n * n
    } else {
        (n * n - n) / 2
    }
}

fn gnm_connected<G, R>(n: usize, m: usize, mut rng: R) -> Option<G::Builder>
    where G: WithBuilder,
          G::Kind: UniformEdgeKind,
          R: Rng
{
    use std::collections::HashSet;

    if n == 0 {
        return Some(G::builder(0, 0));
    }

    if m > max_num_edges::<G>(n) || m < n - 1 {
        return None;
    }

    let mut b = G::builder(n, m);
    let mut set = HashSet::new();
    for (u, v) in RandomTreeIter::new(n, &mut rng) {
        set.insert((u, v));
        b.add_edge(u, v)
    }

    while set.len() != m {
        let u = rng.gen_range(0, n);
        let v = rng.gen_range(0, n);
        if u == v || set.contains(&(u, v)) || G::Kind::is_undirected() && set.contains(&(v, u)) {
            continue;
        }
        set.insert((u, v));
        b.add_edge(u, v)
    }

    Some(b)
}

fn gnm<G, R>(n: usize, m: usize, mut rng: R) -> Option<G::Builder>
    where G: WithBuilder,
          G::Kind: UniformEdgeKind,
          R: Rng
{
    use std::collections::HashSet;

    if m > max_num_edges::<G>(n) {
        return None;
    }

    let mut b = G::builder(n, m);
    let mut set = HashSet::new();
    while set.len() != m {
        let u = rng.gen_range(0, n);
        let v = rng.gen_range(0, n);
        if u == v || set.contains(&(u, v)) || G::Kind::is_undirected() && set.contains(&(v, u)) {
            continue;
        }
        set.insert((u, v));
        b.add_edge(u, v)
    }

    Some(b)
}


// Iterator

struct RandomTreeIter<R> {
    visited: Vec<bool>,
    rem: usize,
    rng: R,
    range: Range<usize>,
    cur: usize,
}

impl<R: Rng> RandomTreeIter<R> {
    fn new(n: usize, mut rng: R) -> Self {
        let range = Range::new(0, n);
        let cur = range.ind_sample(&mut rng);
        let mut visited = vec![false; n];
        visited[cur] = true;
        RandomTreeIter {
            visited,
            rem: n.checked_sub(1).unwrap_or(0),
            rng,
            range,
            cur,
        }
    }
}

impl<R: Rng> Iterator for RandomTreeIter<R> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.rem == 0 {
            return None;
        }
        loop {
            let v = self.range.ind_sample(&mut self.rng);
            if self.visited[v] {
                self.cur = v;
            } else {
                self.rem -= 1;
                self.visited[v] = true;
                let u = mem::replace(&mut self.cur, v);
                return Some((u, v));
            }
        }
    }
}


// Tests

#[doc(hidden)]
pub trait BuilderTests {
    type G: WithBuilder + VertexList + EdgeList;

    fn graph_macro() {
        let g: Self::G = graph!(
            5,
            (1, 2),
            (4, 0),
        );
        assert_eq!(5, g.num_vertices());
        assert_eq!(2, g.num_edges());
    }

    fn graph_prop_macro()
        where Self::G: WithEdgeProp<u32>
    {
        let (g, w): (Self::G, _) = graph!(
            5,
            (1, 2) -> 3,
            (4, 0) -> 4,
        );
        assert_eq!(5, g.num_vertices());
        assert_eq!(2, g.num_edges());
        let mut sum = 0;
        for e in g.edges() {
            sum += w[e];
        }
        assert_eq!(7, sum);
    }

    fn complete() {
        let (g, v, e) = complete::<Self::G>(3).finalize_();
        assert_eq!((v[0], v[1]), g.ends(e[0]));
        assert_eq!((v[0], v[2]), g.ends(e[1]));
        assert_eq!((v[1], v[2]), g.ends(e[2]));

        for (n, &m) in (0..5).zip(&[0, 0, 1, 3, 6, 10]) {
            let (g, v, _) = complete::<Self::G>(n).finalize_();
            assert_eq!(n, g.num_vertices());
            assert_eq!(m, g.num_edges());
            assert_eq!(set(v), set(g.vertices()));
        }
    }

    #[cfg_attr(feature = "cargo-clippy", allow(needless_range_loop))]
    fn complete_binary_tree()
        where Self::G: Incidence + WithVertexProp<Color>
    {
        let (g, _, _) = complete_binary_tree::<Self::G>(0).finalize_();
        assert_eq!(1, g.num_vertices());
        assert_eq!(0, g.num_edges());

        let (g, v, _) = complete_binary_tree::<Self::G>(1).finalize_();
        assert_eq!(3, g.num_vertices());
        assert_eq!(2, g.num_edges());
        assert_eq!(set(vec![(v[0], v[1]), (v[0], v[2])]),
                   set(g.out_edges_ends(v[0])));

        for h in 2..10 {
            let (g, v, _) = complete_binary_tree::<Self::G>(h).finalize_();
            assert!(g.is_tree());
            assert_eq!(2, g.out_degree(v[0]));
            for i in 1..g.num_vertices() / 2 - 1 {
                assert_eq!(3, g.out_degree(v[i]));
            }
            for i in (g.num_vertices() / 2)..g.num_vertices() {
                assert_eq!(1, g.out_degree(v[i]));
            }
        }
    }

    fn random_tree()
        where Self::G: Incidence + WithVertexProp<Color>
    {
        let mut rng = XorShiftRng::new_unseeded();
        for n in 0..100 {
            for _ in 0..10 {
                let g = Self::G::new_random_tree(n, &mut rng);
                assert_eq!(n, g.num_vertices());
                if n > 0 {
                    assert_eq!(n - 1, g.num_edges());
                }
                assert!(g.is_tree());
            }
        }
    }

    fn gnm()
        where Self::G: WithEdge + VertexList + EdgeList,
              <Self::G as WithEdge>::Kind: UniformEdgeKind
    {
        let mut rng = XorShiftRng::new_unseeded();

        assert!(Self::G::new_gnm(4, 20, &mut rng).is_none());

        for n in 0..10 {
            for m in 0..30 {
                if let Some(g) = Self::G::new_gnm(n, m, &mut rng) {
                    assert_eq!(n, g.num_vertices());
                    assert_eq!(m, g.num_edges());
                }
            }
        }
    }

    fn gnm_connected()
        where Self::G: Incidence + WithVertexProp<Color>,
              <Self::G as WithEdge>::Kind: UniformEdgeKind
    {
        let mut rng = XorShiftRng::new_unseeded();

        assert!(Self::G::new_gnm_connected(4, 20, &mut rng).is_none());
        assert!(Self::G::new_gnm_connected(4, 2, &mut rng).is_none());

        for n in 1..10 {
            for m in (n - 1)..30 {
                if let Some(g) = Self::G::new_gnm_connected(n, m, &mut rng) {
                    assert!(g.is_connected());
                    assert_eq!(n, g.num_vertices());
                    assert_eq!(m, g.num_edges());
                }
            }
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! graph_builder_tests {
    ($T: ident) => (
        delegate_tests!{
            $T,
            graph_macro,
            graph_prop_macro,
            complete,
            complete_binary_tree,
            random_tree,
            gnm,
            gnm_connected
        }
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand;

    #[test]
    fn random_tree_mean_diameter() {
        let mut rng = rand::weak_rng();
        let n = 100;
        let times = 1000;
        let sum: Result<usize, _> = (0..times)
            .map(|_| StaticGraph::new_random_tree(n, &mut rng).tree_diameter())
            .sum();
        assert_eq!(28, sum.unwrap() / times);
    }
}
