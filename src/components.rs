use prelude::*;
use traverse::*;

pub trait Components: Incidence {
    fn num_components(&self) -> u64
        where Self: DfsDefault
    {
        let mut num = 0;
        self.dfs(NumComponents(&mut num));
        num
    }

    fn is_connected(&self) -> bool
        where Self: DfsDefault
    {
        let mut con = true;
        self.dfs(IsConnected(&mut con));
        con
    }
}

impl<G: Incidence> Components for G {}


pub struct IsConnected<'a> {
    connected: &'a mut bool,
    saw_root: bool,
}

#[allow(non_snake_case)]
pub fn IsConnected<'a>(con: &'a mut bool) -> IsConnected<'a> {
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
pub fn NumComponents<'a>(num: &'a mut u64) -> OnDiscoverRootVertex<Count<'a>> {
    OnDiscoverRootVertex(Count(num))
}
