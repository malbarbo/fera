//! Random selection of vertices and edges.

use prelude::*;

use rand::Rng;

// TODO: specialization of *_iter
// TODO: add bounds to methods
pub trait Choose: WithEdge {
    fn choose_vertex<R: Rng>(&self, rng: R) -> Option<Vertex<Self>>;

    fn choose_vertex_iter<R: Rng>(&self, rng: R) -> ChooseVertexIter<Self, R> {
        ChooseVertexIter {
            g: self,
            rng: rng,
        }
    }


    fn choose_out_neighbor<R: Rng>(&self, v: Vertex<Self>, rng: R) -> Option<Vertex<Self>>;

    fn choose_out_neighbor_iter<R: Rng>(&self,
                                        v: Vertex<Self>,
                                        rng: R)
                                        -> ChooseOutNeighborIter<Self, R> {
        ChooseOutNeighborIter {
            g: self,
            v: v,
            rng: rng,
        }
    }


    fn choose_edge<R: Rng>(&self, rng: R) -> Option<Edge<Self>>;

    fn choose_edge_iter<R: Rng>(&self, rng: R) -> ChooseEdgeIter<Self, R> {
        ChooseEdgeIter {
            g: self,
            rng: rng,
        }
    }


    fn choose_out_edge<R: Rng>(&self, v: Vertex<Self>, rng: R) -> Option<Edge<Self>>;

    fn choose_out_edge_iter<R: Rng>(&self, v: Vertex<Self>, rng: R) -> ChooseOutEdgeIter<Self, R> {
        ChooseOutEdgeIter {
            g: self,
            v: v,
            rng: rng,
        }
    }


    fn random_walk<R: Rng>(&self, mut rng: R) -> RandomWalk<Self, R> {
        let cur = self.choose_vertex(&mut rng);
        RandomWalk {
            g: self,
            cur: cur,
            rng: rng,
        }
    }
}


pub struct ChooseVertexIter<'a, G: 'a, R> {
    g: &'a G,
    rng: R,
}

impl<'a, G, R> Iterator for ChooseVertexIter<'a, G, R>
    where G: 'a + Choose,
          R: Rng
{
    type Item = Vertex<G>;

    fn next(&mut self) -> Option<Vertex<G>> {
        G::choose_vertex(self.g, &mut self.rng)
    }
}


pub struct ChooseOutNeighborIter<'a, G: 'a + WithVertex, R> {
    g: &'a G,
    v: Vertex<G>,
    rng: R,
}

impl<'a, G, R> Iterator for ChooseOutNeighborIter<'a, G, R>
    where G: 'a + Choose,
          R: Rng
{
    type Item = Vertex<G>;

    fn next(&mut self) -> Option<Vertex<G>> {
        G::choose_out_neighbor(self.g, self.v, &mut self.rng)
    }
}


pub struct ChooseEdgeIter<'a, G: 'a, R> {
    g: &'a G,
    rng: R,
}

impl<'a, G, R> Iterator for ChooseEdgeIter<'a, G, R>
    where G: 'a + Choose,
          R: Rng
{
    type Item = Edge<G>;

    fn next(&mut self) -> Option<Edge<G>> {
        G::choose_edge(self.g, &mut self.rng)
    }
}


pub struct ChooseOutEdgeIter<'a, G: 'a + WithVertex, R> {
    g: &'a G,
    v: Vertex<G>,
    rng: R,
}

impl<'a, G, R> Iterator for ChooseOutEdgeIter<'a, G, R>
    where G: 'a + Choose,
          R: Rng
{
    type Item = Edge<G>;

    fn next(&mut self) -> Option<Edge<G>> {
        G::choose_out_edge(self.g, self.v, &mut self.rng)
    }
}


pub struct RandomWalk<'a, G: 'a + WithVertex, R> {
    g: &'a G,
    cur: Option<Vertex<G>>,
    rng: R,
}

impl<'a, G, R> Iterator for RandomWalk<'a, G, R>
    where G: 'a + Choose,
          R: Rng
{
    type Item = Edge<G>;

    fn next(&mut self) -> Option<Self::Item> {
        self.cur.and_then(|cur| if let Some(e) = self.g.choose_out_edge(cur, &mut self.rng) {
            self.cur = Some(self.g.target(e));
            Some(e)
        } else {
            self.cur = None;
            None
        })
    }
}

// TODO: write tests
