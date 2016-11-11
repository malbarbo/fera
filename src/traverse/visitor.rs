use graph::*;
use super::control::*;

// TODO: check if event names make sense for both dfs and bfs
pub trait Visitor<G>
    where G: WithEdge
{
    fn discover_root_vertex(&mut self, _g: &G, _v: Vertex<G>) -> Control {
        Control::Continue
    }

    fn finish_root_vertex(&mut self, _g: &G, _v: Vertex<G>) -> Control {
        Control::Continue
    }

    fn discover_vertex(&mut self, _g: &G, _v: Vertex<G>) -> Control {
        Control::Continue
    }

    fn finish_vertex(&mut self, _g: &G, _v: Vertex<G>) -> Control {
        Control::Continue
    }

    fn discover_edge(&mut self, _g: &G, _e: Edge<G>) -> Control {
        Control::Continue
    }

    fn finish_edge(&mut self, _g: &G, _e: Edge<G>) -> Control {
        Control::Continue
    }

    fn discover_tree_edge(&mut self, _g: &G, _e: Edge<G>) -> Control {
        Control::Continue
    }

    fn finish_tree_edge(&mut self, _g: &G, _e: Edge<G>) -> Control {
        Control::Continue
    }

    fn discover_back_edge(&mut self, _g: &G, _e: Edge<G>) -> Control {
        Control::Continue
    }

    fn discover_cross_or_forward_edge(&mut self, _g: &G, _e: Edge<G>) -> Control {
        Control::Continue
    }
}

impl<'a, G, V> Visitor<G> for &'a mut V
    where G: WithEdge,
          V: Visitor<G>
{
    fn discover_root_vertex(&mut self, g: &G, v: Vertex<G>) -> Control {
        V::discover_root_vertex(self, g, v)
    }

    fn finish_root_vertex(&mut self, g: &G, v: Vertex<G>) -> Control {
        V::finish_root_vertex(self, g, v)
    }

    fn discover_vertex(&mut self, g: &G, v: Vertex<G>) -> Control {
        V::discover_vertex(self, g, v)
    }

    fn finish_vertex(&mut self, g: &G, v: Vertex<G>) -> Control {
        V::finish_vertex(self, g, v)
    }

    fn discover_edge(&mut self, g: &G, e: Edge<G>) -> Control {
        V::discover_edge(self, g, e)
    }

    fn finish_edge(&mut self, g: &G, e: Edge<G>) -> Control {
        V::finish_edge(self, g, e)
    }

    fn discover_tree_edge(&mut self, g: &G, e: Edge<G>) -> Control {
        V::discover_tree_edge(self, g, e)
    }

    fn finish_tree_edge(&mut self, g: &G, e: Edge<G>) -> Control {
        V::finish_tree_edge(self, g, e)
    }

    fn discover_back_edge(&mut self, g: &G, e: Edge<G>) -> Control {
        V::discover_back_edge(self, g, e)
    }

    fn discover_cross_or_forward_edge(&mut self, g: &G, e: Edge<G>) -> Control {
        V::discover_cross_or_forward_edge(self, g, e)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TraverseEvent<G: WithVertex + WithEdge> {
    DiscoverRootVertex(Vertex<G>),
    FinishRootVertex(Vertex<G>),
    DiscoverVertex(Vertex<G>),
    FinishVertex(Vertex<G>),
    DiscoverEdge(Edge<G>),
    FinishEdge(Edge<G>),
    DiscoverTreeEdge(Edge<G>),
    FinishTreeEdge(Edge<G>),
    DiscoverBackEdge(Edge<G>),
    DiscoverCrossOrForwardEdge(Edge<G>),
}

pub struct FnTraverseEvent<F>(pub F);

impl<G, F, R> Visitor<G> for FnTraverseEvent<F>
    where G: WithVertex + WithEdge,
          F: FnMut(TraverseEvent<G>) -> R,
          R: Into<Control>
{
    fn discover_root_vertex(&mut self, _g: &G, v: Vertex<G>) -> Control {
        (self.0)(TraverseEvent::DiscoverRootVertex(v)).into()
    }

    fn finish_root_vertex(&mut self, _g: &G, v: Vertex<G>) -> Control {
        (self.0)(TraverseEvent::FinishRootVertex(v)).into()
    }

    fn discover_vertex(&mut self, _g: &G, v: Vertex<G>) -> Control {
        (self.0)(TraverseEvent::DiscoverVertex(v)).into()
    }

    fn finish_vertex(&mut self, _g: &G, v: Vertex<G>) -> Control {
        (self.0)(TraverseEvent::FinishVertex(v)).into()
    }

    fn discover_edge(&mut self, _g: &G, e: Edge<G>) -> Control {
        (self.0)(TraverseEvent::DiscoverEdge(e)).into()
    }

    fn finish_edge(&mut self, _g: &G, e: Edge<G>) -> Control {
        (self.0)(TraverseEvent::FinishEdge(e)).into()
    }

    fn discover_tree_edge(&mut self, _g: &G, e: Edge<G>) -> Control {
        (self.0)(TraverseEvent::DiscoverTreeEdge(e)).into()
    }

    fn finish_tree_edge(&mut self, _g: &G, e: Edge<G>) -> Control {
        (self.0)(TraverseEvent::FinishTreeEdge(e)).into()
    }

    fn discover_back_edge(&mut self, _g: &G, e: Edge<G>) -> Control {
        (self.0)(TraverseEvent::DiscoverBackEdge(e)).into()
    }

    fn discover_cross_or_forward_edge(&mut self, _g: &G, e: Edge<G>) -> Control {
        (self.0)(TraverseEvent::DiscoverCrossOrForwardEdge(e)).into()
    }
}

// TODO: use a macro to generenate single events structs

pub struct DiscoverRootVertex<F>(pub F);

impl<G, F, R> Visitor<G> for DiscoverRootVertex<F>
    where G: WithEdge,
          F: FnMut(Vertex<G>) -> R,
          R: Into<Control>
{
    fn discover_root_vertex(&mut self, _g: &G, v: Vertex<G>) -> Control {
        (self.0)(v).into()
    }
}


pub struct DiscoverTreeEdge<F>(pub F);

impl<G, F, R> Visitor<G> for DiscoverTreeEdge<F>
    where G: WithEdge,
          F: FnMut(Edge<G>) -> R,
          R: Into<Control>
{
    fn discover_tree_edge(&mut self, _g: &G, e: Edge<G>) -> Control {
        (self.0)(e).into()
    }
}


pub struct DiscoverBackEdge<F>(pub F);

impl<G, F, R> Visitor<G> for DiscoverBackEdge<F>
    where G: WithEdge,
          F: FnMut(Edge<G>) -> R,
          R: Into<Control>
{
    fn discover_back_edge(&mut self, _g: &G, e: Edge<G>) -> Control {
        (self.0)(e).into()
    }
}


pub struct DiscoverFinishTime<G: Graph> {
    time: u64,
    pub discover: DefaultVertexPropMut<G, u64>,
    pub finish: DefaultVertexPropMut<G, u64>,
}

impl<G: Graph> DiscoverFinishTime<G> {
    pub fn new(g: &G) -> Self {
        DiscoverFinishTime {
            time: 0,
            discover: g.vertex_prop(0),
            finish: g.vertex_prop(0),
        }
    }

    pub fn is_ancestor_of(&self, ancestor: Vertex<G>, u: Vertex<G>) -> bool {
        self.discover[ancestor] <= self.discover[u] && self.finish[u] <= self.finish[ancestor]
    }
}

impl<G: Graph> Visitor<G> for DiscoverFinishTime<G> {
    fn discover_vertex(&mut self, _g: &G, v: Vertex<G>) -> Control {
        self.discover[v] = self.time;
        self.time += 1;
        Control::Continue
    }

    fn finish_vertex(&mut self, _g: &G, v: Vertex<G>) -> Control {
        self.finish[v] = self.time;
        self.time += 1;
        Control::Continue
    }
}
