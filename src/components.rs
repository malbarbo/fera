use prelude::*;
use traverse::*;

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
