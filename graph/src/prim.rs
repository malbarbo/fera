use params::*;
use prelude::*;
use props::Color;

use fera_fun::first;

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::marker::PhantomData;
use std::ops::DerefMut;

pub trait Prim: Incidence {
    fn prim<W, T>(&self,
                  w: W)
                  -> PrimAlg<&Self,
                             W,
                             NewVertexProp<Self, Color>,
                             NewVertexProp<Self, OptionEdge<Self>>,
                             Owned<PrimPriorityQueue<Self, T>>,
                             PhantomData<T>>
        where W: EdgePropGet<Self, T>,
              T: Ord
    {
        PrimAlg(self,
                w,
                NewVertexProp(self, Color::White),
                NewVertexProp(self, Self::edge_none()),
                Owned(PrimPriorityQueue::<Self, T>::new()),
                PhantomData)
    }
}

impl<G: Incidence> Prim for G {}


generic_struct! {
    #[must_use]
    pub struct PrimAlg(graph, weight, color, parent, queue, _marker)
}

impl<'a, G, W, C, P, Q, T> IntoIterator for PrimAlg<&'a G, W, C, P, Q, PhantomData<T>>
    where G: Incidence + VertexList,
          W: EdgePropGet<G, T>,
          C: ParamDerefMut,
          C::Target: VertexPropMut<G, Color>,
          P: ParamDerefMut,
          P::Target: VertexPropMut<G, OptionEdge<G>>,
          Q: ParamDerefMut<Target = PrimPriorityQueue<G, T>>,
          T: Ord + Default
{
    type Item = Edge<G>;
    type IntoIter = Iter<'a, G, W, C::Output, P::Output, Q::Output, T>;

    fn into_iter(self) -> Self::IntoIter {
        let PrimAlg(g, w, color, parent, queue, _) = self;
        let mut color = color.build();
        let mut queue = queue.build();
        let v = first(g.vertices());
        color[v] = Color::Gray;
        queue.push(QueueItem::new(T::default(), v));
        Iter {
            g,
            w,
            color,
            queue,
            parent: parent.build(),
            _marker: PhantomData,
        }
    }
}

pub struct Iter<'a, G, W, C, P, Q, T>
    where G: 'a
{
    g: &'a G,
    w: W,
    color: C,
    parent: P,
    queue: Q,
    _marker: PhantomData<T>,
}

impl<'a, G, W, C, P, Q, T> Iterator for Iter<'a, G, W, C, P, Q, T>
    where G: 'a + Incidence,
          W: EdgePropGet<G, T>,
          C: DerefMut,
          C::Target: VertexPropMut<G, Color>,
          P: DerefMut,
          P::Target: VertexPropMut<G, OptionEdge<G>>,
          Q: DerefMut<Target = PrimPriorityQueue<G, T>>,
          T: Ord
{
    type Item = Edge<G>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(QueueItem { vertex: u, .. }) = self.queue.pop() {
            if self.color[u] == Color::Black {
                continue;
            }
            self.color[u] = Color::Black;
            for e in self.g.out_edges(u) {
                let v = self.g.target(e);
                if self.color[v] == Color::Black {
                    continue;
                }
                if let Some(p) = self.parent[v].into_option() {
                    if self.w.get(p) < self.w.get(e) {
                        continue;
                    }
                }
                self.color[v] = Color::Gray;
                self.parent[v] = e.into();
                self.queue.push(QueueItem::new(self.w.get(e), v));
            }
            if let e @ Some(_) = self.parent[u].into_option() {
                return e;
            }
        }
        None
    }
}


type PrimPriorityQueue<G, T> = BinaryHeap<QueueItem<T, Vertex<G>>>;

pub struct QueueItem<A, B> {
    prio: A,
    vertex: B,
}

impl<A, B> QueueItem<A, B> {
    pub fn new(prio: A, vertex: B) -> Self {
        Self { prio, vertex }
    }
}

impl<A: PartialEq, B> PartialEq for QueueItem<A, B> {
    fn eq(&self, other: &Self) -> bool {
        self.prio == other.prio
    }
}

impl<A: Eq, B> Eq for QueueItem<A, B> {}

impl<A: PartialOrd, B> PartialOrd for QueueItem<A, B> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.prio.partial_cmp(&self.prio)
    }
}

impl<A: Ord, B> Ord for QueueItem<A, B> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.prio.cmp(&self.prio)
    }
}

#[cfg(test)]
mod tests {
    use super::Prim;
    use prelude::*;
    use fera_fun::vec;

    #[test]
    fn mst() {
        let g: StaticGraph = graph!(
            5,
            (0, 4), (2, 3), (0, 1), (1, 4), (1, 2), (2, 4), (3, 4)
            // expected tree
            // 0      1       2               4
        );
        let mut weight = g.default_edge_prop(0usize);
        for (e, &w) in g.edges().zip(&[1, 2, 3, 4, 5, 6, 7]) {
            weight[e] = w;
        }
        let e = vec(g.edges());
        let tree = vec(g.prim(&weight));
        assert_eq!(11usize, sum_prop(&weight, &tree));
        assert_eq!(vec![e[0], e[2], e[4], e[1]], tree);
    }
}
