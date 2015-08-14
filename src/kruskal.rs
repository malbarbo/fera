use super::*;
use super::unionfind::DisjointSet;

pub trait Visitor<G: Basic> {
    fn visit(&mut self, e: G::Edge, in_same_set: bool) -> bool;
}

impl<F, G> Visitor< G> for F
    where G: Basic,
          F: FnMut(G::Edge, bool) -> bool {
    fn visit(&mut self, e: G::Edge, in_same_set: bool) -> bool {
        self(e, in_same_set)
    }
}

pub trait Kruskal: Basic + WithEdgeProp + WithVertexProp + Sized {
    fn kruskal_edges<I, V>(&self, edges: I, mut visitor: V)
        where I: Iterator<Item=Self::Edge>,
              V: Visitor<Self>,
              Self::Vertex: PartialEq {
        let mut ds = DisjointSet::new(self);
        for e in edges {
            let (u, v) = self.endvertices(e);
            let in_same_set = ds.in_same_set(u, v);
            if !visitor.visit(e, in_same_set) {
                return;
            }
            if !in_same_set {
                ds.union(u, v);
            }
        }
    }

    fn kruskal<T, V>(&self, weight: &EdgeProp<Self, T>, visitor: V)
        where T: Ord,
              V: Visitor<Self>,
              Self: for<'a> EdgePropType<'a, T>,
              Self::Vertex: PartialEq {
        let mut edges = self.edges().collect::<Vec<_>>();
        edges.sort_by(|a, b| weight[*a].cmp(&weight[*b]));
        self.kruskal_edges(edges.iter().cloned(), visitor);
    }

    fn kruskal_mst<T>(&self, weight: &EdgeProp<Self, T>) -> Vec<Self::Edge>
        where T: Ord,
              Self: for<'a> EdgePropType<'a, T>,
              Self::Vertex: PartialEq {
        let mut tree = vec![];
        self.kruskal::<T, _>(weight, |e: Self::Edge, in_same_set: bool| {
            if !in_same_set {
                tree.push(e);
            }
            tree.len() != self.num_vertices() - 1
        });
        tree
    }
}

impl<G> Kruskal for G where G: Basic + WithVertexProp + WithEdgeProp { }


#[cfg(test)]
mod tests {
    use super::*;
    use super::super::*;
    use super::super::iter::*;

    #[test]
    fn kruskal_mst() {
        let g = StaticGraph::new_with_edges(5, &[(0, 4), (2, 3), (0, 1), (1, 4), (1, 2), (2, 4), (3, 4)]);
        // expected tree                           0       1       2               3
        let mut weight = g.edge_prop(0usize);
        for (e, w) in g.edges().zip(&[1, 2, 3, 4, 5, 6, 7]) {
            weight[e] = *w;
        }
        let e = g.edges().as_vec();
        assert_eq!(vec![e[0], e[1], e[2], e[4]], g.kruskal_mst(&weight));
    }
}
