use graph::*;

use rand::Rng;

// TODO: There is no vertex
// TODO: There is no edge
// TODO: There is no inc edge
pub trait Choose: Undirected {
    fn choose_vertex<R>(&self, rng: &mut R) -> Vertex<Self> where R: Rng;

    fn choose_vertex_if<R, F>(&self, rng: &mut R, fun: &mut F) -> Vertex<Self>
        where R: Rng,
              F: FnMut(Vertex<Self>) -> bool
    {
        choose(|| self.choose_vertex(rng), fun)
    }

    fn choose_edge<R>(&self, rng: &mut R) -> Edge<Self> where R: Rng;

    fn choose_edge_if<R, F>(&self, rng: &mut R, fun: &mut F) -> Edge<Self>
        where R: Rng,
              F: FnMut(Edge<Self>) -> bool
    {
        choose(|| self.choose_edge(rng), fun)
    }

    fn choose_inc_edge<R: Rng>(&self, rng: &mut R, v: Vertex<Self>) -> Edge<Self> where R: Rng;

    fn choose_inc_edge_if<R, F>(&self, rng: &mut R, v: Vertex<Self>, fun: &mut F) -> Edge<Self>
        where R: Rng,
              F: FnMut(Edge<Self>) -> bool
    {
        choose(|| self.choose_inc_edge(rng, v), fun)
    }

    fn choose_neighbor<R: Rng>(&self, rng: &mut R, v: Vertex<Self>) -> Vertex<Self> where R: Rng {
        self.target(self.choose_inc_edge(rng, v))
    }

    fn choose_neighbor_if<R, F>(&self, rng: &mut R, v: Vertex<Self>, fun: &mut F) -> Vertex<Self>
        where R: Rng,
              F: FnMut(Vertex<Self>) -> bool
    {
        let e = self.choose_inc_edge_if(rng, v, &mut |e| fun(self.target(e)));
        self.target(e)
    }
}

fn choose<Y: Copy, G: FnMut() -> Y, F: FnMut(Y) -> bool>(mut gen: G, fun: &mut F) -> Y {
    loop {
        // TODO: on static create an range and use ind_sample
        let e = gen();
        if fun(e) {
            return e;
        }
    }
}

// TODO: write tests
