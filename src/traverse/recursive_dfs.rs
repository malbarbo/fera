use graph::*;
use super::visitor::*;

pub fn recursive_dfs<G, V>(g: &G, mut vis: V)
    where G: VertexList + Incidence + WithVertexProp<Color>,
          V: Visitor<G>
{
    let mut color = g.default_vertex_prop(Color::White);
    for v in g.vertices() {
        if color[v] == Color::White {
            vis.discover_root_vertex(g, v);
            recursive_dfs_visit(g, G::edge_none(), v, &mut color, &mut vis);
            vis.finish_root_vertex(g, v);
        }
    }
}

pub fn recursive_dfs_visit<G, C, V>(g: &G,
                                    from: OptionEdge<G>,
                                    u: Vertex<G>,
                                    color: &mut C,
                                    vis: &mut V)
    where G: Incidence,
          C: VertexPropMut<G, Color>,
          V: Visitor<G>
{
    color[u] = Color::Gray;
    vis.discover_vertex(g, u);
    for e in g.out_edges(u) {
        let v = g.target(e);
        if g.is_undirected_edge(e) && color[v] == Color::Black || G::edge_some(e) == from {
            continue;
        }
        vis.discover_edge(g, e);
        match color[v] {
            Color::White => {
                vis.discover_tree_edge(g, e);
                recursive_dfs_visit(g, e.into(), v, color, vis);
                vis.finish_tree_edge(g, e);
            }
            Color::Gray => {
                vis.discover_back_edge(g, e);
            }
            Color::Black => {
                vis.discover_cross_or_forward_edge(g, e);
            }
        }
        vis.finish_edge(g, e);
    }
    color[u] = Color::Black;
    vis.finish_vertex(g, u);
}
