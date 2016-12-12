use graph::*;

use std::borrow::BorrowMut;

macro_rules! generic_struct {
    ($S:ident($zero:ident)) => (
        #[derive(Default)]
        pub struct $S<A>(pub A);

        impl<A> $S<A> {
            pub fn $zero<N>(self, zero: N) -> $S<N> {
                $S(zero)
            }
        }
    );
    ($S:ident($zero:ident, $one:ident)) => (
        #[derive(Default)]
        pub struct $S<A, B>(pub A, pub B);

        impl<A, B> $S<A, B> {
            pub fn $zero<N>(self, zero: N) -> $S<N, B> {
                $S(zero, self.1)
            }

            pub fn $one<N>(self, one: N) -> $S<A, N> {
                $S(self.0, one)
            }
        }
    );
    ($S:ident($zero:ident, $one:ident, $two:ident)) => (
        #[derive(Default)]
        pub struct $S<A, B, C>(pub A, pub B, pub C);

        impl<A, B, C> $S<A, B, C> {
            pub fn $zero<N>(self, zero: N) -> $S<N, B, C> {
                $S(zero, self.1, self.2)
            }

            pub fn $one<N>(self, one: N) -> $S<A, N, C> {
                $S(self.0, one, self.2)
            }

            pub fn $two<N>(self, two: N) -> $S<A, B, N> {
                $S(self.0, self.1, two)
            }
        }
    );
    ($S:ident($zero:ident, $one:ident, $two:ident, $three:ident)) => (
        #[derive(Default)]
        pub struct $S<A, B, C, D>(pub A, pub B, pub C, pub D);

        impl<A, B, C, D> $S<A, B, C, D> {
            pub fn $zero<N>(self, zero: N) -> $S<N, B, C, D> {
                $S(zero, self.1, self.2, self.3)
            }

            pub fn $one<N>(self, one: N) -> $S<A, N, C, D> {
                $S(self.0, one, self.2, self.3)
            }

            pub fn $two<N>(self, two: N) -> $S<A, B, N, D> {
                $S(self.0, self.1, two, self.3)
            }

            pub fn $three<N>(self, three: N) -> $S<A, B, C, N> {
                $S(self.0, self.1, self.2, three)
            }
        }
    );
    ($S:ident($zero:ident, $one:ident, $two:ident, $three:ident, $four:ident)) => (
        #[derive(Default)]
        pub struct $S<A, B, C, D, E>(pub A, pub B, pub C, pub D, pub E);

        impl<A, B, C, D, E> $S<A, B, C, D, E> {
            pub fn $zero<N>(self, zero: N) -> $S<N, B, C, D, E> {
                $S(zero, self.1, self.2, self.3, self.4)
            }

            pub fn $one<N>(self, one: N) -> $S<A, N, C, D, E> {
                $S(self.0, one, self.2, self.3, self.4)
            }

            pub fn $two<N>(self, two: N) -> $S<A, B, N, D, E> {
                $S(self.0, self.1, two, self.3, self.4)
            }

            pub fn $three<N>(self, three: N) -> $S<A, B, C, N, E> {
                $S(self.0, self.1, self.2, three, self.4)
            }

            pub fn $four<N>(self, four: N) -> $S<A, B, C, D, N> {
                $S(self.0, self.1, self.2, self.3, four)
            }
        }
    );
}

pub trait Param<'a, Input, Borrowed> {
    type Output: BorrowMut<Borrowed>;

    fn build(self, _input: &'a Input) -> Self::Output;
}

impl<'a, Input, Borrowed, P> Param<'a, Input, Borrowed> for P
    where P: BorrowMut<Borrowed>
{
    type Output = P;

    fn build(self, _input: &'a Input) -> Self::Output {
        self
    }
}


// Iterator

pub trait ParamIterator<'a, Input: 'a> {
    type Item;
    type Output: Iterator<Item = Self::Item>;

    fn build(self, input: &'a Input) -> Self::Output;
}

impl<'a, Input, I> ParamIterator<'a, Input> for I
    where Input: 'a,
          I: IntoIterator
{
    type Item = I::Item;
    type Output = I::IntoIter;

    fn build(self, _input: &'a Input) -> Self::Output {
        self.into_iter()
    }
}


#[derive(Default)]
pub struct AllVertices;

impl<'a, G> ParamIterator<'a, G> for AllVertices
    where G: 'a + VertexList
{
    type Item = Vertex<G>;
    type Output = VertexIter<'a, G>;

    fn build(self, g: &'a G) -> Self::Output {
        g.vertices()
    }
}


#[derive(Default)]
pub struct AllEdges;

impl<'a, G> ParamIterator<'a, G> for AllEdges
    where G: 'a + EdgeList
{
    type Item = Edge<G>;
    type Output = EdgeIter<'a, G>;

    fn build(self, g: &'a G) -> Self::Output {
        g.edges()
    }
}


// VertexProp

pub trait ParamVertexProp<G: WithVertex, T> {
    type Prop: VertexPropMut<G, T>;
    type Output: BorrowMut<Self::Prop>;

    fn build(self, g: &G) -> Self::Output;
}

#[derive(Default)]
pub struct NewVertexProp<T>(pub T);

impl<G, T> ParamVertexProp<G, T> for NewVertexProp<T>
    where G: WithVertexProp<T>,
          T: Clone
{
    type Prop = DefaultVertexPropMut<G, T>;
    type Output = DefaultVertexPropMut<G, T>;

    fn build(self, g: &G) -> Self::Output {
        g.vertex_prop(self.0)
    }
}

impl<'a, G, T, P> ParamVertexProp<G, T> for &'a mut P
    where G: WithVertex,
          P: VertexPropMut<G, T>
{
    type Prop = P;
    type Output = &'a mut P;

    fn build(self, _g: &G) -> Self::Output {
        self
    }
}
