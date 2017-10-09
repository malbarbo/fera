use std::mem;
use std::ops::Range;
use std::ptr;

use DynamicTree;

mod seq;
pub use self::seq::*;

// invariants:
// if x is a isolated vertex,
//     active[x] == None
// else
//     active[x].source == u
pub struct EulerTourTree<A: Sequence<&'static Edge>> {
    tours: Box<[A]>,
    ends: Box<[(usize, usize)]>,
    edges: Box<[Edge]>,
    active: Box<[Option<&'static Edge>]>,
    free_tours: Vec<usize>,
    free_edges: Vec<usize>,
}

impl<A: Sequence<&'static Edge>> DynamicTree for EulerTourTree<A> {
    // TODO: use an opaque type
    type Edge = usize;

    fn is_connected(&self, u: usize, v: usize) -> bool {
        self.find_root_node(u) == self.find_root_node(v)
    }

    fn link(&mut self, u: usize, v: usize) -> Option<Self::Edge> {
        if self.is_connected(u, v) {
            return None;
        }
        let (i, e, f) = self.new_edge(u, v);
        match (self.active[u], self.active[v]) {
            (Some(u_act), Some(v_act)) => {
                // TODO: avoid one call to make_root
                self.make_root(u);
                self.make_root(v);
                self.push(u_act.tour(), e);
                self.free_tours.push(v_act.tour());
                {
                    let (u_tour, v_tour) = self.get_tours(u_act.tour(), v_act.tour());
                    for i in 0..v_tour.len() {
                        v_tour[i].set_tour(u_act.tour());
                        v_tour[i].set_rank(u_tour.len() + i);
                    }
                    u_tour.append(v_tour);
                }
                self.push(u_act.tour(), f);
            }
            (Some(u_act), None) => {
                self.make_root(u);
                self.active[v] = Some(f);
                self.push(u_act.tour(), e);
                self.push(u_act.tour(), f);
            }
            (None, Some(v_act)) => {
                self.make_root(v);
                self.active[u] = Some(e);
                self.push(v_act.tour(), f);
                self.push(v_act.tour(), e);
            }
            (None, None) => {
                self.active[u] = Some(e);
                self.active[v] = Some(f);
                self.new_tour(e, f);
            }
        }
        debug_assert!(self.is_connected(u, v));
        debug_assert!(self.check());
        Some(i)
    }

    fn cut(&mut self, edge: Self::Edge) {
        // TODO: avoid make_root
        let (u, v) = self.ends(&self.edges[edge << 1]);
        self.make_root(u);
        let (i, range) = self.tour_range(edge);
        let j = self.free_tours.pop().unwrap();
        let (i_free, j_free) = {
            let (i_tour, j_tour) = self.get_tours(i, j);
            i_tour.extract(range, j_tour);
            for k in 0..j_tour.len() {
                j_tour[k].set_tour(j);
                j_tour[k].set_rank(k);
            }
            for k in 0..i_tour.len() {
                i_tour[k].set_tour(i);
                i_tour[k].set_rank(k);
            }
            (i_tour.len() == 0, j_tour.len() == 0)
        };
        let e = self.tours[i].last().cloned();
        self.set_active(u, e);
        let e = self.tours[j].first().cloned();
        self.set_active(v, e);
        if i_free {
            self.free_tours.push(i);
        }
        if j_free {
            self.free_tours.push(j);
        }
        self.free_edges.push(edge);
        debug_assert!(!self.is_connected(u, v));
        debug_assert!(self.check());
    }
}

impl<A: Sequence<&'static Edge>> EulerTourTree<A> {
    pub fn new(n: usize) -> Self {
        let max_edges = 2 * (n - 1);
        let max_tours = n / 2 + 1;

        Self {
            tours: (0..max_tours)
                .map(|_| A::with_capacity(max_edges))
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            ends: vec![(0, 0); n - 1].into_boxed_slice(),
            edges: (0..max_edges)
                .map(Edge::new)
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            active: vec![None; n].into_boxed_slice(),
            free_edges: (0..n - 1).rev().collect(),
            free_tours: (0..max_tours).rev().collect(),
        }
    }

    fn make_root(&mut self, x: usize) {
        if let Some(e) = self.active[x] {
            assert_eq!(x, self.source(e));
            if e.rank() == 0 {
                return;
            }

            let tour = &mut self.tours[e.tour()];
            tour.rotate(e.rank());
            for i in 0..tour.len() {
                tour[i].set_rank(i);
            }
        }
        debug_assert_eq!(x, self.find_root_node(x));
        debug_assert!(self.check());
    }

    fn find_root_node(&self, x: usize) -> usize {
        if let Some(e) = self.active[x] {
            self.source(self.tours[e.tour()].first().unwrap())
        } else {
            x
        }
    }

    fn source(&self, e: &Edge) -> usize {
        self.ends(e).0
    }

    fn ends(&self, e: &Edge) -> (usize, usize) {
        let (u, v) = self.ends[e.index()];
        if e.is_reversed() { (v, u) } else { (u, v) }
    }

    fn tour_range(&self, index: usize) -> (usize, Range<usize>) {
        let e = &self.edges[index << 1];
        let f = &self.edges[(index << 1) + 1];
        if e.rank() < f.rank() {
            (e.tour(), e.rank()..f.rank() + 1)
        } else {
            (e.tour(), f.rank()..e.rank() + 1)
        }
    }

    fn set_active(&mut self, v: usize, e: Option<&'static Edge>) {
        self.active[v] = e.map(|e| {
            let (s, t) = self.ends(e);
            if s == v {
                e
            } else {
                assert_eq!(v, t);
                self.pair(e)
            }
        });
    }

    fn pair(&self, e: &Edge) -> &'static Edge {
        unsafe { mem::transmute(&self.edges[e.id ^ 1]) }
    }

    fn get_tours(&mut self, i: usize, j: usize) -> (&mut A, &mut A) {
        assert_ne!(i, j);
        unsafe {
            let a: *mut A = &mut self.tours[i];
            let b: *mut A = &mut self.tours[j];
            (mem::transmute(a), mem::transmute(b))
        }
    }

    fn push(&mut self, tour: usize, e: &'static Edge) {
        e.set_tour(tour);
        e.set_rank(self.tours[tour].len());
        self.tours[tour].push(e);
    }

    fn new_edge(&mut self, u: usize, v: usize) -> (usize, &'static Edge, &'static Edge) {
        assert_ne!(u, v);
        let i = self.free_edges.pop().unwrap();
        self.ends[i] = (u, v);
        let e = &self.edges[i << 1];
        let f = &self.edges[(i << 1) + 1];
        unsafe { (i, mem::transmute(e), mem::transmute(f)) }
    }

    fn new_tour(&mut self, e: &'static Edge, f: &'static Edge) -> usize {
        let i = self.free_tours.pop().unwrap();
        e.set_tour(i);
        e.set_rank(0);
        f.set_tour(i);
        f.set_rank(1);
        self.tours[i].push(e);
        self.tours[i].push(f);
        i
    }

    fn check(&self) -> bool {
        for i in 0..self.active.len() {
            if let Some(e) = self.active[i] {
                assert!(ptr::eq(e, self.tours[e.tour()][e.rank()]));
            }
        }
        for (i, tour) in self.tours.iter().enumerate() {
            for j in 0..tour.len() {
                let (u, v) = self.ends(tour[j]);
                assert_eq!(
                    self.find_root_node(u),
                    self.find_root_node(v),
                    "\nactive {} = {:?}\nactive {} = {:?}",
                    u,
                    self.active[u],
                    v,
                    self.active[v]
                );
                assert_eq!(
                    tour[j].tour(),
                    i,
                    "edge {} = {:?}",
                    tour[j].index(),
                    self.ends(tour[j])
                );
                assert_eq!(tour[j].rank(), j);
            }
        }
        true
    }
}
