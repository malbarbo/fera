// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Cycles related algorithms, including testing if a graph is acyclic.

use prelude::*;
use props::Color;
use traverse::*;

pub trait Cycles: Incidence {
    fn is_acyclic(&self) -> bool
    where
        Self: VertexList + WithVertexProp<Color>,
    {
        let mut acyclic = true;
        self.dfs(IsAcyclic(&mut acyclic)).run();
        acyclic
    }

    fn is_dag(&self) -> bool
    where
        Self: VertexList + WithVertexProp<Color>,
    {
        let mut dag = true;
        self.dfs(IsDag(&mut dag)).run();
        dag
    }

    fn is_cycle_graph(&self) -> bool
    where
        Self: VertexList + EdgeList + WithVertexProp<Color>,
    {
        self.num_edges() == self.num_vertices()
            && self
                .vertices()
                .next()
                .map(|first| {
                    let mut prev = first;
                    let mut count = 1;
                    let mut cur = self.out_neighbors(prev).next().unwrap();
                    while let Some(v) = self.out_neighbors(cur).filter(|&u| prev != u).next() {
                        count += 1;
                        prev = cur;
                        cur = v;
                        if count == self.num_edges() {
                            break;
                        }
                    }
                    count == self.num_edges() && cur == first
                })
                .unwrap_or(false)
    }
}

impl<G: Incidence> Cycles for G {}

pub struct IsAcyclic<'a>(pub &'a mut bool);

impl<'a, G: WithEdge> Visitor<G> for IsAcyclic<'a> {
    fn start(&mut self, _g: &G) -> Control {
        *self.0 = true;
        Control::Continue
    }

    fn discover_back_edge(&mut self, _g: &G, _e: Edge<G>) -> Control {
        *self.0 = false;
        Control::Break
    }
}

pub struct IsDag<'a>(pub &'a mut bool);

impl<'a, G: WithEdge> Visitor<G> for IsDag<'a> {
    fn start(&mut self, _g: &G) -> Control {
        *self.0 = true;
        Control::Continue
    }

    fn discover_edge(&mut self, g: &G, e: Edge<G>) -> Control {
        *self.0 &= g.orientation(e).is_directed();
        continue_if(*self.0)
    }

    fn discover_back_edge(&mut self, _g: &G, _e: Edge<G>) -> Control {
        *self.0 = false;
        Control::Break
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_cycle_graph() {
        let g: StaticDigraph = graph!(4, (0, 1), (1, 2), (2, 3), (3, 0));
        assert!(g.is_cycle_graph());
        let g: StaticDigraph = graph!(4, (0, 1), (1, 2), (1, 3), (0, 3));
        assert!(!g.is_cycle_graph());
        let g: StaticDigraph = graph!(4, (0, 1), (1, 2), (1, 3), (3, 1));
        assert!(!g.is_cycle_graph());
        let g: StaticDigraph = graph!(4, (0, 1), (1, 2), (1, 3));
        assert!(!g.is_cycle_graph());
    }
}
