use DynamicTree;

pub struct ParentPointerTree {
    parent: Vec<usize>,
}

impl ParentPointerTree {
    pub fn new(n: usize) -> Self {
        Self { parent: (0..n).collect() }
    }

    pub fn has_edge(&self, x: usize, y: usize) -> bool {
        x != y && (self.parent[x] == y || self.parent[y] == x)
    }

    fn find_root(&self, mut x: usize) -> usize {
        while x != self.parent[x] {
            x = self.parent[x];
        }
        x
    }

    fn make_root(&mut self, x: usize) {
        let mut prev = x;
        let mut cur = x;
        let mut p = self.parent[cur];
        while p != cur {
            self.parent[cur] = prev;
            prev = cur;
            cur = p;
            p = self.parent[cur];
        }
        self.parent[cur] = prev;
        debug_assert_eq!(x, self.parent[x]);
    }
}

impl DynamicTree for ParentPointerTree {
    type Edge = (usize, usize);

    fn is_connected(&self, x: usize, y: usize) -> bool {
        self.find_root(x) == self.find_root(y)
    }

    fn link(&mut self, x: usize, y: usize) -> Option<Self::Edge> {
        if self.is_connected(x, y) {
            None
        } else {
            self.make_root(y);
            self.parent[y] = x;
            Some((x, y))
        }
    }

    fn cut(&mut self, (x, y): Self::Edge) {
        if self.parent[x] == y {
            self.parent[x] = x;
        } else if self.parent[y] == x {
            self.parent[y] = y;
        } else {
            panic!("The edge ({}, {}) does not exist", x, y);
        }
    }
}
