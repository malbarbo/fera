//! Support for generic algorithms parameters.

use prelude::*;

use std::ops::{Deref, DerefMut};

macro_rules! generic_struct {
    ($(#[$attr:meta])* pub struct $S:ident($zero:ident)) => (
        $(#[$attr])*
        pub struct $S<A>(pub A);

        impl<A> $S<A> {
            pub fn $zero<N>(self, zero: N) -> $S<N> {
                $S(zero)
            }
        }
    );
    ($(#[$attr:meta])* pub struct $S:ident($zero:ident, $one:ident)) => (
        $(#[$attr])*
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
    ($(#[$attr:meta])* pub struct $S:ident($zero:ident, $one:ident, $two:ident)) => (
        $(#[$attr])*
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
    ($(#[$attr:meta])* pub struct $S:ident($zero:ident, $one:ident, $two:ident,
                                           $three:ident)) => (
        $(#[$attr])*
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
    ($(#[$attr:meta])* pub struct $S:ident($zero:ident, $one:ident, $two:ident,
                                           $three:ident, $four:ident)) => (
        $(#[$attr])*
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
    ($(#[$attr:meta])* pub struct $S:ident($zero:ident, $one:ident, $two:ident,
                                           $three:ident, $four:ident, $five:ident)) => (
        $(#[$attr])*
        pub struct $S<A, B, C, D, E, F>(pub A, pub B, pub C, pub D, pub E, pub F);

        impl<A, B, C, D, E, F> $S<A, B, C, D, E, F> {
            pub fn $zero<N>(self, zero: N) -> $S<N, B, C, D, E, F> {
                $S(zero, self.1, self.2, self.3, self.4, self.5)
            }

            pub fn $one<N>(self, one: N) -> $S<A, N, C, D, E, F> {
                $S(self.0, one, self.2, self.3, self.4, self.5)
            }

            pub fn $two<N>(self, two: N) -> $S<A, B, N, D, E, F> {
                $S(self.0, self.1, two, self.3, self.4, self.5)
            }

            pub fn $three<N>(self, three: N) -> $S<A, B, C, N, E, F> {
                $S(self.0, self.1, self.2, three, self.4, self.5)
            }

            pub fn $four<N>(self, four: N) -> $S<A, B, C, D, N, F> {
                $S(self.0, self.1, self.2, self.3, four, self.5)
            }

            pub fn $five<N>(self, five: N) -> $S<A, B, C, D, E, N> {
                $S(self.0, self.1, self.2, self.3, self.4, five)
            }
        }
    );
}


pub trait ParamDerefMut {
    type Target;
    type Output: DerefMut<Target = Self::Target>;

    fn build(self) -> Self::Output;
}

impl<'a, T> ParamDerefMut for &'a mut T {
    type Target = T;
    type Output = &'a mut T;

    fn build(self) -> Self::Output {
        self
    }
}


// TODO: Create an IntoIteratorOwned
pub trait IntoOwned<Owned> {
    fn into_owned(self) -> Owned;
}

impl<T> IntoOwned<T> for T {
    #[inline]
    fn into_owned(self) -> T {
        self
    }
}

impl<'a, T: Clone> IntoOwned<T> for &'a T {
    #[inline]
    fn into_owned(self) -> T {
        T::clone(self)
    }
}

impl<'a, T: Clone> IntoOwned<T> for &'a mut T {
    #[inline]
    fn into_owned(self) -> T {
        T::clone(self)
    }
}


pub struct Owned<T>(pub T);

impl<T> Deref for Owned<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Owned<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> ParamDerefMut for Owned<T> {
    type Target = T;
    type Output = Self;

    fn build(self) -> Self::Output {
        self
    }
}


pub struct NewVertexProp<'a, G: 'a, T>(pub &'a G, pub T);

impl<'a, G, T> ParamDerefMut for NewVertexProp<'a, G, T>
    where G: 'a + WithVertexProp<T>,
          T: Clone
{
    type Target = DefaultVertexPropMut<G, T>;
    type Output = Owned<DefaultVertexPropMut<G, T>>;

    fn build(self) -> Self::Output {
        Owned(self.0.vertex_prop(self.1))
    }
}

// TODO: create NewEdgeProp


// Iterator

// TODO: find a better prefix than All

pub struct AllVertices<'a, G: 'a>(pub &'a G);

impl<'a, G> IntoIterator for AllVertices<'a, G>
    where G: 'a + VertexList
{
    type Item = Vertex<G>;
    type IntoIter = VertexIter<'a, G>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.vertices()
    }
}


pub struct AllEdges<'a, G: 'a>(pub &'a G);

impl<'a, G> IntoIterator for AllEdges<'a, G>
    where G: 'a + EdgeList
{
    type Item = Edge<G>;
    type IntoIter = EdgeIter<'a, G>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.edges()
    }
}


// TODO: create AllOutEdges
