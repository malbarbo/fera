use graph::*;
use traverse::*;

pub trait Components: Incidence {
    fn is_connected(&self) -> bool
        where Self: DfsWithDefaultParams
    {
        let mut con = true;
        self.dfs_with_params(DfsParams::new(), IsConnected(&mut con));
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
