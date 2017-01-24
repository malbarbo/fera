use prelude::*;

pub struct OutNeighborFromOutEdge<'a, G: 'a, I> {
    g: &'a G,
    iter: I,
}

impl<'a, G, I> OutNeighborFromOutEdge<'a, G, I>
    where I: Iterator<Item = Edge<G>>,
          G: 'a + WithEdge
{
    pub fn new(g: &'a G, iter: I) -> Self {
        OutNeighborFromOutEdge { g: g, iter: iter }
    }
}

impl<'a, G, I> Iterator for OutNeighborFromOutEdge<'a, G, I>
    where I: Iterator<Item = Edge<G>>,
          G: 'a + WithEdge
{
    type Item = Vertex<G>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|e| self.g.target(e))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, G, I> ExactSizeIterator for OutNeighborFromOutEdge<'a, G, I>
    where I: Iterator<Item = Edge<G>> + ExactSizeIterator,
          G: 'a + WithEdge
{
    #[inline]
    fn len(&self) -> usize {
        self.iter.len()
    }
}
