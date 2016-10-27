use graph::*;

pub struct IncidenceOutNeighborIter<I, G> {
    iter: I,
    g: *const G,
}

impl<I, G> IncidenceOutNeighborIter<I, G>
    where I: Iterator<Item = Edge<G>>,
          G: WithEdge
{
    pub fn new(iter: I, g: &G) -> Self {
        IncidenceOutNeighborIter {
            iter: iter,
            g: g as *const _,
        }
    }
}


impl<I, G> Iterator for IncidenceOutNeighborIter<I, G>
    where I: Iterator<Item = Edge<G>>,
          G: WithEdge
{
    type Item = Vertex<G>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|e| unsafe { (&*self.g).target(e) })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<I, G> ExactSizeIterator for IncidenceOutNeighborIter<I, G>
    where I: Iterator<Item = Edge<G>> + ExactSizeIterator,
          G: WithEdge
{
}
