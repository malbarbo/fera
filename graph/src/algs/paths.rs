// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Paths related algorithms, including find path between two vertices.

use params::IntoOwned;
use prelude::*;
use props::Color;
use traverse::*;

pub trait Paths: Incidence {
    fn find_path(&self, u: Vertex<Self>, v: Vertex<Self>) -> Option<Vec<Edge<Self>>>
    where
        Self: WithVertexProp<Color>,
    {
        if u == v {
            return None;
        }
        let mut path = vec![];
        self.dfs(RecordPath(&mut path, v)).root(u).run();
        if path.is_empty() {
            None
        } else {
            Some(path)
        }
    }

    fn is_walk<I>(&self, edges: I) -> bool
    where
        I: IntoIterator,
        I::Item: IntoOwned<Edge<Self>>,
    {
        let mut edges = edges.into_iter();
        let mut last = if let Some(e) = edges.next() {
            self.target(e.into_owned())
        } else {
            return true;
        };

        edges.all(|e| {
            let (u, v) = self.ends(e.into_owned());
            if last == u {
                last = v;
                true
            } else {
                false
            }
        })
    }

    fn is_path<I>(&self, edges: I) -> bool
    where
        Self: WithVertexProp<bool>,
        I: IntoIterator,
        I::Item: IntoOwned<Edge<Self>>,
    {
        let mut visited = self.default_vertex_prop(false);
        let mut edges = edges.into_iter();

        let mut last = if let Some(e) = edges.next() {
            let (u, v) = self.ends(e.into_owned());
            if u == v {
                return false;
            }
            visited[u] = true;
            visited[v] = true;
            v
        } else {
            return true;
        };

        edges.all(|e| {
            let (u, v) = self.ends(e.into_owned());

            if last != u || visited[v] {
                false
            } else {
                visited[v] = true;
                last = v;
                true
            }
        })
    }

    fn is_path_graph(&self) -> bool
    where
        Self: VertexList + EdgeList,
    {
        self.num_vertices() == 1
            || self.num_edges() + 1 == self.num_vertices()
                && self
                    .vertices()
                    .filter(|&v| self.out_degree(v) == 1)
                    .next()
                    .map(|mut prev| {
                        let mut count = 1;
                        let mut cur = self.out_neighbors(prev).next().unwrap();
                        while let Some(v) = self.out_neighbors(cur).filter(|&u| prev != u).next() {
                            count += 1;
                            prev = cur;
                            cur = v;
                        }
                        count == self.num_edges()
                    })
                    .unwrap_or(false)
    }
}

impl<G> Paths for G where G: Incidence {}

pub struct RecordPath<'a, G: WithEdge> {
    path: &'a mut Vec<Edge<G>>,
    target: Vertex<G>,
}

#[allow(non_snake_case)]
pub fn RecordPath<G>(path: &mut Vec<Edge<G>>, target: Vertex<G>) -> RecordPath<G>
where
    G: WithEdge,
{
    RecordPath {
        path: path,
        target: target,
    }
}

impl<'a, G: WithEdge> Visitor<G> for RecordPath<'a, G> {
    fn discover_tree_edge(&mut self, g: &G, e: Edge<G>) -> Control {
        self.path.push(e);
        break_if(g.target(e) == self.target)
    }

    fn finish_tree_edge(&mut self, _g: &G, e: Edge<G>) -> Control {
        let r = self.path.pop();
        debug_assert_eq!(Some(e), r);
        Control::Continue
    }
}

#[cfg(test)]
mod tests {
    use super::Paths;
    use fera_fun::vec;
    use prelude::*;

    #[test]
    fn find_path() {
        let g: StaticGraph = graph!(6, (0, 1), (0, 2), (1, 4), (2, 3), (2, 4));
        let e = vec(g.edges());

        assert_eq!(None, g.find_path(0, 0));

        assert_eq!(None, g.find_path(0, 5));

        assert_eq!(vec![e[0]], g.find_path(0, 1).unwrap());

        assert_eq!(vec![e[0], e[1], e[4]], g.find_path(1, 4).unwrap());
    }

    #[test]
    fn is_path_graph() {
        let g: StaticDigraph = graph!(4, (0, 1), (1, 2), (2, 3));
        assert!(g.is_path_graph());
        let g: StaticDigraph = graph!(4, (0, 1), (1, 2), (1, 3));
        assert!(!g.is_path_graph());
    }
}
