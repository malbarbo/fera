use graph::*;

pub struct DisjointSet<'a, G>
    where G: 'a + Graph,
          &'a G: Types<G>,
{
    parent: PropVertex<'a, G, Vertex<G>>,
    rank: PropVertex<'a, G, usize>,
}

impl<'a, G> DisjointSet<'a, G>
    where G: 'a + Graph,
          &'a G: Types<G>,
{
    pub fn new(g: &'a G) -> DisjointSet<G> {
        let mut ds = DisjointSet::<G> {
            parent: g.vertex_prop(g.vertices().next().unwrap()),
            rank: g.vertex_prop(0usize),
        };

        for v in g.vertices() {
            ds.parent[v] = v;
        }

        ds
    }

    pub fn union(&mut self, x: Vertex<G>, y: Vertex<G>) {
        let a = self.find_set(x);
        let b = self.find_set(y);
        assert!( a != b );
        self.link(a, b);
    }

    pub fn in_same_set(&mut self, x: Vertex<G>, y: Vertex<G>) -> bool {
        self.find_set(x) == self.find_set(y)
    }

    fn link(&mut self, x: Vertex<G>, y: Vertex<G>) {
        if self.rank[x] > self.rank[y] {
            self.parent[y] = x;
        } else {
            self.parent[x] = y;
            if self.rank[x] == self.rank[y] {
                self.rank[y] += 1
            }
        }
    }

    fn find_set(&mut self, x: Vertex<G>) -> Vertex<G> {
        // TODO: write a iterative version
        if self.parent[x] != x {
            let p = self.parent[x];
            self.parent[x] = self.find_set(p);
        }
        self.parent[x]
    }
}

#[cfg(test)]
mod tests {
    use static_::*;
    use unionfind::*;

    fn check_groups(ds: &mut DisjointSet<StaticGraph>, groups: &[&[usize]]) {
        for group in groups.iter() {
            for &a in group.iter() {
                assert!(ds.in_same_set(group[0], a));
            }
        }
    }

    #[test]
    fn unionfind() {
        let g = StaticGraph::new_with_edges(5, &[]);
        let mut ds = DisjointSet::new(&g);
        ds.union(0, 2);
        check_groups(&mut ds, &[&[0, 2]]);
        ds.union(1, 3);
        check_groups(&mut ds, &[&[0, 2], &[1, 3]]);
        ds.union(2, 4);
        check_groups(&mut ds, &[&[0, 2, 4], &[1, 3]]);
        ds.union(3, 4);
        check_groups(&mut ds, &[&[0, 2, 4, 1, 3]]);
    }
}
