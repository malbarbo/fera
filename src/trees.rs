use prelude::*;
use traverse::*;

pub trait Trees: Incidence {
    fn is_tree(&self) -> bool
        where Self: EdgeList + DfsDefault
    {
        let mut tree = true;
        self.dfs(IsTree(&mut tree));
        tree
    }
}

impl<G: Incidence> Trees for G {}


pub struct IsTree<'a> {
    tree: &'a mut bool,
    saw_root: bool,
}

#[allow(non_snake_case)]
pub fn IsTree<'a>(tree: &'a mut bool) -> IsTree<'a> {
    IsTree {
        tree: tree,
        saw_root: false,
    }
}

// FIXME: should not require VertexList and EdgeList, it is just an optimization
impl<'a, G: VertexList + EdgeList> Visitor<G> for IsTree<'a> {
    fn start(&mut self, g: &G) -> Control {
        self.saw_root = false;
        *self.tree = g.num_vertices() == 0 || g.num_edges() == g.num_vertices() - 1;
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
