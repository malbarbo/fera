use graph::*;
use traverse::*;

pub trait Props: Graph {
    fn is_acyclic<'a>(&'a self) -> bool
        where &'a Self: Types<Self>
    {
        let mut acyclic = true;
        Dfs::run(self,
                 &mut BackEdgeVisitor(|_| {
                     acyclic = false;
                     false
                 }));
        acyclic
    }

    fn is_connected<'a>(&'a self) -> bool
        where &'a Self: Types<Self>
    {
        self.num_vertices() == 0 ||
        {
            let mut count = 0;
            Dfs::run(self,
                     &mut StartVertexVisitor(|_| {
                         count += 1;
                         count == 1
                     }));
            count == 1
        }
    }

    fn is_tree<'a>(&'a self) -> bool
        where &'a Self: Types<Self>
    {
        self.num_vertices() == 0 ||
        {
            self.num_edges() == self.num_vertices() - 1 && self.is_acyclic()
        }
    }
}

impl<G> Props for G
    where G: Graph { }

#[cfg(test)]
mod tests {
    use static_::*;
    use props::*;

    struct Case {
        g: StaticGraph,
        is_connected: bool,
        is_acyclic: bool,
        is_tree: bool,
    }

    fn cases() -> Vec<Case> {
        let graph = StaticGraph::new_with_edges;
        vec![
            Case { // 0
                g: StaticGraph::new_empty(),
                is_connected: true,
                is_acyclic: true,
                is_tree: true,
            },
            Case { // 1
                g: graph(1, &[]),
                is_connected: true,
                is_acyclic: true,
                is_tree: true,
            },
            Case { // 2
                g: graph(2, &[]),
                is_connected: false,
                is_acyclic: true,
                is_tree: false,
            },
            Case { // 3
                g: graph(2, &[(0, 1)]),
                is_connected: true,
                is_acyclic: true,
                is_tree: true,
            },
            Case { // 4
                g: graph(3, &[(2, 1)]),
                is_connected: false,
                is_acyclic: true,
                is_tree: false,
            },
            Case { // 5
                g: graph(3, &[(2, 1)]),
                is_connected: false,
                is_acyclic: true,
                is_tree: false,
            },
            Case { // 6
                g: graph(3, &[(0, 1), (1, 2)]),
                is_connected: true,
                is_acyclic: true,
                is_tree: true,
            },
            Case { // 7
                g: graph(3, &[(0, 1), (0, 2), (1, 2)]),
                is_connected: true,
                is_acyclic: false,
                is_tree: false,
            },
            Case { // 8
                g: graph(4, &[(0, 1), (0, 2)]),
                is_connected: false,
                is_acyclic: true,
                is_tree: false,
            },
            Case { // 9
                g: graph(4, &[(1, 2), (2, 3), (3, 1)]),
                is_connected: false,
                is_acyclic: false,
                is_tree: false,
            },
        ]
    }

    #[test]
    fn is_connected() {
        for (i, case) in cases().iter().enumerate() {
            assert!(case.is_connected == case.g.is_connected(),
                    format!("Case {}", i));
        }
    }

    #[test]
    fn is_acyclic() {
        for (i, case) in cases().iter().enumerate() {
            assert!(case.is_acyclic == case.g.is_acyclic(),
                    format!("Case {}", i));
        }
    }

    #[test]
    fn is_tree() {
        for (i, case) in cases().iter().enumerate() {
            assert!(case.is_tree == case.g.is_tree(), format!("Case {}", i));
        }
    }
}
