use graph::*;

use rand::Rng;

pub trait Choose: Basic {
    fn choose_vertex<R>(&self, rng: &mut R) -> Vertex<Self>
        where R: Rng;

    fn choose_vertex_if<R, F>(&self, rng: &mut R, fun: &mut F) -> Vertex<Self>
        where R: Rng,
              F: FnMut(Vertex<Self>) -> bool
    {
        choose(|| Choose::choose_vertex(self, rng), fun)
    }

    fn choose_edge<R>(&self, rng: &mut R) -> Edge<Self>
        where R: Rng;

    fn choose_edge_if<R, F>(&self, rng: &mut R, fun: &mut F) -> Edge<Self>
        where R: Rng,
              F: FnMut(Edge<Self>) -> bool
    {
        choose(|| Choose::choose_edge(self, rng), fun)
    }

    fn choose_inc_edge<R: Rng>(&self, rng: &mut R, v: Vertex<Self>) -> Edge<Self>
        where R: Rng;

    fn choose_inc_edge_if<R, F>(&self, rng: &mut R, v: Vertex<Self>, fun: &mut F) -> Edge<Self>
        where R: Rng,
              F: FnMut(Edge<Self>) -> bool
    {
        choose(|| Choose::choose_inc_edge(self, rng, v), fun)
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
