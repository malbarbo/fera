use prelude::*;
use props::Color;
use traverse::*;

pub trait Cycles: Incidence {
    fn is_acyclic(&self) -> bool
        where Self: VertexList + WithVertexProp<Color>
    {
        let mut acyclic = true;
        self.dfs(IsAcyclic(&mut acyclic)).run();
        acyclic
    }

    fn is_dag(&self) -> bool
        where Self: VertexList + WithVertexProp<Color>
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
        *self.0 &= g.is_directed_edge(e);
        continue_if(*self.0)
    }

    fn discover_back_edge(&mut self, _g: &G, _e: Edge<G>) -> Control {
        *self.0 = false;
        Control::Break
    }
}
