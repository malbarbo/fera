use prelude::*;
use traverse::*;

use fera_fun::first;

use std::cmp::min;
use std::marker::PhantomData;

// FIXME: restrict the method to appropiated graph type
pub trait Components: Incidence {
    fn num_components(&self) -> u64
        where Self: DfsDefault
    {
        let mut num = 0;
        self.dfs(NumComponents(&mut num));
        num
    }

    fn connected_components(&self) -> ConnectedComponents<Self, DefaultVertexPropMut<Self, usize>>
        where Self: DfsDefault + WithVertexProp<usize>
    {
        let mut cc = ConnectedComponents(self, self.vertex_prop(0));
        self.dfs(&mut cc);
        cc
    }

    fn is_connected(&self) -> bool
        where Self: DfsDefault
    {
        let mut con = true;
        self.dfs(IsConnected(&mut con));
        con
    }

    fn cut_vertices(&self) -> VecVertex<Self>
        where Self: Graph
    {
        if self.num_vertices() == 0 {
            return vec![];
        }
        let mut vis = FindCutVertices {
            time: 0,
            discover: self.vertex_prop(0),
            low: self.vertex_prop(0),
            root: first(self.vertices()),
            root_childs: 0,
            is_cut: self.vertex_prop(false),
        };
        self.dfs(&mut vis);
        self.vertices().filter(|&v| vis.is_cut[v]).collect()
    }

    fn cut_edges(&self) -> VecEdge<Self>
        where Self: Graph
    {
        let mut vis = FindCutEdges {
            time: 0,
            discover: self.vertex_prop(0),
            low: self.vertex_prop(0),
            cuts: vec![],
        };
        self.dfs(&mut vis);
        vis.cuts
    }
}

impl<G: Incidence> Components for G {}


pub struct IsConnected<'a> {
    connected: &'a mut bool,
    saw_root: bool,
}

#[allow(non_snake_case)]
pub fn IsConnected(con: &mut bool) -> IsConnected {
    IsConnected {
        connected: con,
        saw_root: false,
    }
}

impl<'a, G: WithEdge> Visitor<G> for IsConnected<'a> {
    fn start(&mut self, _g: &G) -> Control {
        *self.connected = true;
        self.saw_root = false;
        Control::Continue
    }

    fn discover_root_vertex(&mut self, _g: &G, _v: Vertex<G>) -> Control {
        if self.saw_root {
            *self.connected = false;
            Control::Break
        } else {
            self.saw_root = true;
            Control::Continue
        }
    }
}


#[allow(non_snake_case)]
pub fn NumComponents(num: &mut u64) -> OnDiscoverRootVertex<Count> {
    OnDiscoverRootVertex(Count(num))
}

pub struct ConnectedComponents<G, V> {
    comp: V,
    cur: usize,
    _marker: PhantomData<G>,
}

#[allow(non_snake_case)]
pub fn ConnectedComponents<G, V>(_g: &G, comp: V) -> ConnectedComponents<G, V> {
    ConnectedComponents {
        comp: comp,
        cur: 0,
        _marker: PhantomData,
    }
}

impl<G, V> Visitor<G> for ConnectedComponents<G, V>
    where G: WithEdge,
          V: VertexPropMut<G, usize>
{
    fn discover_root_vertex(&mut self, _g: &G, _v: Vertex<G>) -> Control {
        self.cur += 1;
        Control::Continue
    }

    fn discover_vertex(&mut self, _g: &G, v: Vertex<G>) -> Control {
        self.comp[v] = self.cur;
        Control::Continue
    }
}

impl<G, V> ConnectedComponents<G, V>
    where G: WithEdge,
          V: VertexPropMut<G, usize>
{
    pub fn is_connected(&self, u: Vertex<G>, v: Vertex<G>) -> bool {
        self.comp[u] == self.comp[v]
    }
}


pub struct FindCutVertices<G: Graph> {
    time: u64,
    discover: DefaultVertexPropMut<G, u64>,
    low: DefaultVertexPropMut<G, u64>,
    root: Vertex<G>,
    root_childs: u64,
    is_cut: DefaultVertexPropMut<G, bool>,
}

impl<G: Graph> Visitor<G> for FindCutVertices<G> {
    fn discover_root_vertex(&mut self, _g: &G, v: Vertex<G>) -> Control {
        self.root = v;
        self.root_childs = 0;
        Control::Continue
    }

    fn finish_root_vertex(&mut self, _g: &G, v: Vertex<G>) -> Control {
        if self.root_childs > 1 {
            self.is_cut[v] = true;
        }
        Control::Continue
    }

    fn discover_vertex(&mut self, _g: &G, v: Vertex<G>) -> Control {
        self.discover[v] = self.time;
        self.low[v] = self.time;
        self.time += 1;
        Control::Continue
    }

    fn discover_tree_edge(&mut self, g: &G, e: Edge<G>) -> Control {
        if self.root == g.source(e) {
            self.root_childs += 1;
        }
        Control::Continue
    }

    fn discover_back_edge(&mut self, g: &G, e: Edge<G>) -> Control {
        let (u, v) = g.ends(e);
        if self.discover[v] != self.discover[u] + 1 {
            // v is not the dfs parent of u
            self.low[u] = min(self.low[u], self.discover[v]);
        }
        Control::Continue
    }

    fn finish_tree_edge(&mut self, g: &G, e: Edge<G>) -> Control {
        let (u, v) = g.ends(e);
        self.low[u] = min(self.low[u], self.low[v]);
        if self.root != u && self.low[v] >= self.discover[u] {
            self.is_cut[u] = true;
        }
        Control::Continue
    }
}


pub struct FindCutEdges<G: Graph> {
    time: u64,
    discover: DefaultVertexPropMut<G, u64>,
    low: DefaultVertexPropMut<G, u64>,
    cuts: Vec<Edge<G>>,
}

impl<G: Graph> Visitor<G> for FindCutEdges<G> {
    fn discover_vertex(&mut self, _g: &G, v: Vertex<G>) -> Control {
        self.discover[v] = self.time;
        self.low[v] = self.time;
        self.time += 1;
        Control::Continue
    }

    fn discover_back_edge(&mut self, g: &G, e: Edge<G>) -> Control {
        let (v, parent) = g.ends(e);
        if self.discover[parent] != self.discover[v] + 1 {
            // this is not an edge to parent
            self.low[v] = min(self.low[v], self.discover[parent]);
        }
        Control::Continue
    }

    fn finish_tree_edge(&mut self, g: &G, e: Edge<G>) -> Control {
        let (u, v) = g.ends(e);
        self.low[u] = min(self.low[u], self.low[v]);
        if self.low[v] > self.discover[u] {
            self.cuts.push(e)
        }
        Control::Continue
    }
}


#[doc(hidden)]
pub fn cut_vertices_naive<G: IncidenceGraph>(g: &G) -> Vec<Vertex<G>> {
    g.vertices().filter(|&v| is_cut_vertex_naive(g, v)).collect()
}

fn is_cut_vertex_naive<G: IncidenceGraph>(g: &G, v: Vertex<G>) -> bool {
    let vertices = g.vertices().filter(|&u| u != v).collect();
    let sub = g.induced_subgraph(vertices);
    sub.num_components() > g.num_components()
}

#[doc(hidden)]
pub fn cut_edges_naive<G: IncidenceGraph>(g: &G) -> Vec<Edge<G>> {
    g.edges().filter(|&e| is_cut_edge_naive(g, e)).collect()
}

fn is_cut_edge_naive<G: IncidenceGraph>(g: &G, e: Edge<G>) -> bool {
    let sub = g.spanning_subgraph(g.edges().filter(|&f| f != e));
    sub.num_components() > g.num_components()
}

#[cfg(test)]
mod tests {
    use prelude::*;
    use super::{Components, cut_vertices_naive, cut_edges_naive};

    fn sorted<T: Ord>(mut v: Vec<T>) -> Vec<T> {
        v.sort();
        v
    }

    #[test]
    fn cut_vertices() {
        // Examples from

        // http://www.geeksforgeeks.org/articulation-points-or-cut-vertices-in-a-graph/
        // 1 --- 0 --- 3
        //  \   /      |
        //    2        4
        let g = graph!(StaticGraph, 5, (0, 1), (0, 2), (0, 3), (1, 2), (3, 4));
        let exp = vec![0, 3];
        assert_eq!(exp, sorted(cut_vertices_naive(&g)));
        assert_eq!(exp, sorted(g.cut_vertices()));

        // 0 -- 1 -- 2 -- 3
        let g = graph!(StaticGraph, 4, (0, 1), (1, 2), (2, 3));
        let exp = vec![1, 2];
        assert_eq!(exp, sorted(cut_vertices_naive(&g)));
        assert_eq!(exp, sorted(g.cut_vertices()));

        // 0       3
        // | \   /   \
        // |   1      5
        // | / | \   /
        // 2   6   4
        let g = graph!(StaticGraph,
                       7,
                       (0, 1),
                       (0, 2),
                       (1, 2),
                       (1, 3),
                       (1, 4),
                       (1, 6),
                       (3, 5),
                       (4, 5));
        let exp = vec![1];
        assert_eq!(exp, sorted(cut_vertices_naive(&g)));
        assert_eq!(exp, sorted(g.cut_vertices()));
    }

    fn ends<G: Graph>(g: &G, edges: Vec<Edge<G>>) -> Vec<(Vertex<G>, Vertex<G>)> {
        edges.into_iter().map(|e| g.ends(e)).collect()
    }

    #[test]
    fn cut_edges() {
        // Examples from
        // http://www.geeksforgeeks.org/bridge-in-a-graph/

        // 1 --- 0 --- 3
        //  \   /      |
        //    2        4
        let g = graph!(StaticGraph, 5, (0, 1), (0, 2), (0, 3), (1, 2), (3, 4));
        let exp = vec![(0, 3), (3, 4)];
        assert_eq!(exp, sorted(ends(&g, cut_edges_naive(&g))));
        assert_eq!(exp, sorted(ends(&g, g.cut_edges())));

        // 0 -- 1 -- 2 -- 3
        let g = graph!(StaticGraph, 4, (0, 1), (1, 2), (2, 3));
        let exp = vec![(0, 1), (1, 2), (2, 3)];
        assert_eq!(exp, sorted(ends(&g, cut_edges_naive(&g))));
        assert_eq!(exp, sorted(ends(&g, g.cut_edges())));

        // 0       3
        // | \   /   \
        // |   1      5
        // | / | \   /
        // 2   6   4
        let g = graph!(StaticGraph,
                       7,
                       (0, 1),
                       (0, 2),
                       (1, 2),
                       (1, 3),
                       (1, 4),
                       (1, 6),
                       (3, 5),
                       (4, 5));
        let exp = vec![(1, 6)];
        assert_eq!(exp, sorted(ends(&g, cut_edges_naive(&g))));
        assert_eq!(exp, sorted(ends(&g, g.cut_edges())));
    }
}
