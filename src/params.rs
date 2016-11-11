use graph::*;
use std::borrow::BorrowMut;

macro_rules! define_param {
    ($S:ident($first:ident)) => (
        #[derive(Default)]
        pub struct $S<A>(pub A);

        impl<A> $S<A> {
            pub fn $first<N>(self, first: N) -> $S<N> {
                $S(first)
            }
        }
    );
    ($S:ident($first:ident, $second:ident)) => (
        #[derive(Default)]
        pub struct $S<A, B>(pub A, pub B);

        impl<A, B> $S<A, B> {
            pub fn $first<N>(self, first: N) -> $S<N, B> {
                $S(first, self.1)
            }

            pub fn $second<N>(self, second: N) -> $S<A, N> {
                $S(self.0, second)
            }
        }
    );
    ($S:ident($first:ident, $second:ident, $third:ident)) => (
        #[derive(Default)]
        pub struct $S<A, B, C>(pub A, pub B, pub C);

        impl<A, B, C> $S<A, B, C> {
            pub fn $first<N>(self, first: N) -> $S<N, B, C> {
                $S(first, self.1, self.2)
            }

            pub fn $second<N>(self, second: N) -> $S<A, N, C> {
                $S(self.0, second, self.2)
            }

            pub fn $third<N>(self, third: N) -> $S<A, B, N> {
                $S(self.0, self.1, third)
            }
        }
    );
}

pub trait Param<'a, Input, Target> {
    type Output: BorrowMut<Target>;

    fn build(self, _input: &'a Input) -> Self::Output;
}

impl<'a, Input, Target, P> Param<'a, Input, Target> for P
    where P: BorrowMut<Target>
{
    type Output = P;

    fn build(self, _input: &'a Input) -> Self::Output {
        self
    }
}


// VertexProp

pub trait ParamVertexProp<G: WithVertex, T> {
    type Target: VertexPropMut<G, T>;
    type Output: BorrowMut<Self::Target>;

    fn build(self, g: &G) -> Self::Output;
}

#[derive(Default)]
pub struct NewVertexProp<T>(pub T);

impl<G, T> ParamVertexProp<G, T> for NewVertexProp<T>
    where G: WithVertexProp<T>,
          T: Clone
{
    type Target = DefaultVertexPropMut<G, T>;
    type Output = DefaultVertexPropMut<G, T>;

    fn build(self, g: &G) -> Self::Output {
        g.vertex_prop(self.0)
    }
}

impl<'a, G, T, P> ParamVertexProp<G, T> for &'a mut P
    where G: WithVertex,
          P: VertexPropMut<G, T>
{
    type Target = P;
    type Output = &'a mut P;

    fn build(self, _g: &G) -> Self::Output {
        self
    }
}


// VertexIter

pub trait ParamVertexIter<'a, G: 'a + WithVertex> {
    type Output: Iterator<Item = Vertex<G>>;

    fn build(self, g: &'a G) -> Self::Output;
}

#[derive(Default)]
pub struct AllVertices;

impl<'a, G> ParamVertexIter<'a, G> for AllVertices
    where G: 'a + VertexList
{
    type Output = VertexIter<'a, G>;

    fn build(self, g: &'a G) -> Self::Output {
        g.vertices()
    }
}

impl<'a, G, P> ParamVertexIter<'a, G> for P
    where G: 'a + WithVertex,
          P: Iterator<Item = Vertex<G>>
{
    type Output = P;

    fn build(self, _g: &'a G) -> Self::Output {
        self
    }
}
