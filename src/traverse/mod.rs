use graph::*;

#[macro_use]
mod control;
mod visitor;
mod bfs;
mod dfs;
mod recursive_dfs;

pub use self::control::*;
pub use self::visitor::*;
pub use self::bfs::*;
pub use self::dfs::*;
pub use self::recursive_dfs::*;

pub trait Traverser<'a, G>
    where G: 'a + Incidence
{
    fn graph(&self) -> &G;

    fn is_discovered(&self, v: Vertex<G>) -> bool;

    fn traverse<V: Visitor<G>>(&mut self, v: Vertex<G>, vis: V) -> bool;

    fn traverse_all<V: Visitor<G>>(&mut self, vis: V) where G: VertexList;

    fn traverse_vertices<I, V>(&mut self, vertices: I, mut vis: V)
        where I: IntoIterator<Item = Vertex<G>>,
              V: Visitor<G>
    {
        for v in vertices {
            if !self.is_discovered(v) {
                break_unless!(vis.discover_root_vertex(self.graph(), v));
                break_unless!(continue_if(self.traverse(v, &mut vis)));
                break_unless!(vis.finish_root_vertex(self.graph(), v));
            }
        }
    }
}
