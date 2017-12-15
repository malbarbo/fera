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
