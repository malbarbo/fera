pub mod graphadj;

// Basic

pub trait Basic<'a> {
    type Vertex: Copy;
    type Edge: Copy;
    type VertexIter: Iterator<Item=Self::Vertex>;
    type EdgeIter: Iterator<Item=Self::Edge>;

    fn num_vertices(&'a self) -> usize;
    fn vertices(&'a self) -> Self::VertexIter;

    fn num_edges(&'a self) -> usize;
    fn edges(&'a self) -> Self::EdgeIter;

    fn source(&'a self, e: Self::Edge) -> Self::Vertex;
    fn target(&'a self, e: Self::Edge) -> Self::Vertex;

    fn edge_vertices(&'a self, e: Self::Edge) -> (Self::Vertex, Self::Vertex) {
        (self.source(e), self.target(e))
    }

    // FIXME: make a lazy iterator
    fn edges_as_vertex_pairs(&'a self) -> Vec<(Self::Vertex, Self::Vertex)> {
        fn f<'a, T: ?Sized + Basic<'a>>(x: &'a T) -> Vec<(T::Vertex, T::Vertex)> {
            x.edges().map(|e| x.edge_vertices(e)).collect()
        }
        f(self)
    }
}


// Degree

pub trait Degree<'a>: Basic<'a> {
    fn degree(&'a self, v: Self::Vertex) -> usize;
}


// Adj

pub trait Adj<'a>: Basic<'a> {
    type NeighborsIter: Iterator<Item=Self::Vertex>;
    fn neighbors(&'a self, v: Self::Vertex) -> Self::NeighborsIter;
}


// Vertex Property

pub trait VertexPropType<'a, T>: Basic<'a> {
    type Type: std::ops::IndexMut<Self::Vertex, Output=T>;
}

pub type VertexProp<'a, G, T> = <G as VertexPropType<'a, T>>::Type;

pub trait WithVertexProp {
    fn vertex_prop<T: Clone>(&self, value: T) -> <Self as VertexPropType<T>>::Type;
}


// Edge Property

pub trait EdgePropType<'a, T>: Basic<'a> {
    type Type: std::ops::IndexMut<Self::Edge, Output=T>;
}

pub type EdgeProp<'a, G, T> = <G as EdgePropType<'a, T>>::Type;

pub trait WithEdgeProp {
    fn edge_prop<T: Clone>(&self, value: T) -> <Self as EdgePropType<T>>::Type;
}


// GraphAdj

pub trait GraphAdj<'a>: Basic<'a> + Degree<'a> + Adj<'a> {
}

impl<'a, G> GraphAdj<'a> for G where G: Basic<'a> + Degree<'a> + Adj<'a> {
}



#[cfg(test)]
pub mod tests {
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


    pub fn vertices<'a, G>(g: &'a G) where G: Basic<'a, Vertex=usize>, G::Edge: Debug {
        assert_eq!(5, g.num_vertices());
        assert_set_eq(vec![0, 1, 2, 3, 4], g.vertices());
    }

    pub fn edges<'a, G>(g: &'a G) where G: Basic<'a, Vertex=usize>, G::Edge: Debug {
        assert_eq!(4, g.num_edges());
        assert_set_eq(vec![(0, 1), (0, 2), (1, 2), (1, 3)], g.edges_as_vertex_pairs());
    }

    pub fn degree<'a, G>(g: &'a G) where G: Degree<'a, Vertex=usize>, G::Edge: Debug {
        assert_eq!(2, g.degree(0));
        assert_eq!(3, g.degree(1));
        assert_eq!(2, g.degree(2));
        assert_eq!(1, g.degree(3));
        assert_eq!(0, g.degree(4));
    }

    pub fn neighbors<'a, G>(g: &'a G) where G: Adj<'a, Vertex=usize>, G::Edge: Debug {
        assert_set_eq(vec![1, 2], g.neighbors(0));
        assert_set_eq(vec![0, 2, 3], g.neighbors(1));
        assert_set_eq(vec![0, 1], g.neighbors(2));
        assert_set_eq(vec![1], g.neighbors(3));
        assert_set_eq(vec![], g.neighbors(4));
    }

    pub fn vertex_prop<'a, G>(g: &'a G) where G: WithVertexProp + VertexPropType<'a, usize> + VertexPropType<'a, String> {
        let mut x = g.vertex_prop(0);
        let mut y = g.vertex_prop("a".to_string());
        let v = g.vertices().collect::<Vec<_>>();
        let (a, b, c, d, e) = (v[0], v[1], v[2], v[3], v[4]);
        x[c] = 8;
        y[d] = "b".to_string();
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

    pub fn edge_prop<'a, G>(g: &'a G) where G: WithEdgeProp + EdgePropType<'a, usize> + EdgePropType<'a, String> {
        let mut x = g.edge_prop(0);
        let mut y = g.edge_prop("a".to_string());
        let edges = g.edges().collect::<Vec<_>>();
        let (a, b, c, d) = (edges[0], edges[1], edges[2], edges[3]);
        x[c] = 8;
        y[d] = "b".to_string();
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
