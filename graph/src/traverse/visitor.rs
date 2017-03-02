use graph::*;
use props::*;
use super::control::*;

// TODO: check if event names make sense for both dfs and bfs
pub trait Visitor<G: WithEdge> {
    fn start(&mut self, _g: &G) -> Control {
        Control::Continue
    }

    fn finish(&mut self, _g: &G) -> Control {
        Control::Continue
    }

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
    fn start(&mut self, g: &G) -> Control {
        V::start(self, g)
    }

    fn finish(&mut self, g: &G) -> Control {
        V::start(self, g)
    }

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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EmptyVisitor;


impl<G: WithEdge> Visitor<G> for EmptyVisitor {}

macro_rules! def_visitor_tuple_m {
    ($t:ident, $m:ident, $($name:ident),*) => (
        #[allow(non_snake_case)]
        #[cfg_attr(feature = "cargo-clippy", allow(let_and_return))]
        fn $m(&mut self, g: &G, item: $t<G>) -> Control {
            let ($(ref mut $name),*) = *self;
            let r = Control::Continue;
            $(
                let r = if r == Control::Continue {
                    $name.$m(g, item)
                } else {
                    return Control::Break;
                };
            )*
            r
        }
    )
}

macro_rules! def_visitor_tuple {
    ($($name:ident),*) => (
        impl<G, $($name),*> Visitor<G> for ($($name),*)
            where G: WithEdge,
                  $($name: Visitor<G>),*
        {
            def_visitor_tuple_m!{Vertex, discover_root_vertex, $($name),*}
            def_visitor_tuple_m!{Vertex, finish_root_vertex, $($name),*}

            def_visitor_tuple_m!{Vertex, discover_vertex, $($name),*}
            def_visitor_tuple_m!{Vertex, finish_vertex, $($name),*}

            def_visitor_tuple_m!{Edge, discover_edge, $($name),*}
            def_visitor_tuple_m!{Edge, finish_edge, $($name),*}

            def_visitor_tuple_m!{Edge, discover_tree_edge, $($name),*}
            def_visitor_tuple_m!{Edge, finish_tree_edge, $($name),*}

            def_visitor_tuple_m!{Edge, discover_back_edge, $($name),*}
            def_visitor_tuple_m!{Edge, discover_cross_or_forward_edge, $($name),*}
        }
    )
}

def_visitor_tuple!(A, B);
def_visitor_tuple!(A, B, C);
def_visitor_tuple!(A, B, C, D);


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TraverseEvent<G: WithEdge> {
    Start,
    Finish,
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

pub struct OnTraverseEvent<F>(pub F);

impl<G, F, R> Visitor<G> for OnTraverseEvent<F>
    where G: WithEdge,
          F: FnMut(TraverseEvent<G>) -> R,
          R: Into<Control>
{
    fn start(&mut self, _g: &G) -> Control {
        (self.0)(TraverseEvent::Start).into()
    }

    fn finish(&mut self, _g: &G) -> Control {
        (self.0)(TraverseEvent::Finish).into()
    }

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


// Vertex

pub trait VisitVertex<G: WithEdge> {
    fn visit_vertex(&mut self, g: &G, v: Vertex<G>) -> Control;
}

impl<G, F, R> VisitVertex<G> for F
    where G: WithEdge,
          F: FnMut(Vertex<G>) -> R,
          R: Into<Control>
{
    fn visit_vertex(&mut self, _g: &G, v: Vertex<G>) -> Control {
        self(v).into()
    }
}

macro_rules! def_on_vertex_visitor {
    ($name:ident, $event:ident) => (
        pub struct $name<V>(pub V);

        impl<G, V> Visitor<G> for $name<V>
            where G: WithEdge,
                  V: VisitVertex<G>
        {
            fn $event(&mut self, g: &G, v: Vertex<G>) -> Control {
                self.0.visit_vertex(g, v)
            }
        }
    )
}

def_on_vertex_visitor!(OnDiscoverRootVertex, discover_root_vertex);
def_on_vertex_visitor!(OnFinishRootVertex, finish_root_vertex);

def_on_vertex_visitor!(OnDiscoverVertex, discover_vertex);
def_on_vertex_visitor!(OnFinishVertex, finish_vertex);


// Edge

pub trait VisitEdge<G: WithEdge> {
    fn visit_edge(&mut self, g: &G, e: Edge<G>) -> Control;
}

impl<G, F, R> VisitEdge<G> for F
    where G: WithEdge,
          F: FnMut(Edge<G>) -> R,
          R: Into<Control>
{
    fn visit_edge(&mut self, _g: &G, e: Edge<G>) -> Control {
        self(e).into()
    }
}

macro_rules! def_on_edge_visitor {
    ($name:ident, $event:ident) => (
        pub struct $name<V>(pub V);

        impl<G, V> Visitor<G> for $name<V>
            where G: WithEdge,
                  V: VisitEdge<G>
        {
            fn $event(&mut self, g: &G, e: Edge<G>) -> Control {
                self.0.visit_edge(g, e)
            }
        }
    )
}

def_on_edge_visitor!(OnDiscoverEdge, discover_edge);
def_on_edge_visitor!(OnFinishEdge, finish_edge);

def_on_edge_visitor!(OnDiscoverTreeEdge, discover_tree_edge);
def_on_edge_visitor!(OnFinishTreeEdge, finish_tree_edge);

def_on_edge_visitor!(OnDiscoverBackEdge, discover_back_edge);
def_on_edge_visitor!(OnDiscoverCrossOrBackEdge, discover_cross_or_forward_edge);


// Some visitors

use std::cell::Cell;

pub struct Count<'a> {
    count: &'a mut u64,
}

#[allow(non_snake_case)]
pub fn Count(mut count: &mut u64) -> Count {
    *count = 0;
    Count { count: count }
}

impl<'a, G> VisitVertex<G> for Count<'a>
    where G: WithEdge
{
    fn visit_vertex(&mut self, _g: &G, _v: Vertex<G>) -> Control {
        *self.count += 1;
        Control::Continue
    }
}

impl<'a, G> VisitEdge<G> for Count<'a>
    where G: WithEdge
{
    fn visit_edge(&mut self, _g: &G, _e: Edge<G>) -> Control {
        *self.count += 1;
        Control::Continue
    }
}


#[derive(Default)]
pub struct Time {
    cur: Cell<u64>,
}

impl Time {
    fn get_and_inc(&self) -> u64 {
        let t = self.cur.get();
        self.cur.set(t + 1);
        t
    }
}

pub struct StampTime<'a, P: 'a>(pub &'a Time, pub &'a mut P);

impl<'a, G, P> VisitVertex<G> for StampTime<'a, P>
    where G: WithEdge,
          P: VertexPropMut<G, u64>
{
    fn visit_vertex(&mut self, _g: &G, v: Vertex<G>) -> Control {
        self.1[v] = self.0.get_and_inc();
        Control::Continue
    }
}

pub struct RecordDistance<'a, P: 'a>(pub &'a mut P);

impl<'a, G, P> VisitEdge<G> for RecordDistance<'a, P>
    where G: WithEdge,
          P: VertexPropMut<G, u64>
{
    fn visit_edge(&mut self, g: &G, e: Edge<G>) -> Control {
        let (u, v) = g.ends(e);
        self.0[v] = self.0[u] + 1;
        Control::Continue
    }
}

// TODO: write tests
