use prelude::*;
use props::Color;
use traverse::*;

pub trait Paths: Incidence {
    fn find_path(&self, u: Vertex<Self>, v: Vertex<Self>) -> Option<VecEdge<Self>>
        where Self: WithVertexProp<Color>
    {
        if u == v {
            return None;
        }
        let mut path = vec![];
        self.dfs(RecordPath(&mut path, v))
            .root(u)
            .run();
        if path.is_empty() { None } else { Some(path) }
    }
}

impl<G> Paths for G where G: Incidence {}


pub struct RecordPath<'a, G: WithEdge> {
    path: &'a mut Vec<Edge<G>>,
    target: Vertex<G>,
}

#[allow(non_snake_case)]
pub fn RecordPath<G>(path: &mut Vec<Edge<G>>, target: Vertex<G>) -> RecordPath<G>
    where G: WithEdge
{
    RecordPath {
        path: path,
        target: target,
    }
}

impl<'a, G: WithEdge> Visitor<G> for RecordPath<'a, G> {
    fn discover_tree_edge(&mut self, g: &G, e: Edge<G>) -> Control {
        self.path.push(e);
        break_if(g.target(e) == self.target)
    }

    fn finish_tree_edge(&mut self, _g: &G, e: Edge<G>) -> Control {
        let r = self.path.pop();
        debug_assert_eq!(Some(e), r);
        Control::Continue
    }
}


#[cfg(test)]
mod tests {
    use super::Paths;
    use static_::StaticGraph;
    use graph::*;
    use fera_fun::vec;

    #[test]
    fn find_path() {
        let g: StaticGraph = graph!(6, (0, 1), (0, 2), (1, 4), (2, 3), (2, 4));
        let e = vec(g.edges());

        assert_eq!(None, g.find_path(0, 0));

        assert_eq!(None, g.find_path(0, 5));

        assert_eq!(vec![e[0]], g.find_path(0, 1).unwrap());

        assert_eq!(vec![e[0], e[1], e[4]], g.find_path(1, 4).unwrap());
    }
}
