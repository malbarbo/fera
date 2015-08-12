pub mod static_;
pub mod traverse;
pub mod unionfind;
pub mod kruskal;

// Basic

pub trait Basic {
    type Vertex: Copy;
    type Edge: Copy;
    type VertexIter: Iterator<Item=Self::Vertex>;
    type EdgeIter: Iterator<Item=Self::Edge>;

    fn num_vertices(&self) -> usize;
    fn vertices(&self) -> Self::VertexIter;

    fn num_edges(&self) -> usize;
    fn edges(&self) -> Self::EdgeIter;

    fn source(&self, e: Self::Edge) -> Self::Vertex;
    fn target(&self, e: Self::Edge) -> Self::Vertex;

    fn edge_vertices(&self, e: Self::Edge) -> (Self::Vertex, Self::Vertex) {
        (self.source(e), self.target(e))
    }

    // FIXME: make a lazy iterator
    fn edges_as_vertex_pairs(&self) -> Vec<(Self::Vertex, Self::Vertex)> {
        self.edges().map(|e| self.edge_vertices(e)).collect()
    }
}


// Degree

pub trait Degree: Basic {
    fn degree(&self, v: Self::Vertex) -> usize;
}


// Adj

pub trait AdjIterType<'a>: Basic {
    type Type: Iterator<Item=Self::Vertex>;
}

// FIXME: change definition when [E0122] is resolved
// pub type AdjIter<'a, G: Adj> = <G as AdjIterType<'a>>::Type;
pub type AdjIter<'a, G> = <G as AdjIterType<'a>>::Type;

pub trait Adj: Basic + for<'a> AdjIterType<'a> {
    fn neighbors(&self, v: Self::Vertex) -> AdjIter<Self>;
}


// Vertex Property

pub trait VertexPropType<'a, T>: Basic {
    type Type: std::ops::IndexMut<Self::Vertex, Output=T>;
}

// FIXME: change definition when [E0122] is resolved
// pub type VertexProp<'a, G: VertexPropType<'a, T>, T> = <G as VertexPropType<'a, T>>::Type;
pub type VertexProp<'a, G, T> = <G as VertexPropType<'a, T>>::Type;

pub trait WithVertexProp:
        for<'a> VertexPropType<'a, bool> +
        for<'a> VertexPropType<'a, char> +
        for<'a> VertexPropType<'a, i8> +
        for<'a> VertexPropType<'a, i16> +
        for<'a> VertexPropType<'a, i32> +
        for<'a> VertexPropType<'a, i64> +
        for<'a> VertexPropType<'a, isize> +
        for<'a> VertexPropType<'a, u8> +
        for<'a> VertexPropType<'a, u16> +
        for<'a> VertexPropType<'a, u32> +
        for<'a> VertexPropType<'a, u64> +
        for<'a> VertexPropType<'a, usize> +
        for<'a> VertexPropType<'a, f32> +
        for<'a> VertexPropType<'a, f64> +
        for<'a> VertexPropType<'a, &'a str> +
        for<'a> VertexPropType<'a, String> +
        for<'a> VertexPropType<'a, <Self as Basic>::Vertex> {
    fn vertex_prop<T: Clone>(&self, value: T) -> VertexProp<Self, T>;
}


// Edge Property

pub trait EdgePropType<'a, T>: Basic {
    type Type: std::ops::IndexMut<Self::Edge, Output=T>;
}

// FIXME: change definition when [E0122] is resolved
// pub type EdgeProp<'a, G: EdgePropType<'a, T>, T> = <G as EdgePropType<'a, T>>::Type;
pub type EdgeProp<'a, G, T> = <G as EdgePropType<'a, T>>::Type;

pub trait WithEdgeProp:
        for<'a> EdgePropType<'a, bool> +
        for<'a> EdgePropType<'a, char> +
        for<'a> EdgePropType<'a, i8> +
        for<'a> EdgePropType<'a, i16> +
        for<'a> EdgePropType<'a, i32> +
        for<'a> EdgePropType<'a, i64> +
        for<'a> EdgePropType<'a, isize> +
        for<'a> EdgePropType<'a, u8> +
        for<'a> EdgePropType<'a, u16> +
        for<'a> EdgePropType<'a, u32> +
        for<'a> EdgePropType<'a, u64> +
        for<'a> EdgePropType<'a, usize> +
        for<'a> EdgePropType<'a, f32> +
        for<'a> EdgePropType<'a, f64> +
        for<'a> EdgePropType<'a, &'a str> +
        for<'a> EdgePropType<'a, String> +
        for<'a> EdgePropType<'a, <Self as Basic>::Edge> {
    fn edge_prop<T: Clone>(&self, value: T) -> EdgeProp<Self, T>;
}


// GraphAdj

pub trait GraphAdj: Basic + Degree + Adj {
}

impl<G> GraphAdj for G where G: Basic + Degree + Adj {
}


// Tests

#[cfg(test)]
pub mod tests_ {
    use super::*;
    use std::fmt::Debug;
    use std::collections::HashSet;
    use std::hash::Hash;

    // Test graph (0, 1), (0, 2), (1, 2), (1, 3)

    pub fn assert_iter_eq<T, I1, I2>(a: I1, b: I2)
            where T: Debug + PartialEq, I1: IntoIterator<Item=T>, I2: IntoIterator<Item=T> {
        assert_eq!(a.into_iter().collect::<Vec<T>>(), b.into_iter().collect::<Vec<T>>());
    }

    pub fn assert_set_eq<T, I1, I2>(a: I1, b: I2)
            where T: Debug + Eq + Hash, I2: IntoIterator<Item=T>, I1: IntoIterator<Item=T> {
        assert_eq!(a.into_iter().collect::<HashSet<T>>(), b.into_iter().collect::<HashSet<T>>());
    }


    pub fn vertices<G>(g: &G) where G: Basic<Vertex=usize>, G::Edge: Debug {
        assert_eq!(5, g.num_vertices());
        assert_set_eq(vec![0, 1, 2, 3, 4], g.vertices());
    }

    pub fn edges<G>(g: &G) where G: Basic<Vertex=usize>, G::Edge: Debug {
        assert_eq!(4, g.num_edges());
        assert_set_eq(vec![(0, 1), (0, 2), (1, 2), (1, 3)], g.edges_as_vertex_pairs());
    }

    pub fn degree<G>(g: &G) where G: Degree<Vertex=usize>, G::Edge: Debug {
        assert_eq!(2, g.degree(0));
        assert_eq!(3, g.degree(1));
        assert_eq!(2, g.degree(2));
        assert_eq!(1, g.degree(3));
        assert_eq!(0, g.degree(4));
    }

    pub fn neighbors<G>(g: &G) where G: Adj<Vertex=usize>, G::Edge: Debug {
        assert_set_eq(vec![1, 2], g.neighbors(0));
        assert_set_eq(vec![0, 2, 3], g.neighbors(1));
        assert_set_eq(vec![0, 1], g.neighbors(2));
        assert_set_eq(vec![1], g.neighbors(3));
        assert_set_eq(vec![], g.neighbors(4));
    }

    pub fn vertex_prop<G>(g: &G) where G: WithVertexProp {
        let mut x = g.vertex_prop(0usize);
        let mut y = g.vertex_prop("a");
        let v = g.vertices().collect::<Vec<_>>();
        let (a, b, c, d, e) = (v[0], v[1], v[2], v[3], v[4]);
        x[c] = 8;
        y[d] = "b";
        assert_eq!(0, x[a]);
        assert_eq!(0, x[b]);
        assert_eq!(8, x[c]);
        assert_eq!(0, x[d]);
        assert_eq!(0, x[e]);
        assert_eq!("a", y[a]);
        assert_eq!("a", y[b]);
        assert_eq!("a", y[c]);
        assert_eq!("b", y[d]);
        assert_eq!("a", y[e]);
    }

    pub fn edge_prop<G>(g: &G) where G: WithEdgeProp {
        let mut x = g.edge_prop(0usize);
        let mut y = g.edge_prop("a");
        let edges = g.edges().collect::<Vec<_>>();
        let (a, b, c, d) = (edges[0], edges[1], edges[2], edges[3]);
        x[c] = 8;
        y[d] = "b";
        assert_eq!(0, x[a]);
        assert_eq!(0, x[b]);
        assert_eq!(8, x[c]);
        assert_eq!(0, x[d]);
        assert_eq!("a", y[a]);
        assert_eq!("a", y[b]);
        assert_eq!("a", y[c]);
        assert_eq!("b", y[d]);
    }
}
