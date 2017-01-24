use prelude::*;

use rand::Rng;

// TODO: There is no vertex
// TODO: There is no edge
// TODO: There is no inc edge
// TODO: rename inc to out
// TODO: rename if to iter and return a iterator
pub trait Choose: WithEdge {
    fn choose_vertex<R>(&self, rng: &mut R) -> Vertex<Self> where R: Rng;

    fn choose_vertex_if<R, F>(&self, rng: &mut R, fun: F) -> Vertex<Self>
        where R: Rng,
              F: FnMut(Vertex<Self>) -> bool
    {
        choose(|| self.choose_vertex(rng), fun)
    }

    fn choose_edge<R>(&self, rng: &mut R) -> Edge<Self> where R: Rng;

    fn choose_edge_if<R, F>(&self, rng: &mut R, fun: F) -> Edge<Self>
        where R: Rng,
              F: FnMut(Edge<Self>) -> bool
    {
        choose(|| self.choose_edge(rng), fun)
    }

    fn choose_inc_edge<R>(&self, rng: &mut R, v: Vertex<Self>) -> Edge<Self> where R: Rng;

    fn choose_inc_edge_if<R, F>(&self, rng: &mut R, v: Vertex<Self>, fun: F) -> Edge<Self>
        where R: Rng,
              F: FnMut(Edge<Self>) -> bool
    {
        choose(|| self.choose_inc_edge(rng, v), fun)
    }

    fn choose_neighbor<R>(&self, rng: &mut R, v: Vertex<Self>) -> Vertex<Self>
        where R: Rng
    {
        self.target(self.choose_inc_edge(rng, v))
    }

    fn choose_neighbor_if<R, F>(&self, rng: &mut R, v: Vertex<Self>, mut fun: F) -> Vertex<Self>
        where R: Rng,
              F: FnMut(Vertex<Self>) -> bool
    {
        let e = self.choose_inc_edge_if(rng, v, |e| fun(self.target(e)));
        self.target(e)
    }

    fn random_walk<R>(&self, mut rng: R) -> RandomWalk<Self, R>
        where R: Rng
    {
        let cur = self.choose_vertex(&mut rng);
        RandomWalk {
            g: self,
            cur: cur,
            rng: rng,
        }
    }
}

fn choose<Y: Copy, G: FnMut() -> Y, F: FnMut(Y) -> bool>(mut gen: G, mut fun: F) -> Y {
    loop {
        // TODO: for StaticGraph create a range and use ind_sample
        let e = gen();
        if fun(e) {
            return e;
        }
    }
}

pub struct RandomWalk<'a, G: 'a + WithVertex, R> {
    g: &'a G,
    cur: Vertex<G>,
    rng: R,
}

impl<'a, G, R> Iterator for RandomWalk<'a, G, R>
    where G: 'a + Choose,
          R: Rng
{
    type Item = Edge<G>;

    fn next(&mut self) -> Option<Self::Item> {
        let e = self.g.choose_inc_edge(&mut self.rng, self.cur);
        self.cur = self.g.target(e);
        Some(e)
    }
}

// TODO: write tests
