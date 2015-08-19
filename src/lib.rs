#[macro_export]
macro_rules! set {
    () => {
        HashSet::new()
    };
    ($($x:expr),+) => {
        [$($x,)+].iter().map(|&x| x).collect::<HashSet<_>>()
    }
}

// https://stackoverflow.com/questions/30291584/macro-for-defining-trait-aliases

#[macro_export]
macro_rules! items {
    ($($item:item)*) => ($($item)*);
}

#[macro_export]
macro_rules! trait_alias {
    ($name:ident : $($base:tt)+) => {
        items! {
            pub trait $name: $($base)+ { }
            impl<T: $($base)+> $name for T { }
        }
    };
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
