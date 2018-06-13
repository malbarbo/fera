// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Trees related algortihms, including testing if a graph is a tree.

use prelude::*;
use props::Color;
use traverse::*;

pub trait Trees: Incidence {
    fn is_tree(&self) -> bool
    where
        Self: VertexList + EdgeList + WithVertexProp<Color>,
    {
        let mut tree = true;
        self.dfs(IsTree(&mut tree)).run();
        tree
    }

    fn tree_diameter(&self) -> Result<usize, ()>
    where
        Self: VertexList + EdgeList + WithVertexProp<Color>,
    {
        let mut tree = false;
        let mut dist = 0;
        let mut v = Self::vertex_none();
        self.dfs((IsTree(&mut tree), FarthestVertex(&mut v, &mut dist)))
            .run();
        if tree {
            Ok(v.into_option()
                .map(|r| {
                    self.dfs(FarthestVertex(&mut Self::vertex_none(), &mut dist))
                        .root(r)
                        .run();
                    dist
                })
                .unwrap_or(0))
        } else {
            Err(())
        }
    }
}

impl<G: Incidence> Trees for G {}

pub struct IsTree<'a> {
    tree: &'a mut bool,
    saw_root: bool,
}

#[allow(non_snake_case)]
pub fn IsTree(tree: &mut bool) -> IsTree {
    IsTree {
        tree: tree,
        saw_root: false,
    }
}

// FIXME: should not require VertexList and EdgeList, it is just an optimization
impl<'a, G: WithEdge> Visitor<G> for IsTree<'a> {
    fn start(&mut self, g: &G) -> Control {
        self.saw_root = false;
        if let Some(g) = specialize!(g, VertexList) {
            *self.tree = g.num_vertices() == 0
                || if let Some(g) = specialize!(g, VertexList, EdgeList) {
                    g.num_edges() == g.num_vertices() - 1
                } else {
                    false
                }
        }
        // *self.tree = g.num_vertices() == 0 || g.num_edges() == g.num_vertices() - 1;
        continue_if(*self.tree)
    }

    fn discover_back_edge(&mut self, _g: &G, _e: Edge<G>) -> Control {
        *self.tree = false;
        Control::Break
    }

    fn discover_root_vertex(&mut self, _g: &G, _v: Vertex<G>) -> Control {
        if self.saw_root {
            *self.tree = false;
            Control::Break
        } else {
            self.saw_root = true;
            Control::Continue
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use algs::Distances;
    use rand;

    #[test]
    fn tree_diameter() {
        let mut rng = rand::weak_rng();
        for n in 1..10 {
            for _ in 0..20 {
                let g = StaticGraph::new_random_tree(n, &mut rng);
                assert_eq!(g.tree_diameter(), Ok(g.diameter()));
            }
        }

        for n in 3..30 {
            for d in 2..n - 1 {
                let g = StaticGraph::new_random_tree_with_diameter(n, d, &mut rng).unwrap();
                assert_eq!(Ok(d as usize), g.tree_diameter());
            }
        }
    }
}
