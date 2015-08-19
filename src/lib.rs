#[macro_export]
macro_rules! set {
    () => {
        HashSet::new()
    };
    ($($x:expr),+) => {
        [$($x,)+].iter().map(|&x| x).collect::<HashSet<_>>()
    }
}

#[cfg(test)]
#[macro_use]
pub mod tests;

pub mod graph;
pub use graph::*;

pub mod static_;
pub use static_::*;

pub mod iter;
pub mod kruskal;
pub mod path;
pub mod traverse;
pub mod unionfind;
